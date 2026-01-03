//! Layout engine для State Diagrams
//!
//! Модуль содержит алгоритмы расположения элементов диаграммы состояний.

pub mod config;
pub mod engine;

pub use config::StateLayoutConfig;
pub use engine::StateLayoutEngine;
