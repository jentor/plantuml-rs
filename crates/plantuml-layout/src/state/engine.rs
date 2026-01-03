//! State Diagram Layout Engine
//!
//! Алгоритм layout для диаграмм состояний.
//! Использует послойное расположение с учётом переходов.

use indexmap::{IndexMap, IndexSet};
use plantuml_ast::state::{StateDiagram, StateType, Transition};
use plantuml_model::{Point, Rect};

use super::config::StateLayoutConfig;
use crate::{EdgeType, ElementType, LayoutElement, LayoutResult};

/// Layout engine для state diagrams
pub struct StateLayoutEngine {
    config: StateLayoutConfig,
}

impl StateLayoutEngine {
    /// Создаёт новый engine с конфигурацией по умолчанию
    pub fn new() -> Self {
        Self {
            config: StateLayoutConfig::default(),
        }
    }

    /// Создаёт engine с заданной конфигурацией
    pub fn with_config(config: StateLayoutConfig) -> Self {
        Self { config }
    }

    /// Выполняет layout диаграммы
    pub fn layout(&self, diagram: &StateDiagram) -> LayoutResult {
        let mut elements = Vec::new();
        let mut state_positions: IndexMap<String, Rect> = IndexMap::new();

        // Собираем все состояния (включая [*]) с сохранением порядка
        let mut all_states: IndexSet<String> = IndexSet::new();
        for state in &diagram.states {
            all_states.insert(state.name.clone());
            if let Some(alias) = &state.alias {
                all_states.insert(alias.clone());
            }
        }
        for trans in &diagram.transitions {
            all_states.insert(trans.from.clone());
            all_states.insert(trans.to.clone());
        }

        // Определяем уровни состояний (простой топологический порядок)
        let levels = self.assign_levels(diagram, &all_states);
        
        // Группируем состояния по уровням (используем IndexMap для стабильного порядка)
        let mut level_states: IndexMap<usize, Vec<String>> = IndexMap::new();
        for (state, level) in &levels {
            level_states
                .entry(*level)
                .or_insert_with(Vec::new)
                .push(state.clone());
        }

        // Располагаем состояния по уровням
        let max_level = levels.values().max().copied().unwrap_or(0);
        
        for level in 0..=max_level {
            if let Some(states) = level_states.get(&level) {
                let y = self.config.margin 
                    + level as f64 * (self.config.state_min_height + self.config.vertical_spacing);
                
                let total_width = states.len() as f64 * self.config.state_width 
                    + (states.len() as f64 - 1.0) * self.config.horizontal_spacing;
                let start_x = self.config.margin + (500.0 - total_width) / 2.0; // Центрируем

                for (i, state_name) in states.iter().enumerate() {
                    let x = start_x + i as f64 * (self.config.state_width + self.config.horizontal_spacing);
                    
                    // Определяем тип состояния
                    let state_type = self.get_state_type(diagram, state_name);
                    
                    let (elem, bounds) = self.create_state_element(state_name, state_type, x, y);
                    state_positions.insert(state_name.clone(), bounds);
                    elements.push(elem);
                }
            }
        }

        // Создаём переходы
        for trans in &diagram.transitions {
            if let (Some(from_rect), Some(to_rect)) = 
                (state_positions.get(&trans.from), state_positions.get(&trans.to)) 
            {
                let edge = self.create_transition_element(trans, from_rect, to_rect);
                elements.push(edge);
            }
        }

        // Вычисляем bounds
        let mut result = LayoutResult {
            elements,
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
        };
        result.calculate_bounds();
        
        // Добавляем отступы
        result.bounds.width += self.config.margin * 2.0;
        result.bounds.height += self.config.margin * 2.0;

        result
    }

