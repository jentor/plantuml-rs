//! Демонстрация рендеринга MindMap диаграмм
//!
//! Запуск: cargo run -p plantuml-core --example mindmap_demo

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    // Создаём директорию для выходных файлов
    let output_dir = Path::new("target/mindmap_examples");
    fs::create_dir_all(output_dir).expect("Не удалось создать директорию");

    // Пример 1: Простая MindMap
    let simple = r#"
@startmindmap
* Project
** Planning
** Development
** Testing
@endmindmap
"#;

    // Пример 2: Глубокая иерархия
    let deep_hierarchy = r#"
@startmindmap
* Operating Systems
** Windows
*** Windows 10
*** Windows 11
** Linux
*** Ubuntu
**** Desktop
**** Server
*** Fedora
*** Arch
** macOS
*** Monterey
*** Ventura
@endmindmap
"#;

    // Пример 3: С заголовком
    let with_title = r#"
@startmindmap
title Software Development Lifecycle
* SDLC
** Requirements
*** Gathering
*** Analysis
** Design
*** Architecture
*** UI/UX
** Implementation
*** Coding
*** Code Review
** Testing
*** Unit Tests
*** Integration
** Deployment
@endmindmap
"#;

    // Пример 4: OrgMode стиль (левая/правая сторона)
    let orgmode_style = r#"
@startmindmap
+ Root Topic
++ Right Branch 1
+++ Sub-topic
++ Right Branch 2
-- Left Branch 1
--- Sub-topic
-- Left Branch 2
@endmindmap
"#;

    // Пример 5: Со стилями узлов
    let with_styles = r#"
@startmindmap
* Root
**_ Box Style Node
**- No Border Node
** Default Style
@endmindmap
"#;

    // Пример 6: С цветами
    let with_colors = r#"
@startmindmap
*[#FF6B6B] Urgent Tasks
**[#4ECDC4] High Priority
***[#45B7D1] Task 1
***[#96CEB4] Task 2
**[#FFEAA7] Medium Priority
*** Task 3
*** Task 4
@endmindmap
"#;

    // Пример 7: Большое дерево
    let large_tree = r#"
@startmindmap
* Company Structure
** Engineering
*** Frontend
**** React Team
**** Vue Team
*** Backend
**** API Team
**** Database Team
*** DevOps
**** CI/CD
**** Infrastructure
** Product
*** Product Management
*** UX Design
*** Research
** Sales
*** Enterprise
*** SMB
*** Partners
** HR
*** Recruiting
*** Training
@endmindmap
"#;

    let examples = vec![
        ("simple", simple),
        ("deep_hierarchy", deep_hierarchy),
        ("with_title", with_title),
        ("orgmode_style", orgmode_style),
        ("with_styles", with_styles),
        ("with_colors", with_colors),
        ("large_tree", large_tree),
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

    println!("\nГотово! Проверьте файлы в target/mindmap_examples");
}
