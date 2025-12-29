//! # plantuml-renderer
//!
//! SVG и PNG рендеринг диаграмм PlantUML.

pub mod svg_renderer;
pub mod shapes;

pub use svg_renderer::SvgRenderer;
pub use plantuml_layout::{LayoutResult, LayoutElement, ElementType, Point, Rect};
pub use plantuml_themes::Theme;

/// Трейт для рендереров
pub trait Renderer {
    /// Тип выходных данных
    type Output;
    
    /// Рендерит результат layout
    fn render(&self, layout: &LayoutResult, theme: &Theme) -> Self::Output;
}

/// Опции рендеринга
#[derive(Debug, Clone)]
pub struct RenderOptions {
    /// Добавить XML заголовок
    pub xml_header: bool,
    /// Масштаб
    pub scale: f64,
    /// Цвет фона (None = прозрачный)
    pub background_color: Option<String>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            xml_header: true,
            scale: 1.0,
            background_color: Some("#FFFFFF".to_string()),
        }
    }
}