    /// Назначает уровни состояниям (с сохранением порядка)
    fn assign_levels(&self, diagram: &StateDiagram, all_states: &IndexSet<String>) -> IndexMap<String, usize> {
        let mut levels: IndexMap<String, usize> = IndexMap::new();
        
        // Находим начальные состояния ([*] как источник)
        for state in all_states {
            if state == "[*]" {
                let is_source = diagram.transitions.iter().any(|t| t.from == *state);
                if is_source {
                    levels.insert(state.clone(), 0);
                }
            }
        }

        // Если нет начального [*], находим состояния без входящих переходов
        if levels.is_empty() {
            let targets: IndexSet<&String> = diagram.transitions.iter().map(|t| &t.to).collect();
            for state in all_states {
                if !targets.contains(state) {
                    levels.insert(state.clone(), 0);
                }
            }
        }

        // Если всё ещё пусто, берём первое состояние
        if levels.is_empty() {
            if let Some(first) = all_states.iter().next() {
                levels.insert(first.clone(), 0);
            }
        }

        // BFS для назначения уровней (фиксированное количество итераций)
        let max_iterations = all_states.len() + 1;
        for _ in 0..max_iterations {
            let mut new_levels = levels.clone();
            
            for trans in &diagram.transitions {
                if let Some(&fl) = levels.get(&trans.from) {
                    let new_level = fl + 1;
                    
                    let current = new_levels.get(&trans.to).copied();
                    // Назначаем уровень только если ещё не назначен
                    if current.is_none() {
                        new_levels.insert(trans.to.clone(), new_level);
                    }
                }
            }
            
            if new_levels.len() == levels.len() {
                // Нет новых состояний
                break;
            }
            levels = new_levels;
        }

        // Устанавливаем уровень 0 для оставшихся состояний
        for state in all_states {
            if !levels.contains_key(state) {
                levels.insert(state.clone(), 0);
            }
        }

        levels
    }

    /// Получает тип состояния
    fn get_state_type(&self, diagram: &StateDiagram, name: &str) -> StateType {
        if name == "[*]" {
            // Определяем: это начальное или конечное состояние
            let is_source = diagram.transitions.iter().any(|t| t.from == name);
            let is_target = diagram.transitions.iter().any(|t| t.to == name);
            
            if is_source && !is_target {
                return StateType::Initial;
            } else if is_target && !is_source {
                return StateType::Final;
            }
            return StateType::Initial; // По умолчанию
        }

        if name == "[H]" {
            return StateType::History;
        }
        if name == "[H*]" {
            return StateType::DeepHistory;
        }

        // Ищем в определениях состояний
        for state in &diagram.states {
            if state.name == name || state.alias.as_deref() == Some(name) {
                return state.state_type;
            }
        }

        StateType::Simple
    }

    /// Создаёт элемент состояния
    fn create_state_element(
        &self,
        name: &str,
        state_type: StateType,
        x: f64,
        y: f64,
    ) -> (LayoutElement, Rect) {
        match state_type {
            StateType::Initial => self.create_initial_state(name, x, y),
            StateType::Final => self.create_final_state(name, x, y),
            StateType::Choice => self.create_choice_state(name, x, y),
            StateType::Fork | StateType::Join => self.create_fork_join_state(name, x, y),
            StateType::History => self.create_history_state(name, x, y, false),
            StateType::DeepHistory => self.create_history_state(name, x, y, true),
            StateType::Composite => self.create_composite_state(name, x, y),
            _ => self.create_simple_state(name, x, y),
        }
    }

    /// Создаёт начальное состояние (заполненный круг)
    fn create_initial_state(&self, name: &str, x: f64, y: f64) -> (LayoutElement, Rect) {
        let r = self.config.node_radius;
        let cx = x + self.config.state_width / 2.0;
        let cy = y + r;
        
        let bounds = Rect::new(cx - r, cy - r, r * 2.0, r * 2.0);
        
        (LayoutElement {
            id: format!("initial_{}", name.replace(['[', ']', '*'], "")),
            bounds: bounds.clone(),
            text: None, properties: std::collections::HashMap::new(), element_type: ElementType::Ellipse { label: None },
        }, bounds)
    }

