//! Демонстрация рендеринга Object Diagrams
//!
//! Запуск: cargo run -p plantuml-core --example object_demo

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    // Создаём директорию для выходных файлов
    let output_dir = Path::new("target/object_examples");
    fs::create_dir_all(output_dir).expect("Не удалось создать директорию");

    // Пример 1: Простые объекты
    let simple = r#"
@startuml
object user1
object user2
object user3

user1 --> user2 : friend
user2 --> user3 : colleague
@enduml
"#;

    // Пример 2: Объекты с полями
    let with_fields = r#"
@startuml
object user1 {
    name = "John Doe"
    age = 30
    email = "john@example.com"
}

object user2 {
    name = "Jane Smith"
    age = 25
    email = "jane@example.com"
}

user1 --> user2 : friend
@enduml
"#;

    // Пример 3: Объекты с указанием класса
    let with_class = r#"
@startuml
object john : Person {
    firstName = "John"
    lastName = "Doe"
}

object address1 : Address {
    street = "Main Street 123"
    city = "New York"
    zip = "10001"
}

john --> address1 : lives at
@enduml
"#;

    // Пример 4: Разные типы связей
    let link_types = r#"
@startuml
object company {
    name = "ACME Corp"
}

object department {
    name = "Engineering"
}

object employee {
    name = "John"
}

company *-- department : contains
department o-- employee : has
employee --> company : works for
@enduml
"#;

    // Пример 5: Map (ассоциативный массив)
    let map_example = r#"
@startuml
map config {
    host => localhost
    port => 8080
    debug => true
}

map database {
    driver => postgres
    name => mydb
    user => admin
}

config --> database : uses
@enduml
"#;

    // Пример 6: Социальная сеть
    let social_network = r#"
@startuml
object alice {
    username = "alice"
    followers = 1500
}

object bob {
    username = "bob"
    followers = 800
}

object charlie {
    username = "charlie"
    followers = 2000
}

object post1 {
    content = "Hello World!"
    likes = 42
}

alice --> bob : follows
alice --> charlie : follows
bob --> charlie : follows
alice --> post1 : created
bob --> post1 : liked
@enduml
"#;

    // Рендерим все примеры
    let examples = [
        ("simple", simple),
        ("with_fields", with_fields),
        ("with_class", with_class),
        ("link_types", link_types),
        ("map_example", map_example),
        ("social_network", social_network),
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
