//! Layout engine для Object Diagrams
//!
//! Object Diagram использует тот же layout алгоритм что и Class Diagram,
//! но с конвертацией ObjectDiagram → ClassDiagram.

mod config;
mod engine;

pub use config::ObjectLayoutConfig;
pub use engine::ObjectLayoutEngine;
