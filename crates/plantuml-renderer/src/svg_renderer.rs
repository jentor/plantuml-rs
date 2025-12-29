//! SVG рендерер

use svg::node::element::{Group, Rectangle, Path, Definitions, Marker};
use svg::Document;

use crate::{LayoutResult, LayoutElement, ElementType, Point, Rect, Renderer, RenderOptions};
use plantuml_themes::Theme;

/// SVG рендерер
pub struct SvgRenderer {
    options: RenderOptions,
}

impl SvgRenderer {
    /// Создаёт новый рендерер
    pub fn new() -> Self {
        Self {
            options: RenderOptions::default(),
        }
    }
    
    /// Создаёт рендерер с опциями
    pub fn with_options(options: RenderOptions) -> Self {
        Self { options }
    }
    
    /// Рендерит в строку
    pub fn render_to_string(&self, layout: &LayoutResult, theme: &Theme) -> String {
        self.render(layout, theme)
    }
    
    /// Создаёт SVG документ
    fn create_document(&self, layout: &LayoutResult, theme: &Theme) -> Document {
        let bounds = &layout.bounds;
        let margin = 20.0;
        
        let width = (bounds.width + margin * 2.0) * self.options.scale;
        let height = (bounds.height + margin * 2.0) * self.options.scale;
        
        let mut doc = Document::new()
            .set("width", width)
            .set("height", height)
            .set("viewBox", (
                bounds.x - margin,
                bounds.y - margin,
                bounds.width + margin * 2.0,
                bounds.height + margin * 2.0,
            ))
            .set("xmlns", "http://www.w3.org/2000/svg");
        
        // Фон
        if let Some(bg) = &self.options.background_color {
            let bg_rect = Rectangle::new()
                .set("x", bounds.x - margin)
                .set("y", bounds.y - margin)
                .set("width", bounds.width + margin * 2.0)
                .set("height", bounds.height + margin * 2.0)
                .set("fill", bg.as_str());
            doc = doc.add(bg_rect);
        }
        
        // Определения (маркеры стрелок)
        let defs = self.create_definitions(theme);
        doc = doc.add(defs);
        
        doc
    }
    
    /// Создаёт определения (маркеры, градиенты)
    fn create_definitions(&self, theme: &Theme) -> Definitions {
        let arrow_color = theme.arrow_color.to_css();
        
        // Маркер стрелки
        let arrow_marker = Marker::new()
            .set("id", "arrow")
            .set("markerWidth", 10)
            .set("markerHeight", 10)
            .set("refX", 9)
            .set("refY", 3)
            .set("orient", "auto")
            .set("markerUnits", "strokeWidth")
            .add(
                Path::new()
                    .set("d", "M0,0 L0,6 L9,3 z")
                    .set("fill", arrow_color.as_str())
            );
        
        // Открытый маркер стрелки (для sequence diagrams)
        let open_arrow_marker = Marker::new()
            .set("id", "arrow-open")
            .set("markerWidth", 10)
            .set("markerHeight", 10)
            .set("refX", 9)
            .set("refY", 3)
            .set("orient", "auto")
            .set("markerUnits", "strokeWidth")
            .add(
                Path::new()
                    .set("d", "M0,0 L9,3 L0,6")
                    .set("fill", "none")
                    .set("stroke", arrow_color.as_str())
                    .set("stroke-width", 1)
            );
        
        Definitions::new()
            .add(arrow_marker)
            .add(open_arrow_marker)
    }
    
