//! Pipeline рендеринга диаграмм

use crate::{Error, RenderOptions, Result};
use plantuml_ast::Diagram;
use plantuml_layout::{LayoutResult, LayoutConfig, LayoutElement, ElementType, Rect};
use plantuml_renderer::{SvgRenderer, Renderer};

/// Выполняет полный pipeline рендеринга
pub fn render_pipeline(source: &str, options: &RenderOptions) -> Result<String> {
    // Проверка на пустой исходник
    let source = source.trim();
    if source.is_empty() {
        return Err(Error::EmptySource);
    }
    
    // 1. Препроцессинг
    let processed = preprocess(source)?;
    
    // 2. Парсинг
    let diagram = parse(&processed)?;
    
    // 3. Layout
    let layout = layout(&diagram, options)?;
    
    // 4. Рендеринг
    let svg = render_svg(&layout, options)?;
    
    Ok(svg)
}

/// Этап препроцессинга
fn preprocess(source: &str) -> Result<String> {
    plantuml_preprocessor::preprocess(source)
        .map_err(|e: plantuml_preprocessor::PreprocessError| Error::Preprocess(e.to_string()))
}

/// Этап парсинга
fn parse(source: &str) -> Result<Diagram> {
    plantuml_parser::parse(source)
        .map_err(|e: plantuml_parser::ParseError| Error::Parse(e.to_string()))
}

/// Этап layout
fn layout(diagram: &Diagram, _options: &RenderOptions) -> Result<LayoutResult> {
    let _config = LayoutConfig::default();
    
    // Выбираем layout engine в зависимости от типа диаграммы
    match diagram {
        Diagram::Sequence(seq) => {
            // TODO: Реализовать SequenceLayoutEngine
            // Пока возвращаем заглушку
            Ok(create_placeholder_layout(seq))
        }
        Diagram::Class(class) => {
            // TODO: Реализовать ClassLayoutEngine (Sugiyama)
            Ok(create_placeholder_layout(class))
        }
        Diagram::Activity(act) => {
            Ok(create_placeholder_layout(act))
        }
        Diagram::State(state) => {
            Ok(create_placeholder_layout(state))
        }
        Diagram::Component(comp) => {
            Ok(create_placeholder_layout(comp))
        }
        Diagram::UseCase(uc) => {
            Ok(create_placeholder_layout(uc))
        }
        Diagram::Deployment(dep) => {
            Ok(create_placeholder_layout(dep))
        }
    }
}

/// Создаёт placeholder layout для ещё не реализованных типов диаграмм
fn create_placeholder_layout<T>(_diagram: &T) -> LayoutResult {
    // Создаём простой layout с текстом-заглушкой
    let mut result = LayoutResult {
        elements: vec![
            LayoutElement {
                id: "placeholder".to_string(),
                bounds: Rect::new(10.0, 10.0, 180.0, 80.0),
                element_type: ElementType::Rectangle {
                    label: "Diagram (TODO)".to_string(),
                    corner_radius: 5.0,
                },
            },
        ],
        bounds: Rect::new(0.0, 0.0, 200.0, 100.0),
    };
    result.calculate_bounds();
    result
}

/// Этап SVG рендеринга
fn render_svg(layout: &LayoutResult, options: &RenderOptions) -> Result<String> {
    let render_options = plantuml_renderer::RenderOptions {
        xml_header: options.xml_header,
        scale: options.scale,
        background_color: options.background_color.clone()
            .or_else(|| Some(options.theme.background_color.to_css())),
    };
    
    let renderer = SvgRenderer::with_options(render_options);
    
    Ok(renderer.render(layout, &options.theme))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_basic() {
        let source = "@startuml\nAlice -> Bob\n@enduml";
        let result = render_pipeline(source, &RenderOptions::default());
        assert!(result.is_ok());
    }

    #[test]
    fn test_pipeline_empty_source() {
        let result = render_pipeline("", &RenderOptions::default());
        assert!(matches!(result, Err(Error::EmptySource)));
    }

    #[test]
    fn test_pipeline_whitespace_only() {
        let result = render_pipeline("   \n  \t  ", &RenderOptions::default());
        assert!(matches!(result, Err(Error::EmptySource)));
    }
}
