//! # plantuml-preprocessor
//!
//! Препроцессор PlantUML для обработки директив:
//! - `!include` / `!include_once`
//! - `!define` / `!undef`
//! - `!ifdef` / `!ifndef` / `!else` / `!endif`
//! - `!$variable = value`
//! - `!function` / `!procedure`
//! - `!theme`
//! - `%date()`, `%version()` и другие builtin функции

mod error;
mod directives;
mod variables;
mod builtins;

pub use error::PreprocessError;

use indexmap::IndexMap;

/// Обрабатывает PlantUML исходный код (без поддержки !include)
///
/// Это удобная обёртка над `Preprocessor::new().process(source)`.
///
/// # Пример
///
/// ```rust
/// use plantuml_preprocessor::preprocess;
///
/// let source = "!define DEBUG\n!ifdef DEBUG\nDebug mode\n!endif";
/// let result = preprocess(source).unwrap();
/// assert!(result.contains("Debug mode"));
/// ```
pub fn preprocess(source: &str) -> Result<String> {
    Preprocessor::new().process(source)
}

/// Результат препроцессинга
pub type Result<T> = std::result::Result<T, PreprocessError>;

/// Трейт для разрешения путей файлов
pub trait FileResolver {
    /// Читает содержимое файла по пути
    fn read_file(&self, path: &str) -> Result<String>;
    
    /// Проверяет существование файла
    fn file_exists(&self, path: &str) -> bool;
}

/// Заглушка для FileResolver (не поддерживает !include)
#[derive(Debug, Default)]
pub struct NoopFileResolver;

impl FileResolver for NoopFileResolver {
    fn read_file(&self, path: &str) -> Result<String> {
        Err(PreprocessError::IncludeNotSupported(path.to_string()))
    }
    
    fn file_exists(&self, _path: &str) -> bool {
        false
    }
}

/// Контекст препроцессора
#[derive(Debug)]
pub struct PreprocessContext {
    /// Переменные
    pub variables: IndexMap<String, String>,
    /// Уже включённые файлы (для !include_once)
    pub included_files: Vec<String>,
    /// Текущий уровень вложенности условий
    pub condition_depth: usize,
    /// Активные условия (true = выполнять код)
    pub condition_stack: Vec<bool>,
}

impl Default for PreprocessContext {
    fn default() -> Self {
        Self {
            variables: IndexMap::new(),
            included_files: Vec::new(),
            condition_depth: 0,
            condition_stack: Vec::new(),
        }
    }
}

impl PreprocessContext {
    /// Создаёт новый контекст
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Устанавливает переменную
    pub fn set_variable(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(name.into(), value.into());
    }
    
    /// Получает значение переменной
    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }
    
    /// Проверяет, определена ли переменная
    pub fn is_defined(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
    
    /// Проверяет, нужно ли выполнять текущий код
    pub fn should_output(&self) -> bool {
        self.condition_stack.iter().all(|&b| b)
    }
}

/// Препроцессор PlantUML
pub struct Preprocessor<R: FileResolver = NoopFileResolver> {
    resolver: R,
}

impl Preprocessor<NoopFileResolver> {
    /// Создаёт препроцессор без поддержки !include
    pub fn new() -> Self {
        Self {
            resolver: NoopFileResolver,
        }
    }
}

impl Default for Preprocessor<NoopFileResolver> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: FileResolver> Preprocessor<R> {
    /// Создаёт препроцессор с заданным resolver'ом
    pub fn with_resolver(resolver: R) -> Self {
        Self { resolver }
    }
    
    /// Обрабатывает исходный код PlantUML
    pub fn process(&self, source: &str) -> Result<String> {
        let mut ctx = PreprocessContext::new();
        self.process_with_context(source, &mut ctx)
    }
    
