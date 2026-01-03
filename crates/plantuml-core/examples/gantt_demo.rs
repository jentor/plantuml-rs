//! Демонстрация рендеринга Gantt Diagrams
//!
//! Запуск: cargo run -p plantuml-core --example gantt_demo

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    // Создаём директорию для выходных файлов
    let output_dir = Path::new("target/gantt_examples");
    fs::create_dir_all(output_dir).expect("Не удалось создать директорию");

    // Пример 1: Простая диаграмма Ганта
    let simple = r#"
@startgantt
[Design] lasts 5 days
[Implementation] lasts 10 days
[Testing] lasts 5 days
@endgantt
"#;

    // Пример 2: С датой начала проекта
    let with_date = r#"
@startgantt
project starts 2024-01-15
[Analysis] lasts 3 days
[Design] lasts 5 days
[Development] lasts 15 days
[Testing] lasts 7 days
@endgantt
"#;

    // Пример 3: С зависимостями
    let dependencies = r#"
@startgantt
[Design] as [T1] lasts 5 days
[Frontend] as [T2] starts after [T1] lasts 10 days
[Backend] as [T3] starts after [T1] lasts 12 days
[Integration] starts after [T2] lasts 5 days
@endgantt
"#;

    // Пример 4: С then statement
    let then_example = r#"
@startgantt
[Requirements] lasts 3 days
then [Architecture] lasts 5 days
then [Implementation] lasts 15 days
then [Testing] lasts 7 days
then [Deployment] lasts 2 days
@endgantt
"#;

    // Пример 5: С процентом выполнения
    let progress = r#"
@startgantt
[Task 1] lasts 5 days is 100% completed
[Task 2] lasts 8 days is 75% completed
[Task 3] lasts 6 days is 50% completed
[Task 4] lasts 10 days is 25% completed
[Task 5] lasts 4 days is 0% completed
@endgantt
"#;

    // Пример 6: С разделителями
    let separators = r#"
@startgantt
-- Phase 1: Planning --
[Requirements] lasts 3 days
[Design] lasts 5 days
-- Phase 2: Development --
[Frontend] lasts 10 days
[Backend] lasts 12 days
-- Phase 3: Release --
[Testing] lasts 5 days
[Deployment] lasts 2 days
@endgantt
"#;

    // Пример 7: С выходными днями
    let weekends = r#"
@startgantt
saturday are closed
sunday are closed
[Development Sprint 1] lasts 10 days
then [Development Sprint 2] lasts 10 days
then [Release] lasts 5 days
@endgantt
"#;

    // Рендерим все примеры
    let examples = [
        ("simple", simple),
        ("with_date", with_date),
        ("dependencies", dependencies),
        ("then_example", then_example),
        ("progress", progress),
        ("separators", separators),
        ("weekends", weekends),
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
