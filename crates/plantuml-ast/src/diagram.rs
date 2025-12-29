//! Главный enum Diagram, объединяющий все типы диаграмм.

use serde::{Deserialize, Serialize};

use crate::activity::ActivityDiagram;
use crate::class::ClassDiagram;
use crate::common::DiagramMetadata;
use crate::component::ComponentDiagram;
use crate::sequence::SequenceDiagram;
use crate::state::StateDiagram;
use crate::usecase::UseCaseDiagram;

/// Тип диаграммы PlantUML
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiagramType {
    Sequence,
    Class,
    Activity,
    State,
    Component,
    Deployment,
    UseCase,
    Object,
    Timing,
    // Non-UML
    Gantt,
    MindMap,
    Wbs,
    Json,
    Yaml,
    Network,
    Salt,
    Er,
    Archimate,
}

/// Корневой enum для всех типов диаграмм
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Diagram {
    /// Диаграмма последовательностей
    Sequence(SequenceDiagram),
    /// Диаграмма классов
    Class(ClassDiagram),
    /// Диаграмма активностей
    Activity(ActivityDiagram),
    /// Диаграмма состояний
    State(StateDiagram),
    /// Диаграмма компонентов
    Component(ComponentDiagram),
    /// Диаграмма развёртывания (использует ComponentDiagram)
    Deployment(ComponentDiagram),
    /// Диаграмма вариантов использования
    UseCase(UseCaseDiagram),
    // TODO: Добавить остальные типы по мере реализации
}

impl Diagram {
    /// Возвращает тип диаграммы
    pub fn diagram_type(&self) -> DiagramType {
        match self {
            Diagram::Sequence(_) => DiagramType::Sequence,
            Diagram::Class(_) => DiagramType::Class,
            Diagram::Activity(_) => DiagramType::Activity,
            Diagram::State(_) => DiagramType::State,
            Diagram::Component(_) => DiagramType::Component,
            Diagram::Deployment(_) => DiagramType::Deployment,
            Diagram::UseCase(_) => DiagramType::UseCase,
        }
    }

    /// Возвращает метаданные диаграммы
    pub fn metadata(&self) -> &DiagramMetadata {
        match self {
            Diagram::Sequence(d) => &d.metadata,
            Diagram::Class(d) => &d.metadata,
            Diagram::Activity(d) => &d.metadata,
            Diagram::State(d) => &d.metadata,
            Diagram::Component(d) => &d.metadata,
            Diagram::Deployment(d) => &d.metadata,
            Diagram::UseCase(d) => &d.metadata,
        }
    }
}
