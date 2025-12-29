//! Трейты для layout engines

use crate::{LayoutConfig, LayoutElement, Rect};

/// Результат layout
#[derive(Debug, Clone)]
pub struct LayoutResult {
    /// Все размещённые элементы
    pub elements: Vec<LayoutElement>,
    /// Общий размер диаграммы
    pub bounds: Rect,
}

impl LayoutResult {
    /// Создаёт пустой результат
    pub fn empty() -> Self {
        Self {
            elements: Vec::new(),
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
        }
    }
    
    /// Вычисляет bounds по всем элементам
    pub fn calculate_bounds(&mut self) {
        if self.elements.is_empty() {
            return;
        }
        
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        
        for elem in &self.elements {
            min_x = min_x.min(elem.bounds.x);
            min_y = min_y.min(elem.bounds.y);
            max_x = max_x.max(elem.bounds.x + elem.bounds.width);
            max_y = max_y.max(elem.bounds.y + elem.bounds.height);
        }
        
        self.bounds = Rect::new(min_x, min_y, max_x - min_x, max_y - min_y);
    }
}

/// Трейт для layout engines
pub trait LayoutEngine {
    /// Тип входных данных (AST диаграммы)
    type Input;
    
    /// Выполняет layout диаграммы
    fn layout(&self, input: &Self::Input, config: &LayoutConfig) -> LayoutResult;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ElementType;
    
    #[test]
    fn test_calculate_bounds() {
        let mut result = LayoutResult {
            elements: vec![
                LayoutElement {
                    id: "a".to_string(),
                    bounds: Rect::new(0.0, 0.0, 50.0, 30.0),
                    element_type: ElementType::Rectangle {
                        label: "A".to_string(),
                        corner_radius: 0.0,
                    },
                },
                LayoutElement {
                    id: "b".to_string(),
                    bounds: Rect::new(100.0, 50.0, 50.0, 30.0),
                    element_type: ElementType::Rectangle {
                        label: "B".to_string(),
                        corner_radius: 0.0,
                    },
                },
            ],
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
        };
        
        result.calculate_bounds();
        
        assert_eq!(result.bounds.x, 0.0);
        assert_eq!(result.bounds.y, 0.0);
        assert_eq!(result.bounds.width, 150.0);
        assert_eq!(result.bounds.height, 80.0);
    }
}
