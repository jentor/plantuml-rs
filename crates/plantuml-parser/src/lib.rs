//! # plantuml-parser
//!
//! Лексер и парсер для PlantUML синтаксиса.
//!
//! Использует двухфазный подход:
//! 1. Лексер (logos) — быстрая токенизация
//! 2. Парсер (pest) — PEG грамматики для структуры

pub mod error;
pub mod lexer;
pub mod parsers;

pub use error::ParseError;
pub use plantuml_ast::Diagram;
pub use parsers::parse_sequence;

/// Результат парсинга
pub type Result<T> = std::result::Result<T, ParseError>;

/// Парсит PlantUML исходный код и возвращает AST диаграммы.
///
/// # Аргументы
/// * `source` - исходный код PlantUML
///
/// # Возвращает
/// * `Ok(Diagram)` - распарсенная диаграмма
/// * `Err(ParseError)` - ошибка парсинга
///
/// # Пример
///
/// ```rust
/// use plantuml_parser::parse;
///
/// let source = "@startuml\nAlice -> Bob: Hello\n@enduml";
/// let diagram = parse(source);
/// assert!(diagram.is_ok());
/// ```
pub fn parse(source: &str) -> Result<Diagram> {
    // Определяем тип диаграммы
    let diagram_type = detect_diagram_type(source)?;
    
    match diagram_type {
        DiagramKind::Sequence => parse_sequence_diagram(source),
        DiagramKind::Class => parse_class(source),
        DiagramKind::Activity => parse_activity(source),
        DiagramKind::State => parse_state(source),
        DiagramKind::Component => parse_component(source),
        DiagramKind::UseCase => parse_usecase(source),
        DiagramKind::Unknown => Err(ParseError::UnknownDiagramType),
    }
}

/// Тип диаграммы (для внутреннего использования)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiagramKind {
    Sequence,
    Class,
    Activity,
    State,
    Component,
    UseCase,
    Unknown,
}

/// Определяет тип диаграммы по содержимому
fn detect_diagram_type(source: &str) -> Result<DiagramKind> {
    let source_lower = source.to_lowercase();
    
    // State Diagram — проверяем первой, так как [*] может смешаться с другими
    if source_lower.contains("[*] -->") 
        || source_lower.contains("--> [*]")
        || source_lower.contains("state ")
    {
        return Ok(DiagramKind::State);
    }
    
    // Class Diagram
    if source_lower.contains("class ") 
        || source_lower.contains("interface ")
        || source_lower.contains("abstract class")
        || source_lower.contains("<|--")
        || source_lower.contains("..|>")
    {
        return Ok(DiagramKind::Class);
    }
    
    // Activity Diagram
    if source_lower.contains(":start") 
        || source_lower.contains("start\n")
        || source_lower.contains("if (")
        || source_lower.contains("while (")
    {
        return Ok(DiagramKind::Activity);
    }
    
    // Component Diagram
    if source_lower.contains("[component]") 
        || source_lower.contains("database ")
        || source_lower.contains("node ")
        || (source_lower.contains("package ") && !source_lower.contains("class "))
    {
        return Ok(DiagramKind::Component);
    }
    
    // UseCase Diagram
    if source_lower.contains("usecase ") 
        || source_lower.contains("(usecase)")
        || (source_lower.contains("actor ") && source_lower.contains("rectangle "))
    {
        return Ok(DiagramKind::UseCase);
    }
    
    // Sequence Diagram — проверяем последней (самый распространённый)
    if source_lower.contains("participant") 
        || (source_lower.contains("actor") && source_lower.contains("->"))
        || source_lower.contains("-->")
        || source_lower.contains("->>")
        || source_lower.contains("->")
    {
        return Ok(DiagramKind::Sequence);
    }
    
    Ok(DiagramKind::Unknown)
}

// Парсер для sequence diagrams использует pest грамматику

fn parse_sequence_diagram(source: &str) -> Result<Diagram> {
    let diagram = parsers::parse_sequence(source)?;
    Ok(Diagram::Sequence(diagram))
}

fn parse_class(source: &str) -> Result<Diagram> {
    use plantuml_ast::class::ClassDiagram;
    
    let diagram = ClassDiagram::new();
    let _ = source;
    
    Ok(Diagram::Class(diagram))
}

fn parse_activity(source: &str) -> Result<Diagram> {
    use plantuml_ast::activity::ActivityDiagram;
    
    let diagram = ActivityDiagram::new();
    let _ = source;
    
    Ok(Diagram::Activity(diagram))
}

fn parse_state(source: &str) -> Result<Diagram> {
    use plantuml_ast::state::StateDiagram;
    
    let diagram = StateDiagram::new();
    let _ = source;
    
    Ok(Diagram::State(diagram))
}

fn parse_component(source: &str) -> Result<Diagram> {
    use plantuml_ast::component::ComponentDiagram;
    
    let diagram = ComponentDiagram::new();
    let _ = source;
    
    Ok(Diagram::Component(diagram))
}

fn parse_usecase(source: &str) -> Result<Diagram> {
    use plantuml_ast::usecase::UseCaseDiagram;
    
    let diagram = UseCaseDiagram::new();
    let _ = source;
    
    Ok(Diagram::UseCase(diagram))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_sequence() {
        let source = "@startuml\nAlice -> Bob: Hello\n@enduml";
        assert_eq!(detect_diagram_type(source).unwrap(), DiagramKind::Sequence);
    }
    
    #[test]
    fn test_detect_class() {
        let source = "@startuml\nclass User\n@enduml";
        assert_eq!(detect_diagram_type(source).unwrap(), DiagramKind::Class);
    }
    
    #[test]
    fn test_detect_activity() {
        let source = "@startuml\nstart\n:Action;\nstop\n@enduml";
        assert_eq!(detect_diagram_type(source).unwrap(), DiagramKind::Activity);
    }
    
    #[test]
    fn test_detect_state() {
        let source = "@startuml\n[*] --> Active\n@enduml";
        assert_eq!(detect_diagram_type(source).unwrap(), DiagramKind::State);
    }
    
    #[test]
    fn test_parse_returns_diagram() {
        let source = "@startuml\nAlice -> Bob\n@enduml";
        let result = parse(source);
        assert!(result.is_ok());
    }
}