    /// Создаёт конечное состояние (круг с внутренним кругом)
    fn create_final_state(&self, name: &str, x: f64, y: f64) -> (LayoutElement, Rect) {
        let r = self.config.node_radius;
        let cx = x + self.config.state_width / 2.0;
        let cy = y + r;
        
        let bounds = Rect::new(cx - r, cy - r, r * 2.0, r * 2.0);
        
        (LayoutElement {
            id: format!("final_{}", name.replace(['[', ']', '*'], "")),
            bounds: bounds.clone(),
            text: None, properties: std::collections::HashMap::new(), element_type: ElementType::Ellipse { 
                label: Some("●".to_string()) // Внутренний круг
            },
        }, bounds)
    }

    /// Создаёт простое состояние
    fn create_simple_state(&self, name: &str, x: f64, y: f64) -> (LayoutElement, Rect) {
        let bounds = Rect::new(x, y, self.config.state_width, self.config.state_min_height);
        
        (LayoutElement {
            id: format!("state_{}", name),
            bounds: bounds.clone(),
            text: None, properties: std::collections::HashMap::new(), element_type: ElementType::Rectangle {
                label: name.to_string(),
                corner_radius: self.config.state_corner_radius,
            },
        }, bounds)
    }

    /// Создаёт составное состояние
    fn create_composite_state(&self, name: &str, x: f64, y: f64) -> (LayoutElement, Rect) {
        // Составное состояние выше обычного
        let height = self.config.state_min_height * 2.0;
        let bounds = Rect::new(x, y, self.config.state_width * 1.5, height);
        
        (LayoutElement {
            id: format!("composite_{}", name),
            bounds: bounds.clone(),
            text: None, properties: std::collections::HashMap::new(), element_type: ElementType::Rectangle {
                label: name.to_string(),
                corner_radius: self.config.state_corner_radius,
            },
        }, bounds)
    }

    /// Создаёт choice state (ромб)
    fn create_choice_state(&self, name: &str, x: f64, y: f64) -> (LayoutElement, Rect) {
        let size = self.config.choice_size;
        let cx = x + self.config.state_width / 2.0;
        let cy = y + size / 2.0;
        
        let bounds = Rect::new(cx - size / 2.0, cy - size / 2.0, size, size);
        
        (LayoutElement {
            id: format!("choice_{}", name),
            bounds: bounds.clone(),
            text: None, properties: std::collections::HashMap::new(), element_type: ElementType::Text {
                text: "◇".to_string(), // Ромб
                font_size: 16.0,
            },
        }, bounds)
    }

    /// Создаёт fork/join bar
    fn create_fork_join_state(&self, name: &str, x: f64, y: f64) -> (LayoutElement, Rect) {
        let cx = x + self.config.state_width / 2.0;
        let bounds = Rect::new(
            cx - self.config.bar_width / 2.0,
            y,
            self.config.bar_width,
            self.config.bar_height,
        );
        
        (LayoutElement {
            id: format!("bar_{}", name),
            bounds: bounds.clone(),
            text: None, properties: std::collections::HashMap::new(), element_type: ElementType::Rectangle {
                label: String::new(),
                corner_radius: 0.0,
            },
        }, bounds)
    }

    /// Создаёт history state
    fn create_history_state(&self, name: &str, x: f64, y: f64, deep: bool) -> (LayoutElement, Rect) {
        let r = self.config.node_radius * 0.8;
        let cx = x + self.config.state_width / 2.0;
        let cy = y + r;
        
        let bounds = Rect::new(cx - r, cy - r, r * 2.0, r * 2.0);
        
        let label = if deep { "H*" } else { "H" };
        
        (LayoutElement {
            id: format!("history_{}", name.replace(['[', ']', '*'], "")),
            bounds: bounds.clone(),
            text: None, properties: std::collections::HashMap::new(), element_type: ElementType::Ellipse { 
                label: Some(label.to_string())
            },
        }, bounds)
    }

