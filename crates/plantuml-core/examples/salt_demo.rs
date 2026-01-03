//! Демонстрация Salt (Wireframe) диаграмм

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    let examples = vec![
        (
            "login_form",
            r#"@startsalt
{+
  Login    | "MyName   "
  Password | "****     "
  [Cancel] | [  OK   ]
}
@endsalt"#,
        ),
        (
            "widgets",
            r#"@startsalt
{
  Just plain text
  [This is my button]
  ()
  (X)
  []
  [X]
  "Enter text here   "
  ^This is a droplist^
}
@endsalt"#,
        ),
        (
            "grid_all_borders",
            r#"@startsalt
{#
  Name   | "John Doe        "
  Email  | "john@example.com"
  Phone  | "+1234567890     "
  [Save] | [Cancel]
}
@endsalt"#,
        ),
        (
            "separators",
            r#"@startsalt
{
  Header text
  ..
  Some content here
  ==
  Another section
  ~~
  More text
  --
  [Submit]
}
@endsalt"#,
        ),
        (
            "complex_form",
            r#"@startsalt
{+
  Username:    | "admin           "
  Password:    | "********        "
  ..
  Remember me: | [X]
  Theme:       | ^Dark mode^
  ..
  [Cancel] | [Apply] | [  OK  ]
}
@endsalt"#,
        ),
    ];

    // Создаём директорию для вывода
    let output_dir = Path::new("target/salt_examples");
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

    println!("\nГотово! Проверьте файлы в target/salt_examples");
}