    /// Обрабатывает исходный код с заданным контекстом
    pub fn process_with_context(&self, source: &str, ctx: &mut PreprocessContext) -> Result<String> {
        let mut output = String::new();
        
        for line in source.lines() {
            let trimmed = line.trim();
            
            // Обработка директив препроцессора
            if trimmed.starts_with('!') {
                self.process_directive(trimmed, ctx)?;
                continue;
            }
            
            // Пропускаем строки, если мы внутри ложного условия
            if !ctx.should_output() {
                continue;
            }
            
            // Подстановка переменных
            let processed = self.substitute_variables(line, ctx);
            
            // Обработка builtin функций
            let processed = builtins::process_builtins(&processed);
            
            output.push_str(&processed);
            output.push('\n');
        }
        
        Ok(output)
    }
    
    /// Обрабатывает директиву препроцессора
    fn process_directive(&self, line: &str, ctx: &mut PreprocessContext) -> Result<()> {
        let directive = &line[1..]; // Убираем '!'
        
        if let Some(rest) = directive.strip_prefix("define ") {
            directives::handle_define(rest, ctx)?;
        } else if let Some(rest) = directive.strip_prefix("undef ") {
            directives::handle_undef(rest.trim(), ctx);
        } else if let Some(rest) = directive.strip_prefix("ifdef ") {
            directives::handle_ifdef(rest.trim(), ctx, true);
        } else if let Some(rest) = directive.strip_prefix("ifndef ") {
            directives::handle_ifdef(rest.trim(), ctx, false);
        } else if directive == "else" {
            directives::handle_else(ctx)?;
        } else if directive == "endif" {
            directives::handle_endif(ctx)?;
        } else if let Some(rest) = directive.strip_prefix("include ") {
            self.handle_include(rest.trim(), ctx, false)?;
        } else if let Some(rest) = directive.strip_prefix("include_once ") {
            self.handle_include(rest.trim(), ctx, true)?;
        } else if directive.starts_with('$') {
            // Переменная: !$var = value
            variables::handle_variable_assignment(directive, ctx)?;
        }
        // TODO: !function, !procedure, !theme
        
        Ok(())
    }
    
    /// Обрабатывает !include
    fn handle_include(&self, path: &str, ctx: &mut PreprocessContext, once: bool) -> Result<()> {
        if !ctx.should_output() {
            return Ok(());
        }
        
        let path = path.trim_matches(|c| c == '<' || c == '>' || c == '"');
        
        if once && ctx.included_files.contains(&path.to_string()) {
            return Ok(());
        }
        
        let content = self.resolver.read_file(path)?;
        ctx.included_files.push(path.to_string());
        
        // Рекурсивная обработка включённого файла
        self.process_with_context(&content, ctx)?;
        
        Ok(())
    }
    
    /// Подставляет переменные в строку
    fn substitute_variables(&self, line: &str, ctx: &PreprocessContext) -> String {
        variables::substitute(line, &ctx.variables)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_preprocess() {
        let preprocessor = Preprocessor::new();
        let source = "@startuml\nAlice -> Bob\n@enduml";
        let result = preprocessor.process(source).unwrap();
        assert!(result.contains("Alice -> Bob"));
    }
    
    #[test]
    fn test_variable_substitution() {
        let preprocessor = Preprocessor::new();
        let source = "!$name = \"Alice\"\nparticipant $name";
        let result = preprocessor.process(source).unwrap();
        assert!(result.contains("participant Alice"));
    }
    
    #[test]
    fn test_ifdef() {
        let preprocessor = Preprocessor::new();
        let source = r#"
!define DEBUG
!ifdef DEBUG
debug message
!endif
"#;
        let result = preprocessor.process(source).unwrap();
        assert!(result.contains("debug message"));
    }
    
    #[test]
    fn test_ifndef() {
        let preprocessor = Preprocessor::new();
        let source = r#"
!ifndef RELEASE
debug mode
!endif
"#;
        let result = preprocessor.process(source).unwrap();
        assert!(result.contains("debug mode"));
    }
}
