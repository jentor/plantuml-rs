//! Демонстрация YAML диаграмм

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    let examples = vec![
        (
            "simple",
            r#"@startyaml
name: John Doe
age: 30
active: true
@endyaml"#,
        ),
        (
            "inline_objects",
            r#"@startyaml
user: {name: Alice, role: admin}
settings: {theme: dark, lang: en}
@endyaml"#,
        ),
        (
            "config",
            r#"@startyaml
title Application Config
server:
  host: localhost
  port: 8080
database:
  driver: postgres
  name: myapp
@endyaml"#,
        ),
    ];

    // Создаём директорию для вывода
    let output_dir = Path::new("target/yaml_examples");
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

    println!("\nГотово! Проверьте файлы в target/yaml_examples");
}
