//! Ошибки препроцессора

use thiserror::Error;

/// Ошибки препроцессора PlantUML
#[derive(Error, Debug)]
pub enum PreprocessError {
    /// Ошибка чтения файла
    #[error("не удалось прочитать файл: {0}")]
    FileReadError(String),
    
    /// !include не поддерживается
    #[error("!include не поддерживается в этом окружении: {0}")]
    IncludeNotSupported(String),
    
    /// Файл не найден
    #[error("файл не найден: {0}")]
    FileNotFound(String),
    
    /// Синтаксическая ошибка
    #[error("синтаксическая ошибка в директиве: {0}")]
    SyntaxError(String),
    
    /// Несбалансированные !ifdef/!endif
    #[error("несбалансированные !ifdef/!endif")]
    UnbalancedCondition,
    
    /// Неизвестная переменная
    #[error("неизвестная переменная: {0}")]
    UnknownVariable(String),
    
    /// Рекурсивное включение
    #[error("рекурсивное включение файла: {0}")]
    RecursiveInclude(String),
    
    /// Ошибка вычисления выражения
    #[error("ошибка вычисления выражения: {0}")]
    ExpressionError(String),
}
