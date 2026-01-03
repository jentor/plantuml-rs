//! Демонстрация JSON диаграмм

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    let examples = vec![
        (
            "simple",
            r#"@startjson
{
  "name": "John Doe",
  "age": 30,
  "active": true
}
@endjson"#,
        ),
        (
            "nested",
            r#"@startjson
{
  "user": {
    "id": 1,
    "name": "Alice",
    "email": "alice@example.com"
  },
  "settings": {
    "theme": "dark",
    "notifications": true
  }
}
@endjson"#,
        ),
        (
            "with_arrays",
            r#"@startjson
{
  "project": "plantuml-rs",
  "version": "0.1.0",
  "authors": ["Alice", "Bob", "Charlie"],
  "dependencies": {
    "serde": "1.0",
    "pest": "2.7"
  }
}
@endjson"#,
        ),
        (
            "complex",
            r#"@startjson
{
  "company": "Acme Corp",
  "employees": [
    {
      "name": "John",
      "role": "Developer",
      "skills": ["Rust", "Python", "Go"]
    },
    {
      "name": "Jane",
      "role": "Designer",
      "skills": ["Figma", "Sketch"]
    }
  ],
  "locations": {
    "headquarters": {
      "city": "New York",
      "country": "USA"
    },
    "branches": ["London", "Tokyo", "Sydney"]
  }
}
@endjson"#,
        ),
        (
            "api_response",
            r#"@startjson
title API Response

{
  "status": "success",
  "code": 200,
  "data": {
    "users": [
      {"id": 1, "name": "Alice"},
      {"id": 2, "name": "Bob"}
    ],
    "total": 2,
    "page": 1
  },
  "meta": {
    "timestamp": "2024-01-01T00:00:00Z",
    "version": "v2"
  }
}
@endjson"#,
        ),
    ];

    // Создаём директорию для вывода
    let output_dir = Path::new("target/json_examples");
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

    println!("\nГотово! Проверьте файлы в target/json_examples");
}
