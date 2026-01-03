//! Демонстрация ER диаграмм

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    let examples = vec![
        (
            "simple",
            r#"@startuml
entity User {
  * id : int <<PK>>
  --
  name : varchar
  email : varchar
}
@enduml"#,
        ),
        (
            "ecommerce",
            r#"@startuml
entity User {
  * id : int <<PK>>
  --
  username : varchar
  email : varchar
  created_at : datetime
}

entity Order {
  * id : int <<PK>>
  --
  * user_id : int <<FK>>
  total : decimal
  status : varchar
  created_at : datetime
}

entity OrderItem {
  * id : int <<PK>>
  --
  * order_id : int <<FK>>
  * product_id : int <<FK>>
  quantity : int
  price : decimal
}

entity Product {
  * id : int <<PK>>
  --
  name : varchar
  description : text
  price : decimal
  stock : int
}

User ||--o{ Order : places
Order ||--|{ OrderItem : contains
Product ||--o{ OrderItem : included_in
@enduml"#,
        ),
        (
            "blog",
            r#"@startuml
entity Author {
  * id : int <<PK>>
  --
  name : varchar
  email : varchar
}

entity Post {
  * id : int <<PK>>
  --
  * author_id : int <<FK>>
  title : varchar
  content : text
  published : boolean
}

entity Comment {
  * id : int <<PK>>
  --
  * post_id : int <<FK>>
  author_name : varchar
  content : text
}

entity Tag {
  * id : int <<PK>>
  --
  name : varchar
}

Author ||--o{ Post : writes
Post ||--o{ Comment : has
Post }o--o{ Tag : tagged_with
@enduml"#,
        ),
    ];

    // Создаём директорию для вывода
    let output_dir = Path::new("target/er_examples");
    fs::create_dir_all(output_dir).expect("Не удалось создать директорию");

    let options = RenderOptions::default();

    for (name, source) in examples {
        match render(source, &options) {
            Ok(svg) => {
                let output_path = output_dir.join(format!("{}.svg", name));
                fs::write(&output_path, &svg).expect("Не удалось записать файл");
                println!("✓ Сохранено: {}", output_path.display());
            }
            Err(e) => {
                eprintln!("✗ Ошибка для {}: {}", name, e);
            }
        }
    }

    println!("\nГотово! Проверьте файлы в target/er_examples");
}
