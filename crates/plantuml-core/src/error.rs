//! Типы ошибок для plantuml-core

use thiserror::Error;

/// Тип результата для plantuml-core
pub type Result<T> = std::result::Result<T, Error>;

/// Ошибки библиотеки plantuml-core
#[derive(Error, Debug)]
pub enum Error {
    /// Ошибка препроцессора
    #[error("ошибка препроцессора: {0}")]
    Preprocess(String),

    /// Ошибка парсинга
    #[error("ошибка парсинга: {0}")]
    Parse(String),

    /// Ошибка layout
    #[error("ошибка layout: {0}")]
    Layout(String),

    /// Ошибка рендеринга
    #[error("ошибка рендеринга: {0}")]
    Render(String),

    /// Неподдерживаемый тип диаграммы
    #[error("неподдерживаемый тип диаграммы: {0}")]
    UnsupportedDiagram(String),

    /// Неизвестная тема
    #[error("неизвестная тема: {0}")]
    UnknownTheme(String),

    /// Пустой исходный код
    #[error("пустой исходный код")]
    EmptySource,
}