    /// Рендерит элемент
    fn render_element(&self, element: &LayoutElement, theme: &Theme) -> Group {
        let mut group = Group::new().set("id", element.id.as_str());
        
        match &element.element_type {
            ElementType::Rectangle { label, corner_radius } => {
                group = self.render_rectangle(&element.bounds, label, *corner_radius, theme, group);
            }
            ElementType::Ellipse { label } => {
                group = self.render_ellipse(&element.bounds, label.as_deref(), theme, group);
            }
            ElementType::Edge { points, label, arrow_start, arrow_end } => {
                group = self.render_edge(points, label.as_deref(), *arrow_start, *arrow_end, theme, group);
            }
            ElementType::Text { text, font_size } => {
                group = self.render_text(&element.bounds, text, *font_size, theme, group);
            }
            ElementType::Group { label, children } => {
                group = self.render_group(&element.bounds, label.as_deref(), children, theme, group);
            }
        }
        
        group
    }
    
    /// Рендерит прямоугольник
    fn render_rectangle(
        &self,
        bounds: &Rect,
        label: &str,
        corner_radius: f64,
        theme: &Theme,
        mut group: Group,
    ) -> Group {
        let rect = Rectangle::new()
            .set("x", bounds.x)
            .set("y", bounds.y)
            .set("width", bounds.width)
            .set("height", bounds.height)
            .set("rx", corner_radius)
            .set("ry", corner_radius)
            .set("fill", theme.node_background.to_css())
            .set("stroke", theme.node_border.to_css())
            .set("stroke-width", 1);
        
        group = group.add(rect);
        
        // Текст по центру
        let text = svg::node::element::Text::new(label)
            .set("x", bounds.x + bounds.width / 2.0)
            .set("y", bounds.y + bounds.height / 2.0)
            .set("text-anchor", "middle")
            .set("dominant-baseline", "middle")
            .set("font-family", theme.font_family.as_str())
            .set("font-size", theme.font_size)
            .set("fill", theme.text_color.to_css());
        
        group.add(text)
    }
    
    /// Рендерит эллипс
    fn render_ellipse(
        &self,
        bounds: &Rect,
        label: Option<&str>,
        theme: &Theme,
        mut group: Group,
    ) -> Group {
        let cx = bounds.x + bounds.width / 2.0;
        let cy = bounds.y + bounds.height / 2.0;
        let rx = bounds.width / 2.0;
        let ry = bounds.height / 2.0;
        
        let ellipse = svg::node::element::Ellipse::new()
            .set("cx", cx)
            .set("cy", cy)
            .set("rx", rx)
            .set("ry", ry)
            .set("fill", theme.node_background.to_css())
            .set("stroke", theme.node_border.to_css())
            .set("stroke-width", 1);
        
        group = group.add(ellipse);
        
        if let Some(label) = label {
            let text = svg::node::element::Text::new(label)
                .set("x", cx)
                .set("y", cy)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle")
                .set("font-family", theme.font_family.as_str())
                .set("font-size", theme.font_size)
                .set("fill", theme.text_color.to_css());
            
            group = group.add(text);
        }
        
        group
    }
    
    /// Рендерит линию/стрелку
    fn render_edge(
        &self,
        points: &[Point],
        label: Option<&str>,
        arrow_start: bool,
        arrow_end: bool,
        theme: &Theme,
        mut group: Group,
    ) -> Group {
        if points.len() < 2 {
            return group;
        }
        
        // Строим путь
        let mut d = format!("M{},{}", points[0].x, points[0].y);
        for p in &points[1..] {
            d.push_str(&format!(" L{},{}", p.x, p.y));
        }
        
        let mut path = Path::new()
            .set("d", d)
            .set("fill", "none")
            .set("stroke", theme.arrow_color.to_css())
            .set("stroke-width", 1);
        
        if arrow_end {
            path = path.set("marker-end", "url(#arrow)");
        }
        if arrow_start {
            path = path.set("marker-start", "url(#arrow)");
        }
        
        group = group.add(path);
        
        // Метка посередине
        if let Some(label) = label {
            let mid_idx = points.len() / 2;
            let mid = if points.len() % 2 == 0 && mid_idx > 0 {
                Point::new(
                    (points[mid_idx - 1].x + points[mid_idx].x) / 2.0,
                    (points[mid_idx - 1].y + points[mid_idx].y) / 2.0,
                )
            } else {
                points[mid_idx]
            };
            
            // Белый фон для текста
            let text_bg = Rectangle::new()
                .set("x", mid.x - 30.0)
                .set("y", mid.y - 8.0)
                .set("width", 60.0)
                .set("height", 16.0)
                .set("fill", "white");
            
            let text = svg::node::element::Text::new(label)
                .set("x", mid.x)
                .set("y", mid.y)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle")
                .set("font-family", theme.font_family.as_str())
                .set("font-size", theme.font_size - 1.0)
                .set("fill", theme.text_color.to_css());
            
            group = group.add(text_bg).add(text);
        }
        
        group
    }
    
