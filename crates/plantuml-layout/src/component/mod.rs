//! Layout engine для Component Diagrams
//!
//! Модуль содержит алгоритмы расположения элементов диаграммы компонентов.

pub mod config;
pub mod engine;

pub use config::ComponentLayoutConfig;
pub use engine::ComponentLayoutEngine;
