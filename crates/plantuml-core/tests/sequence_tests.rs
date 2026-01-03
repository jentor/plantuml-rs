//! Визуальные тесты для sequence diagrams
//!
//! Используем insta для snapshot тестирования SVG вывода.

use plantuml_core::{render, RenderOptions};

/// Тест простой sequence diagram
#[test]
fn test_simple_sequence_svg() {
    let source = r#"@startuml
Alice -> Bob: Hello
Bob --> Alice: Hi
@enduml"#;

    let svg = render(source, &RenderOptions::default()).unwrap();

    // Проверяем что SVG валидный
    assert!(svg.contains("<?xml"));
    assert!(svg.contains("<svg"));
    assert!(svg.contains("Alice"));
    assert!(svg.contains("Bob"));
    assert!(svg.contains("Hello"));

    // Snapshot тест
    insta::assert_snapshot!("simple_sequence", svg);
}

/// Тест sequence diagram с участниками разных типов
#[test]
fn test_participant_types_svg() {
    let source = r#"@startuml
participant User
actor Admin
database DB

User -> Admin: Request
Admin -> DB: Query
DB --> Admin: Data
Admin --> User: Response
@enduml"#;

    let svg = render(source, &RenderOptions::default()).unwrap();

    // Проверяем базовые элементы SVG
    assert!(svg.contains("<?xml"));
    assert!(svg.contains("<svg"));
    assert!(svg.contains("participant_"));

    insta::assert_snapshot!("participant_types", svg);
}

/// Тест self-message
#[test]
fn test_self_message_svg() {
    let source = r#"@startuml
participant Server

Server -> Server: Process
Server -> Server: Validate
@enduml"#;

    let svg = render(source, &RenderOptions::default()).unwrap();

    assert!(svg.contains("Server"));
    assert!(svg.contains("Process"));

    insta::assert_snapshot!("self_message", svg);
}

/// Тест fragment (alt)
#[test]
fn test_alt_fragment_svg() {
    let source = r#"@startuml
Alice -> Bob: Request

alt success
    Bob --> Alice: OK
else failure
    Bob --> Alice: Error
end
@enduml"#;

    let svg = render(source, &RenderOptions::default()).unwrap();

    assert!(svg.contains("Alice"));
    assert!(svg.contains("Bob"));

    insta::assert_snapshot!("alt_fragment", svg);
}
