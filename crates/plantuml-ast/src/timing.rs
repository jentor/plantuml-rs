//! AST типы для Timing Diagrams (временных диаграмм).
//!
//! Timing Diagram показывает изменение состояний объектов во времени.

use serde::{Deserialize, Serialize};

use crate::common::{Color, DiagramMetadata, Note};

/// Диаграмма временных последовательностей
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimingDiagram {
    /// Метаданные диаграммы
    pub metadata: DiagramMetadata,
    /// Участники (линии времени)
    pub participants: Vec<TimingParticipant>,
    /// События изменения состояний
    pub state_changes: Vec<StateChange>,
    /// Ограничения времени
    pub constraints: Vec<TimeConstraint>,
    /// Заметки
    pub notes: Vec<Note>,
}

impl TimingDiagram {
    /// Создаёт новую пустую диаграмму
    pub fn new() -> Self {
        Self::default()
    }

    /// Добавляет участника
    pub fn add_participant(&mut self, participant: TimingParticipant) {
        self.participants.push(participant);
    }

    /// Добавляет изменение состояния
    pub fn add_state_change(&mut self, change: StateChange) {
        self.state_changes.push(change);
    }
}

/// Тип участника временной диаграммы
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ParticipantType {
    /// robust — полное отображение состояний (прямоугольники)
    #[default]
    Robust,
    /// concise — компактное отображение (линии)
    Concise,
    /// clock — тактовый сигнал
    Clock,
    /// binary — бинарный сигнал (0/1)
    Binary,
}

impl ParticipantType {
    /// Парсит тип из строки
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "robust" => Some(Self::Robust),
            "concise" => Some(Self::Concise),
            "clock" => Some(Self::Clock),
            "binary" => Some(Self::Binary),
            _ => None,
        }
    }
}

/// Участник временной диаграммы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingParticipant {
    /// Имя участника
    pub name: String,
    /// Алиас (короткое имя)
    pub alias: Option<String>,
    /// Тип участника
    pub participant_type: ParticipantType,
    /// Возможные состояния (для robust/concise)
    pub states: Vec<String>,
    /// Цвет
    pub color: Option<Color>,
}

impl TimingParticipant {
    /// Создаёт нового участника типа robust
    pub fn robust(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            alias: None,
            participant_type: ParticipantType::Robust,
            states: Vec::new(),
            color: None,
        }
    }

    /// Создаёт нового участника типа concise
    pub fn concise(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            alias: None,
            participant_type: ParticipantType::Concise,
            states: Vec::new(),
            color: None,
        }
    }

    /// Создаёт clock
    pub fn clock(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            alias: None,
            participant_type: ParticipantType::Clock,
            states: Vec::new(),
            color: None,
        }
    }

    /// Устанавливает алиас
    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }

    /// Добавляет возможное состояние
    pub fn add_state(&mut self, state: impl Into<String>) {
        self.states.push(state.into());
    }

    /// Возвращает имя для использования (алиас или имя)
    pub fn display_name(&self) -> &str {
        self.alias.as_deref().unwrap_or(&self.name)
    }
}

/// Изменение состояния участника в определённый момент времени
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    /// Участник (имя или алиас)
    pub participant: String,
    /// Момент времени
    pub time: TimeValue,
    /// Новое состояние
    pub state: String,
    /// Описание/метка
    pub label: Option<String>,
}

impl StateChange {
    /// Создаёт новое изменение состояния
    pub fn new(participant: impl Into<String>, time: TimeValue, state: impl Into<String>) -> Self {
        Self {
            participant: participant.into(),
            time,
            state: state.into(),
            label: None,
        }
    }

    /// Устанавливает метку
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Значение времени
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeValue {
    /// Абсолютное время (число)
    Absolute(f64),
    /// Относительное время (+offset)
    Relative(f64),
    /// Именованный момент
    Named(String),
}

impl TimeValue {
    /// Создаёт абсолютное время
    pub fn absolute(value: f64) -> Self {
        Self::Absolute(value)
    }

    /// Создаёт относительное время
    pub fn relative(offset: f64) -> Self {
        Self::Relative(offset)
    }

    /// Возвращает числовое значение (для сортировки)
    pub fn as_f64(&self) -> f64 {
        match self {
            TimeValue::Absolute(v) => *v,
            TimeValue::Relative(v) => *v,
            TimeValue::Named(_) => 0.0,
        }
    }
}

/// Ограничение времени (стрелка между моментами)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConstraint {
    /// Начальный момент
    pub from_time: TimeValue,
    /// Конечный момент
    pub to_time: TimeValue,
    /// Метка (например: "{5 ms}")
    pub label: Option<String>,
}

impl TimeConstraint {
    /// Создаёт новое ограничение
    pub fn new(from: TimeValue, to: TimeValue) -> Self {
        Self {
            from_time: from,
            to_time: to,
            label: None,
        }
    }

    /// Устанавливает метку
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_participant() {
        let p = TimingParticipant::robust("Web Browser").with_alias("WB");
        assert_eq!(p.name, "Web Browser");
        assert_eq!(p.alias, Some("WB".to_string()));
        assert_eq!(p.participant_type, ParticipantType::Robust);
        assert_eq!(p.display_name(), "WB");
    }

    #[test]
    fn test_state_change() {
        let change = StateChange::new("WB", TimeValue::absolute(100.0), "Running");
        assert_eq!(change.participant, "WB");
        assert_eq!(change.state, "Running");
    }

    #[test]
    fn test_time_value() {
        assert_eq!(TimeValue::absolute(100.0).as_f64(), 100.0);
        assert_eq!(TimeValue::relative(50.0).as_f64(), 50.0);
    }
}
