//! Layout engine для YAML диаграмм
//!
//! Переиспользует JSON layout engine, так как структура данных идентична.

pub mod config;
pub mod engine;

pub use config::YamlLayoutConfig;
pub use engine::YamlLayoutEngine;
