//! Layout engine для Gantt Diagrams
//!
//! Gantt Diagram отображает задачи на горизонтальной временной шкале.

mod config;
mod engine;

pub use config::GanttLayoutConfig;
pub use engine::GanttLayoutEngine;
