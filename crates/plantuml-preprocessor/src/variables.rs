//! Обработка переменных препроцессора

use indexmap::IndexMap;

use crate::{PreprocessContext, PreprocessError, Result};

/// Обрабатывает присваивание переменной: !$var = value
pub fn handle_variable_assignment(directive: &str, ctx: &mut PreprocessContext) -> Result<()> {
    if !ctx.should_output() {
        return Ok(());
    }

    // Формат: $name = value
    let parts: Vec<&str> = directive.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(PreprocessError::SyntaxError(format!(
            "неверный формат присваивания: {}",
            directive
        )));
    }

    let name = parts[0].trim().trim_start_matches('$');
    let value = parts[1].trim().trim_matches('"');

    ctx.set_variable(format!("${}", name), value.to_string());

    Ok(())
}

/// Подставляет переменные в строку
pub fn substitute(line: &str, variables: &IndexMap<String, String>) -> String {
    let mut result = line.to_string();

    for (name, value) in variables {
        // Подстановка $name и ${name}
        result = result.replace(name, value);

        // Также подстановка ${name}
        let braced = format!("${{{}}}", name.trim_start_matches('$'));
        result = result.replace(&braced, value);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_assignment() {
        let mut ctx = PreprocessContext::new();
        handle_variable_assignment("$name = \"Alice\"", &mut ctx).unwrap();
        assert_eq!(ctx.get_variable("$name"), Some(&"Alice".to_string()));
    }

    #[test]
    fn test_substitute() {
        let mut vars = IndexMap::new();
        vars.insert("$name".to_string(), "Alice".to_string());
        vars.insert("$color".to_string(), "#FF0000".to_string());

        let result = substitute("participant $name #$color", &vars);
        assert_eq!(result, "participant Alice ##FF0000");
    }
}
