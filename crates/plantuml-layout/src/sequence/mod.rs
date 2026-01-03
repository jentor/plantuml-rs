//! Layout engine для Sequence Diagrams.
//!
//! Алгоритм размещения:
//! 1. Участники располагаются горизонтально с равными интервалами
//! 2. Lifelines проводятся вертикально от каждого участника
//! 3. Сообщения располагаются вертикально в порядке появления
//! 4. Фрагменты (alt, opt, loop) рисуются как прямоугольники вокруг содержимого
//! 5. Активации рисуются как узкие прямоугольники на lifeline

mod config;
mod engine;
mod metrics;

pub use config::SequenceLayoutConfig;
pub use engine::SequenceLayoutEngine;
