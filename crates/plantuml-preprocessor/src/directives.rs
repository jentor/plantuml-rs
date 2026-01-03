//! Обработка директив препроцессора

use crate::{PreprocessContext, PreprocessError, Result};

/// Обрабатывает !define
pub fn handle_define(rest: &str, ctx: &mut PreprocessContext) -> Result<()> {
    if !ctx.should_output() {
        return Ok(());
    }

    let rest = rest.trim();

    // Простой define: !define NAME или !define NAME value
    if let Some((name, value)) = rest.split_once(' ') {
        ctx.set_variable(name.trim().to_string(), value.trim().to_string());
    } else {
        // Просто определяем как пустую строку
        ctx.set_variable(rest.to_string(), String::new());
    }

    Ok(())
}

/// Обрабатывает !undef
pub fn handle_undef(name: &str, ctx: &mut PreprocessContext) {
    if ctx.should_output() {
        ctx.variables.shift_remove(name);
    }
}

/// Обрабатывает !ifdef / !ifndef
pub fn handle_ifdef(name: &str, ctx: &mut PreprocessContext, is_ifdef: bool) {
    let defined = ctx.is_defined(name);
    let condition = if is_ifdef { defined } else { !defined };

    // Если мы уже внутри ложного условия, вложенное условие тоже ложно
    let effective = ctx.should_output() && condition;

    ctx.condition_stack.push(effective);
    ctx.condition_depth += 1;
}

/// Обрабатывает !else
pub fn handle_else(ctx: &mut PreprocessContext) -> Result<()> {
    if ctx.condition_stack.is_empty() {
        return Err(PreprocessError::UnbalancedCondition);
    }

    // Проверяем родительское условие до мутации
    let len = ctx.condition_stack.len();
    let parent_ok = len <= 1 || ctx.condition_stack[..len - 1].iter().all(|&b| b);

    // Инвертируем текущее условие если родитель true
    if parent_ok {
        if let Some(last) = ctx.condition_stack.last_mut() {
            *last = !*last;
        }
    }

    Ok(())
}

/// Обрабатывает !endif
pub fn handle_endif(ctx: &mut PreprocessContext) -> Result<()> {
    if ctx.condition_stack.is_empty() {
        return Err(PreprocessError::UnbalancedCondition);
    }

    ctx.condition_stack.pop();
    ctx.condition_depth = ctx.condition_depth.saturating_sub(1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define() {
        let mut ctx = PreprocessContext::new();
        handle_define("DEBUG true", &mut ctx).unwrap();
        assert_eq!(ctx.get_variable("DEBUG"), Some(&"true".to_string()));
    }

    #[test]
    fn test_ifdef_defined() {
        let mut ctx = PreprocessContext::new();
        ctx.set_variable("DEBUG", "");

        handle_ifdef("DEBUG", &mut ctx, true);
        assert!(ctx.should_output());

        handle_endif(&mut ctx).unwrap();
        assert!(ctx.should_output());
    }

    #[test]
    fn test_ifdef_not_defined() {
        let mut ctx = PreprocessContext::new();

        handle_ifdef("DEBUG", &mut ctx, true);
        assert!(!ctx.should_output());

        handle_endif(&mut ctx).unwrap();
        assert!(ctx.should_output());
    }

    #[test]
    fn test_else() {
        let mut ctx = PreprocessContext::new();

        handle_ifdef("DEBUG", &mut ctx, true); // false, DEBUG не определён
        assert!(!ctx.should_output());

        handle_else(&mut ctx).unwrap();
        assert!(ctx.should_output()); // Теперь true

        handle_endif(&mut ctx).unwrap();
    }
}
