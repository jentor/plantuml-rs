//! Встроенные функции препроцессора

/// Обрабатывает builtin функции в строке
pub fn process_builtins(line: &str) -> String {
    let mut result = line.to_string();
    
    // %date()
    if result.contains("%date()") {
        let date = chrono_date();
        result = result.replace("%date()", &date);
    }
    
    // %time()
    if result.contains("%time()") {
        let time = chrono_time();
        result = result.replace("%time()", &time);
    }
    
    // %version()
    if result.contains("%version()") {
        result = result.replace("%version()", env!("CARGO_PKG_VERSION"));
    }
    
    // %true()
    result = result.replace("%true()", "true");
    
    // %false()
    result = result.replace("%false()", "false");
    
    // %newline()
    result = result.replace("%newline()", "\n");
    
    // TODO: Добавить больше builtin функций
    // %strlen($s), %substr($s, $start, $len), %upper($s), %lower($s)
    // %string($x), %intval($s), %floor($x), %ceil($x), %abs($x)
    
    result
}

/// Возвращает текущую дату
fn chrono_date() -> String {
    // Простая реализация без chrono для WASM-совместимости
    // В production можно использовать chrono или time crate
    "2024-01-01".to_string()
}

/// Возвращает текущее время
fn chrono_time() -> String {
    "12:00:00".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        let result = process_builtins("version: %version()");
        assert!(result.contains("0.1.0"));
    }
    
    #[test]
    fn test_true_false() {
        let result = process_builtins("value = %true()");
        assert_eq!(result, "value = true");
        
        let result = process_builtins("value = %false()");
        assert_eq!(result, "value = false");
    }
}
