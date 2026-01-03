//! Layout engine для Timing Diagrams
//!
//! Timing Diagram отображает временную шкалу горизонтально,
//! с участниками (сигналами) размещёнными вертикально.

mod config;
mod engine;

pub use config::TimingLayoutConfig;
pub use engine::TimingLayoutEngine;
