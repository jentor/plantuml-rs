//! Демонстрация рендеринга WBS (Work Breakdown Structure) диаграмм
//!
//! Запуск: cargo run -p plantuml-core --example wbs_demo

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    // Создаём директорию для выходных файлов
    let output_dir = Path::new("target/wbs_examples");
    fs::create_dir_all(output_dir).expect("Не удалось создать директорию");

    // Пример 1: Простая WBS
    let simple = r#"@startwbs
* Project
** Planning
** Execution
** Closing
@endwbs"#;

    // Пример 2: Разработка ПО
    let software_dev = r#"@startwbs
* Software Development
** Requirements
*** Gathering
*** Analysis
** Design
*** Architecture
*** Database
*** UI/UX
** Implementation
*** Backend
*** Frontend
*** Mobile
** Testing
*** Unit Tests
*** Integration
*** UAT
** Deployment
@endwbs"#;

    // Пример 3: С заголовком
    let with_title = r#"@startwbs
title Project Work Breakdown Structure
* New Product Launch
** Market Research
** Product Design
** Manufacturing
** Marketing
** Distribution
@endwbs"#;

    // Пример 4: Глубокая иерархия
    let deep_hierarchy = r#"@startwbs
* Company Restructuring
** Phase 1
*** Assessment
**** Current State Analysis
**** Gap Identification
*** Planning
**** Strategy Definition
**** Resource Allocation
** Phase 2
*** Implementation
**** Team Formation
**** Process Changes
*** Monitoring
** Phase 3
*** Evaluation
*** Optimization
@endwbs"#;

    // Пример 5: IT проект
    let it_project = r#"@startwbs
* IT Infrastructure Upgrade
** Hardware
*** Servers
*** Network Equipment
*** Workstations
** Software
*** Operating Systems
*** Applications
*** Security Tools
** Training
*** Admin Training
*** User Training
** Documentation
@endwbs"#;

    let examples = vec![
        ("simple", simple),
        ("software_dev", software_dev),
        ("with_title", with_title),
        ("deep_hierarchy", deep_hierarchy),
        ("it_project", it_project),
    ];

    let options = RenderOptions::default();

    for (name, source) in examples {
        match render(source, &options) {
            Ok(svg) => {
                let path = output_dir.join(format!("{}.svg", name));
                fs::write(&path, &svg).expect("Не удалось записать файл");
                println!("✓ Сохранено: {}", path.display());
            }
            Err(e) => {
                eprintln!("✗ Ошибка рендеринга {}: {:?}", name, e);
            }
        }
    }

    println!("\nГотово! Проверьте файлы в target/wbs_examples");
}
