//! Визуальные тесты для state diagrams
//!
//! Используем insta для snapshot тестирования SVG вывода.

use plantuml_core::{render, RenderOptions};

/// Тест простой state diagram
#[test]
fn test_simple_state_svg() {
    let source = r#"@startuml
[*] --> Active
Active --> Inactive : timeout
Inactive --> Active : resume
Active --> [*] : close
@enduml"#;

    let svg = render(source, &RenderOptions::default()).unwrap();

    // Проверяем что SVG валидный
    assert!(svg.contains("<?xml"));
    assert!(svg.contains("<svg"));
    assert!(svg.contains("Active"));
    assert!(svg.contains("Inactive"));

    // Snapshot тест
    insta::assert_snapshot!("simple_state", svg);
}

/// Тест state diagram с определениями состояний
#[test]
fn test_state_with_definitions_svg() {
    let source = r#"@startuml
state Active {
  state Processing
  state Waiting
  Processing --> Waiting
  Waiting --> Processing
}

[*] --> Active
Active --> [*]
@enduml"#;

    let svg = render(source, &RenderOptions::default()).unwrap();

    assert!(svg.contains("<?xml"));
    assert!(svg.contains("<svg"));
    assert!(svg.contains("Active"));

    insta::assert_snapshot!("state_with_definitions", svg);
}

/// Тест state diagram с choice point
#[test]
fn test_state_with_choice_svg() {
    let source = r#"@startuml
state check <<choice>>
[*] --> check
check --> Valid : [valid]
check --> Invalid : [invalid]
Valid --> [*]
Invalid --> [*]
@enduml"#;

    let svg = render(source, &RenderOptions::default()).unwrap();

    assert!(svg.contains("<?xml"));
    assert!(svg.contains("<svg"));

    insta::assert_snapshot!("state_with_choice", svg);
}
