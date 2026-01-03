//! Демонстрация рендеринга Component Diagrams
//!
//! Запуск: cargo run -p plantuml-core --example component_demo

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    // Создаём директорию для выходных файлов
    let output_dir = Path::new("target/component_examples");
    fs::create_dir_all(output_dir).expect("Не удалось создать директорию");

    // Пример 1: Простые компоненты
    let simple = r#"
@startuml
component API
component Worker
database MySQL
cloud AWS

API --> Worker
Worker --> MySQL
API --> AWS
@enduml
"#;

    // Пример 2: Bracket синтаксис
    let bracket = r#"
@startuml
[User Service]
[Order Service]
[Payment Service]

[User Service] --> [Order Service] : creates orders
[Order Service] --> [Payment Service] : process payment
@enduml
"#;

    // Пример 3: С пакетами
    let with_packages = r#"
@startuml
package "Backend" {
    component API
    component Worker
    database PostgreSQL
}

package "Frontend" {
    component WebApp
    component MobileApp
}

WebApp --> API
MobileApp --> API
API --> Worker
Worker --> PostgreSQL
@enduml
"#;

    // Пример 4: Различные типы компонентов
    let various_types = r#"
@startuml
actor User
component "Web Application" as WebApp
database PostgreSQL
queue RabbitMQ
cloud "AWS S3" as S3
storage FileSystem
node "Application Server" as AppServer

User --> WebApp
WebApp --> PostgreSQL : SQL
WebApp --> RabbitMQ : async
WebApp --> S3 : files
AppServer --> FileSystem
@enduml
"#;

    // Пример 5: С интерфейсами
    let with_interfaces = r#"
@startuml
interface HTTP
interface REST

component API
component Gateway

Gateway --> HTTP
API --> REST
HTTP --> API
@enduml
"#;

    // Пример 6: Микросервисы
    let microservices = r#"
@startuml
cloud "API Gateway" as GW

package "Services" {
    component UserService
    component OrderService
    component PaymentService
    component NotificationService
}

database UserDB
database OrderDB
queue Kafka

GW --> UserService
GW --> OrderService
GW --> PaymentService

UserService --> UserDB
OrderService --> OrderDB
OrderService --> Kafka
PaymentService --> Kafka
NotificationService --> Kafka
@enduml
"#;

    // Рендерим все примеры
    let examples = [
        ("simple", simple),
        ("bracket", bracket),
        ("with_packages", with_packages),
        ("various_types", various_types),
        ("with_interfaces", with_interfaces),
        ("microservices", microservices),
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
