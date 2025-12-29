//! # plantuml-model
//!
//! Типизированные модели для layout и рендеринга.
//! Преобразование AST в модели, готовые для визуализации.

pub use plantuml_ast as ast;

/// Точка в 2D пространстве
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// Размер в 2D пространстве
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
    
    pub fn zero() -> Self {
        Self { width: 0.0, height: 0.0 }
    }
}

/// Прямоугольник (bounding box)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn from_point_size(point: Point, size: Size) -> Self {
        Self {
            x: point.x,
            y: point.y,
            width: size.width,
            height: size.height,
        }
    }
    
    pub fn center(&self) -> Point {
        Point {
            x: self.x + self.width / 2.0,
            y: self.y + self.height / 2.0,
        }
    }
    
    pub fn top_left(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
    
    pub fn top_right(&self) -> Point {
        Point { x: self.x + self.width, y: self.y }
    }
    
    pub fn bottom_left(&self) -> Point {
        Point { x: self.x, y: self.y + self.height }
    }
    
    pub fn bottom_right(&self) -> Point {
        Point { x: self.x + self.width, y: self.y + self.height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rect_center() {
        let rect = Rect::new(0.0, 0.0, 100.0, 50.0);
        let center = rect.center();
        assert_eq!(center.x, 50.0);
        assert_eq!(center.y, 25.0);
    }
}
