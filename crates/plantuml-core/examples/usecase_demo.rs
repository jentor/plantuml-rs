//! Демонстрация рендеринга Use Case Diagrams
//!
//! Запуск: cargo run -p plantuml-core --example usecase_demo

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    // Создаём директорию для выходных файлов
    let output_dir = Path::new("target/usecase_examples");
    fs::create_dir_all(output_dir).expect("Не удалось создать директорию");

    // Пример 1: Простая диаграмма
    let simple = r#"
@startuml
actor User
actor Admin

usecase Login
usecase Dashboard
usecase Settings

User --> Login
User --> Dashboard
Admin --> Settings
Admin --> Dashboard
@enduml
"#;

    // Пример 2: С пакетом/системой
    let with_system = r#"
@startuml
actor Customer
actor Manager

rectangle "E-Commerce System" {
    usecase "Browse Products"
    usecase "Add to Cart"
    usecase Checkout
    usecase "Manage Orders"
}

Customer --> (Browse Products)
Customer --> (Add to Cart)
Customer --> (Checkout)
Manager --> (Manage Orders)
@enduml
"#;

    // Пример 3: Include и Extend
    let include_extend = r#"
@startuml
actor User

usecase Login
usecase "Verify Credentials"
usecase "Two Factor Auth"
usecase Dashboard

User --> Login
Login ..> (Verify Credentials) : <<include>>
Login ..> (Two Factor Auth) : <<extend>>
Login --> Dashboard
@enduml
"#;

    // Пример 4: С алиасами
    let with_aliases = r#"
@startuml
actor "Web User" as WU
actor "Mobile User" as MU
actor "System Administrator" as SA

usecase "Browse Catalog" as UC1
usecase "Make Purchase" as UC2
usecase "Manage System" as UC3

WU --> UC1
WU --> UC2
MU --> UC1
MU --> UC2
SA --> UC3
@enduml
"#;

    // Пример 5: Colon syntax
    let colon_syntax = r#"
@startuml
:User: --> (Login)
:User: --> (View Profile)
:Admin: --> (Manage Users)
(Login) --> (Dashboard)
@enduml
"#;

    // Пример 6: Наследование актёров
    let generalization = r#"
@startuml
actor User
actor Admin
actor SuperAdmin

usecase Login
usecase Settings
usecase "System Config"

User --> Login
Admin --> Settings
Admin --|> User
SuperAdmin --> (System Config)
SuperAdmin --|> Admin
@enduml
"#;

    // Рендерим все примеры
    let examples = [
        ("simple", simple),
        ("with_system", with_system),
        ("include_extend", include_extend),
        ("with_aliases", with_aliases),
        ("colon_syntax", colon_syntax),
        ("generalization", generalization),
    ];

    let options = RenderOptions::default();

    for (name, source) in examples.iter() {
        match render(source, &options) {
            Ok(svg) => {
                let path = output_dir.join(format!("{}.svg", name));
                fs::write(&path, &svg).expect("Не удалось записать файл");
                println!("✓ Сохранено: {}", path.display());
            }
            Err(e) => {
                eprintln!("✗ Ошибка в примере {}: {:?}", name, e);
            }
        }
    }

    println!("\nГотово! Проверьте файлы в {}", output_dir.display());
}
