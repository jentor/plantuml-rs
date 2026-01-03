//! Pipeline рендеринга диаграмм

use std::path::Path;

use crate::{Error, RenderOptions, Result};
use plantuml_ast::Diagram;
use plantuml_layout::{
    ActivityLayoutEngine, ClassLayoutEngine, ComponentLayoutEngine, ErLayoutEngine,
    GanttLayoutEngine, JsonLayoutEngine, LayoutConfig, LayoutResult, MindMapLayoutEngine,
    NetworkLayoutEngine, ObjectLayoutEngine, SaltLayoutEngine, SequenceLayoutEngine,
    StateLayoutEngine, TimingLayoutEngine, UseCaseLayoutEngine, WbsLayoutEngine, YamlLayoutEngine,
};
use plantuml_preprocessor::{FsFileResolver, Preprocessor};
use plantuml_renderer::{Renderer, SvgRenderer};

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

/// Выполняет полный pipeline с поддержкой !include
pub fn render_pipeline_with_includes(
    source: &str,
    base_path: &Path,
    options: &RenderOptions,
) -> Result<String> {
    // Проверка на пустой исходник
    let source = source.trim();
    if source.is_empty() {
        return Err(Error::EmptySource);
    }

    // 1. Препроцессинг с поддержкой файлов
    let processed = preprocess_with_includes(source, base_path)?;

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

/// Этап препроцессинга с поддержкой !include
fn preprocess_with_includes(source: &str, base_path: &Path) -> Result<String> {
    let resolver = FsFileResolver::new(base_path);
    let preprocessor = Preprocessor::with_resolver(resolver);
    preprocessor
        .process(source)
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
            // Используем SequenceLayoutEngine для sequence diagrams
            let engine = SequenceLayoutEngine::new();
            Ok(engine.layout(seq))
        }
        Diagram::Class(class) => {
            // Используем ClassLayoutEngine для class diagrams (Sugiyama algorithm)
            let engine = ClassLayoutEngine::new();
            Ok(engine.layout_diagram(class))
        }
        Diagram::Activity(act) => {
            // Используем ActivityLayoutEngine для activity diagrams
            let engine = ActivityLayoutEngine::new();
            Ok(engine.layout(act))
        }
        Diagram::State(state) => {
            // Используем StateLayoutEngine для state diagrams
            let engine = StateLayoutEngine::new();
            Ok(engine.layout(state))
        }
        Diagram::Component(comp) => {
            // Используем ComponentLayoutEngine для component diagrams
            let engine = ComponentLayoutEngine::new();
            Ok(engine.layout(comp))
        }
        Diagram::UseCase(uc) => {
            // Используем UseCaseLayoutEngine для use case diagrams
            let engine = UseCaseLayoutEngine::new();
            Ok(engine.layout(uc))
        }
        Diagram::Deployment(dep) => {
            // Deployment использует ComponentLayoutEngine (та же структура)
            let engine = ComponentLayoutEngine::new();
            Ok(engine.layout(dep))
        }
        Diagram::Object(obj) => {
            // Используем ObjectLayoutEngine для object diagrams
            let engine = ObjectLayoutEngine::new();
            Ok(engine.layout(obj))
        }
        Diagram::Timing(timing) => {
            // Используем TimingLayoutEngine для timing diagrams
            let engine = TimingLayoutEngine::new();
            Ok(engine.layout(timing))
        }
        Diagram::Gantt(gantt) => {
            // Используем GanttLayoutEngine для gantt diagrams
            let engine = GanttLayoutEngine::new();
            Ok(engine.layout(gantt))
        }
        Diagram::MindMap(mindmap) => {
            // Используем MindMapLayoutEngine для mindmap diagrams
            let engine = MindMapLayoutEngine::new();
            Ok(engine.layout(mindmap))
        }
        Diagram::Wbs(wbs) => {
            // Используем WbsLayoutEngine для wbs diagrams
            let engine = WbsLayoutEngine::new();
            Ok(engine.layout(wbs))
        }
        Diagram::Json(json) => {
            // Используем JsonLayoutEngine для json diagrams
            use plantuml_layout::traits::LayoutEngine as _;
            let engine = JsonLayoutEngine::new();
            Ok(engine.layout(json, &_config))
        }
        Diagram::Yaml(yaml) => {
            // Используем YamlLayoutEngine для yaml diagrams
            use plantuml_layout::traits::LayoutEngine as _;
            let engine = YamlLayoutEngine::new();
            Ok(engine.layout(yaml, &_config))
        }
        Diagram::Er(er) => {
            // Используем ErLayoutEngine для ER diagrams
            use plantuml_layout::traits::LayoutEngine as _;
            let engine = ErLayoutEngine::new();
            Ok(engine.layout(er, &_config))
        }
        Diagram::Network(net) => {
            // Используем NetworkLayoutEngine для network diagrams
            use plantuml_layout::traits::LayoutEngine as _;
            let engine = NetworkLayoutEngine::new();
            Ok(engine.layout(net, &_config))
        }
        Diagram::Salt(salt) => {
            // Используем SaltLayoutEngine для salt diagrams
            use plantuml_layout::traits::LayoutEngine as _;
            let engine = SaltLayoutEngine::new();
            Ok(engine.layout(salt, &_config))
        }
        Diagram::Archimate(arch) => {
            // Archimate использует ComponentLayoutEngine
            let engine = ComponentLayoutEngine::new();
            Ok(engine.layout(arch))
        }
    }
}

/// Этап SVG рендеринга
fn render_svg(layout: &LayoutResult, options: &RenderOptions) -> Result<String> {
    let render_options = plantuml_renderer::RenderOptions {
        xml_header: options.xml_header,
        scale: options.scale,
        // None означает использовать PlantUML default (#FEFECE)
        background_color: options.background_color.clone(),
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

    #[test]
    fn test_pipeline_box_sequence() {
        let source = r#"@startuml
box "Фронтенд" #LightBlue
    participant "React App" as React
    participant "Redux Store" as Redux
end box

box "Бэкенд" #LightGreen
    participant "API Gateway" as API
    participant "Auth Service" as Auth
    participant "User Service" as User
end box

React -> Redux: dispatch(login)
Redux -> API: POST /auth/login
API -> Auth: validateCredentials
Auth -> User: getUserById
User --> Auth: user data
Auth --> API: JWT token
API --> Redux: { token, user }
Redux --> React: state updated
@enduml"#;
        let result = render_pipeline(source, &RenderOptions::default());
        assert!(result.is_ok(), "Pipeline error: {:?}", result.err());
    }
}