    /// Создаёт элемент перехода
    fn create_transition_element(
        &self,
        trans: &Transition,
        from_rect: &Rect,
        to_rect: &Rect,
    ) -> LayoutElement {
        // Вычисляем точки соединения
        let (start, end) = self.calculate_connection_points(from_rect, to_rect);
        
        // Формируем метку
        let label = trans.label();
        let label_opt = if label.is_empty() { None } else { Some(label) };

        let min_x = start.x.min(end.x);
        let min_y = start.y.min(end.y);
        let max_x = start.x.max(end.x);
        let max_y = start.y.max(end.y);

        LayoutElement {
            id: format!("trans_{}_{}", trans.from.replace(['[', ']', '*'], ""), 
                       trans.to.replace(['[', ']', '*'], "")),
            bounds: Rect::new(min_x, min_y, (max_x - min_x).max(1.0), (max_y - min_y).max(1.0)),
            text: None, properties: std::collections::HashMap::new(), element_type: ElementType::Edge {
                points: vec![start, end],
                label: label_opt,
                arrow_start: false,
                arrow_end: true,
                dashed: false,
                edge_type: EdgeType::Association,
            },
        }
    }

    /// Вычисляет точки соединения для перехода
    fn calculate_connection_points(&self, from: &Rect, to: &Rect) -> (Point, Point) {
        let from_center_x = from.x + from.width / 2.0;
        let from_center_y = from.y + from.height / 2.0;
        let to_center_x = to.x + to.width / 2.0;
        let to_center_y = to.y + to.height / 2.0;

        // Определяем направление
        let dx = to_center_x - from_center_x;
        let dy = to_center_y - from_center_y;

        let start;
        let end;

        if dy.abs() > dx.abs() {
            // Вертикальное соединение
            if dy > 0.0 {
                // Вниз
                start = Point::new(from_center_x, from.y + from.height);
                end = Point::new(to_center_x, to.y);
            } else {
                // Вверх
                start = Point::new(from_center_x, from.y);
                end = Point::new(to_center_x, to.y + to.height);
            }
        } else {
            // Горизонтальное соединение
            if dx > 0.0 {
                // Вправо
                start = Point::new(from.x + from.width, from_center_y);
                end = Point::new(to.x, to_center_y);
            } else {
                // Влево
                start = Point::new(from.x, from_center_y);
                end = Point::new(to.x + to.width, to_center_y);
            }
        }

        (start, end)
    }
}

impl Default for StateLayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plantuml_ast::state::State;

    #[test]
    fn test_layout_simple_state_machine() {
        let mut diagram = StateDiagram::new();
        diagram.add_transition(Transition::new("[*]", "Active"));
        diagram.add_transition(Transition::new("Active", "Inactive").with_event("timeout"));
        diagram.add_transition(Transition::new("Inactive", "[*]"));

        let engine = StateLayoutEngine::new();
        let result = engine.layout(&diagram);

        // Должны быть: initial, Active, Inactive, final + 3 перехода
        assert!(result.elements.len() >= 6);
    }

    #[test]
    fn test_layout_with_choice() {
        let mut diagram = StateDiagram::new();
        diagram.add_state(State {
            name: "choice1".to_string(),
            alias: None,
            description: None,
            stereotype: None,
            state_type: StateType::Choice,
            substates: Vec::new(),
            internal_transitions: Vec::new(),
            regions: Vec::new(),
            color: None,
            entry_action: None,
            exit_action: None,
            do_action: None,
        });
        
        diagram.add_transition(Transition::new("[*]", "choice1"));
        diagram.add_transition(Transition::new("choice1", "State1"));
        diagram.add_transition(Transition::new("choice1", "State2"));

        let engine = StateLayoutEngine::new();
        let result = engine.layout(&diagram);

        assert!(!result.elements.is_empty());
    }

    #[test]
    fn test_layout_with_fork_join() {
        let mut diagram = StateDiagram::new();
        diagram.add_state(State {
            name: "fork1".to_string(),
            alias: None,
            description: None,
            stereotype: None,
            state_type: StateType::Fork,
            substates: Vec::new(),
            internal_transitions: Vec::new(),
            regions: Vec::new(),
            color: None,
            entry_action: None,
            exit_action: None,
            do_action: None,
        });
        
        diagram.add_transition(Transition::new("[*]", "fork1"));
        diagram.add_transition(Transition::new("fork1", "Task1"));
        diagram.add_transition(Transition::new("fork1", "Task2"));

        let engine = StateLayoutEngine::new();
        let result = engine.layout(&diagram);

        assert!(!result.elements.is_empty());
    }
}
