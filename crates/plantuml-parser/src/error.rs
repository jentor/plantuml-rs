//! Ошибки парсера

use thiserror::Error;

/// Ошибки парсинга PlantUML
#[derive(Error, Debug)]
pub enum ParseError {
    /// Неожиданный токен
    #[error("неожиданный токен '{token}' в строке {line}, позиция {column}")]
    UnexpectedToken {
        token: String,
        line: usize,
        column: usize,
    },
    
    /// Синтаксическая ошибка
    #[error("синтаксическая ошибка в строке {line}: {message}")]
    SyntaxError {
        line: usize,
        message: String,
    },
    
    /// Неизвестный тип диаграммы
    #[error("не удалось определить тип диаграммы")]
    UnknownDiagramType,
    
    /// Отсутствует @startuml
    #[error("отсутствует @startuml")]
    MissingStartTag,
    
    /// Отсутствует @enduml
    #[error("отсутствует @enduml")]
    MissingEndTag,
    
    /// Неизвестный участник
    #[error("неизвестный участник: {0}")]
    UnknownParticipant(String),
    
    /// Ошибка лексера
    #[error("ошибка лексера: {0}")]
    LexerError(String),
    
    /// Ошибка грамматики
    #[error("ошибка грамматики: {0}")]
    GrammarError(String),
}
