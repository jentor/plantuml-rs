//! # plantuml-core
//!
//! Pure Rust библиотека для рендеринга PlantUML диаграмм.
//!
//! Это главный фасад библиотеки, предоставляющий простой API для:
//! - Парсинга PlantUML исходного кода
//! - Рендеринга диаграмм в SVG
//! - Настройки тем и стилей
//!
//! ## Быстрый старт
//!
//! ```rust
//! use plantuml_core::{render, RenderOptions};
//!
//! let source = r#"
//! @startuml
//! Alice -> Bob: Hello
//! Bob --> Alice: Hi
//! @enduml
//! "#;
//!
//! let svg = render(source, &RenderOptions::default()).unwrap();
//! println!("{}", svg);
//! ```
//!
//! ## Архитектура
//!
//! Библиотека состоит из нескольких crates:
//!
//! - `plantuml-ast` — AST типы для всех диаграмм
//! - `plantuml-parser` — лексер и парсер
//! - `plantuml-preprocessor` — препроцессор (!include, !define, etc.)
//! - `plantuml-layout` — алгоритмы размещения элементов
//! - `plantuml-renderer` — SVG/PNG рендеринг
//! - `plantuml-themes` — темы и skinparam

mod error;
mod options;
mod pipeline;

pub use error::{Error, Result};
pub use options::RenderOptions;

// Re-exports для удобства
pub use plantuml_ast::Diagram;
pub use plantuml_parser::parse;
pub use plantuml_themes::Theme;

/// Рендерит PlantUML диаграмму в SVG.
///
/// Это основная функция библиотеки. Она выполняет полный pipeline:
/// 1. Препроцессинг (обработка директив)
/// 2. Парсинг (построение AST)
/// 3. Layout (расчёт позиций элементов)
/// 4. Рендеринг (генерация SVG)
///
/// # Аргументы
///
/// * `source` - исходный код PlantUML
/// * `options` - опции рендеринга
///
/// # Возвращает
///
/// * `Ok(String)` - SVG строка
/// * `Err(Error)` - ошибка на любом этапе
///
/// # Пример
///
/// ```rust
/// use plantuml_core::{render, RenderOptions};
///
/// let source = "@startuml\nAlice -> Bob\n@enduml";
/// let svg = render(source, &RenderOptions::default()).unwrap();
/// assert!(svg.contains("<svg"));
/// ```
pub fn render(source: &str, options: &RenderOptions) -> Result<String> {
    pipeline::render_pipeline(source, options)
}

/// Парсит PlantUML и возвращает AST без рендеринга.
///
/// Полезно для анализа структуры диаграммы или для собственного рендеринга.
///
/// # Пример
///
/// ```rust
/// use plantuml_core::parse_diagram;
///
/// let source = "@startuml\nAlice -> Bob: Hello\n@enduml";
/// let diagram = parse_diagram(source).unwrap();
/// ```
pub fn parse_diagram(source: &str) -> Result<Diagram> {
    // Препроцессинг
    let processed = plantuml_preprocessor::preprocess(source)
        .map_err(|e: plantuml_preprocessor::PreprocessError| Error::Preprocess(e.to_string()))?;
    
    // Парсинг
    let diagram = plantuml_parser::parse(&processed)
        .map_err(|e: plantuml_parser::ParseError| Error::Parse(e.to_string()))?;
    
    Ok(diagram)
}

/// Возвращает список поддерживаемых тем.
pub fn available_themes() -> Vec<&'static str> {
    vec!["default", "minimal", "dark", "sketchy", "cerulean"]
}

/// Информация о версии библиотеки.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_basic_sequence() {
        let source = "@startuml\nAlice -> Bob: Hello\n@enduml";
        let result = render(source, &RenderOptions::default());
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_parse_diagram() {
        let source = "@startuml\nAlice -> Bob\n@enduml";
        let result = parse_diagram(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_available_themes() {
        let themes = available_themes();
        assert!(themes.contains(&"default"));
        assert!(themes.contains(&"dark"));
    }

    #[test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty());
    }
}
