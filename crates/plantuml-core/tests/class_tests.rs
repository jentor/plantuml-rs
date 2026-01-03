//! Визуальные тесты для class diagrams
//!
//! Используем insta для snapshot тестирования SVG вывода.

use plantuml_core::{render, RenderOptions};

/// Тест простой class diagram
#[test]
fn test_simple_class_svg() {
    let source = r#"@startuml
class User {
    -id: Long
    -name: String
    +getId(): Long
    +getName(): String
}
@enduml"#;

    let svg = render(source, &RenderOptions::default()).unwrap();

    // Проверяем что SVG валидный
    assert!(svg.contains("<?xml"));
    assert!(svg.contains("<svg"));
    assert!(svg.contains("User"));

    // Snapshot тест
    insta::assert_snapshot!("simple_class", svg);
}

/// Тест наследования
#[test]
fn test_inheritance_svg() {
    let source = r#"@startuml
class Animal {
    +eat()
}

class Dog {
    +bark()
}

class Cat {
    +meow()
}

Dog --|> Animal
Cat --|> Animal
@enduml"#;

    let svg = render(source, &RenderOptions::default()).unwrap();

    assert!(svg.contains("Animal"));
    assert!(svg.contains("Dog"));
    assert!(svg.contains("Cat"));

    insta::assert_snapshot!("inheritance", svg);
}

/// Тест интерфейса и реализации
#[test]
fn test_interface_svg() {
    let source = r#"@startuml
interface Serializable {
    +serialize(): String
}

class User {
    -name: String
    +serialize(): String
}

User ..|> Serializable
@enduml"#;

    let svg = render(source, &RenderOptions::default()).unwrap();

    assert!(svg.contains("Serializable"));
    assert!(svg.contains("User"));

    insta::assert_snapshot!("interface", svg);
}

/// Тест композиции и агрегации
#[test]
fn test_composition_aggregation_svg() {
    let source = r#"@startuml
class Car {
    -engine: Engine
    -wheels: List<Wheel>
}

class Engine {
    -power: int
}

class Wheel {
    -size: int
}

Car *-- Engine : contains
Car o-- Wheel : has
@enduml"#;

    let svg = render(source, &RenderOptions::default()).unwrap();

    assert!(svg.contains("Car"));
    assert!(svg.contains("Engine"));
    assert!(svg.contains("Wheel"));

    insta::assert_snapshot!("composition_aggregation", svg);
}

/// Тест сложной иерархии
#[test]
fn test_complex_hierarchy_svg() {
    let source = r#"@startuml
interface Repository<T> {
    +findById(id): T
    +save(entity): T
}

abstract class AbstractRepository<T> {
    #entityClass: Class
    +findById(id): T
}

class UserRepository {
    +findByName(name): User
}

class ProductRepository {
    +findByCategory(cat): List<Product>
}

AbstractRepository ..|> Repository
UserRepository --|> AbstractRepository
ProductRepository --|> AbstractRepository
@enduml"#;

    let svg = render(source, &RenderOptions::default()).unwrap();

    assert!(svg.contains("Repository"));
    assert!(svg.contains("AbstractRepository"));
    assert!(svg.contains("UserRepository"));
    assert!(svg.contains("ProductRepository"));

    insta::assert_snapshot!("complex_hierarchy", svg);
}
