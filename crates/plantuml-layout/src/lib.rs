//! # plantuml-layout
//!
//! Layout engines для автоматического размещения элементов диаграмм.

pub mod traits;
pub mod config;

pub use traits::{LayoutEngine, LayoutResult};
pub use config::LayoutConfig;
pub use plantuml_model::{Point, Size, Rect};

/// Элемент результата layout
#[derive(Debug, Clone)]
pub struct LayoutElement {
    /// Уникальный ID элемента
    pub id: String,
    /// Bounding box элемента
    pub bounds: Rect,
    /// Тип элемента (для рендеринга)
    pub element_type: ElementType,
}

/// Тип элемента layout
#[derive(Debug, Clone)]
pub enum ElementType {
    /// Прямоугольник (класс, участник, etc.)
    Rectangle {
        label: String,
        corner_radius: f64,
    },
    /// Эллипс (начальное/конечное состояние)
    Ellipse {
        label: Option<String>,
    },
    /// Линия/стрелка
    Edge {
        points: Vec<Point>,
        label: Option<String>,
        arrow_start: bool,
        arrow_end: bool,
    },
    /// Текст
    Text {
        text: String,
        font_size: f64,
    },
    /// Группа (пакет, фрагмент)
    Group {
        label: Option<String>,
        children: Vec<LayoutElement>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_layout_element() {
        let elem = LayoutElement {
            id: "test".to_string(),
            bounds: Rect::new(0.0, 0.0, 100.0, 50.0),
            element_type: ElementType::Rectangle {
                label: "Test".to_string(),
                corner_radius: 5.0,
            },
        };
        
        assert_eq!(elem.id, "test");
    }
}