    /// Рендерит текст
    fn render_text(
        &self,
        bounds: &Rect,
        text_content: &str,
        font_size: f64,
        theme: &Theme,
        group: Group,
    ) -> Group {
        let text = svg::node::element::Text::new(text_content)
            .set("x", bounds.x)
            .set("y", bounds.y + font_size)
            .set("font-family", theme.font_family.as_str())
            .set("font-size", font_size)
            .set("fill", theme.text_color.to_css());
        
        group.add(text)
    }
    
    /// Рендерит группу
    fn render_group(
        &self,
        bounds: &Rect,
        label: Option<&str>,
        children: &[LayoutElement],
        theme: &Theme,
        mut group: Group,
    ) -> Group {
        // Рамка группы
        let rect = Rectangle::new()
            .set("x", bounds.x)
            .set("y", bounds.y)
            .set("width", bounds.width)
            .set("height", bounds.height)
            .set("fill", "none")
            .set("stroke", theme.node_border.to_css())
            .set("stroke-width", 1)
            .set("stroke-dasharray", "5,3");
        
        group = group.add(rect);
        
        // Заголовок
        if let Some(label) = label {
            let header_bg = Rectangle::new()
                .set("x", bounds.x)
                .set("y", bounds.y)
                .set("width", bounds.width)
                .set("height", 20.0)
                .set("fill", theme.node_background.to_css());
            
            let text = svg::node::element::Text::new(label)
                .set("x", bounds.x + 5.0)
                .set("y", bounds.y + 14.0)
                .set("font-family", theme.font_family.as_str())
                .set("font-size", theme.font_size)
                .set("font-weight", "bold")
                .set("fill", theme.text_color.to_css());
            
            group = group.add(header_bg).add(text);
        }
        
        // Дочерние элементы
        for child in children {
            group = group.add(self.render_element(child, theme));
        }
        
        group
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for SvgRenderer {
    type Output = String;
    
    fn render(&self, layout: &LayoutResult, theme: &Theme) -> String {
        let mut doc = self.create_document(layout, theme);
        
        for element in &layout.elements {
            let rendered = self.render_element(element, theme);
            doc = doc.add(rendered);
        }
        
        if self.options.xml_header {
            format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", doc)
        } else {
            doc.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plantuml_layout::ElementType;
    
    #[test]
    fn test_render_empty() {
        let renderer = SvgRenderer::new();
        let layout = LayoutResult::empty();
        let theme = Theme::default();
        
        let svg = renderer.render(&layout, &theme);
        assert!(svg.contains("<?xml"));
        assert!(svg.contains("<svg"));
    }
    
    #[test]
    fn test_render_rectangle() {
        let renderer = SvgRenderer::new();
        let layout = LayoutResult {
            elements: vec![
                LayoutElement {
                    id: "test".to_string(),
                    bounds: Rect::new(10.0, 10.0, 100.0, 50.0),
                    element_type: ElementType::Rectangle {
                        label: "Hello".to_string(),
                        corner_radius: 5.0,
                    },
                },
            ],
            bounds: Rect::new(0.0, 0.0, 120.0, 70.0),
        };
        let theme = Theme::default();
        
        let svg = renderer.render(&layout, &theme);
        assert!(svg.contains("<rect"));
        assert!(svg.contains("Hello"));
    }
}
