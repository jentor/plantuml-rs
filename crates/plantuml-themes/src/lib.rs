//! # plantuml-themes
//!
//! Темы и skinparam для стилизации диаграмм PlantUML.

use serde::{Deserialize, Serialize};

/// Цвет
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color(String);

impl Color {
    /// Создаёт цвет из строки
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
    
    /// Возвращает CSS представление
    pub fn to_css(&self) -> String {
        // Возвращаем значение as-is: hex (#fff), rgb(...), или именованные цвета
        self.0.clone()
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new("#000000")
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// Тема оформления
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Название темы
    pub name: String,
    
    /// Цвет фона диаграммы
    pub background_color: Color,
    
    /// Цвет фона узлов
    pub node_background: Color,
    
    /// Цвет границы узлов
    pub node_border: Color,
    
    /// Цвет текста
    pub text_color: Color,
    
    /// Цвет стрелок
    pub arrow_color: Color,
    
    /// Семейство шрифтов
    pub font_family: String,
    
    /// Размер шрифта
    pub font_size: f64,
    
    /// Толщина линий
    pub line_width: f64,
    
    /// Радиус скругления углов
    pub corner_radius: f64,
    
    /// Тень
    pub shadow: bool,
    
    /// Рукописный стиль
    pub handwritten: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            background_color: Color::new("#FFFFFF"),
            node_background: Color::new("#FEFECE"),
            node_border: Color::new("#A80036"),
            text_color: Color::new("#000000"),
            arrow_color: Color::new("#A80036"),
            font_family: "Arial, sans-serif".to_string(),
            font_size: 13.0,
            line_width: 1.0,
            corner_radius: 5.0,
            shadow: true,
            handwritten: false,
        }
    }
}

impl Theme {
    /// Тема по умолчанию
    pub fn default_theme() -> Self {
        Self::default()
    }
    
    /// Минималистичная тема
    pub fn minimal() -> Self {
        Self {
            name: "minimal".to_string(),
            background_color: Color::new("#FFFFFF"),
            node_background: Color::new("#FFFFFF"),
            node_border: Color::new("#333333"),
            text_color: Color::new("#333333"),
            arrow_color: Color::new("#333333"),
            font_family: "Helvetica, Arial, sans-serif".to_string(),
            font_size: 12.0,
            line_width: 1.0,
            corner_radius: 0.0,
            shadow: false,
            handwritten: false,
        }
    }
    
    /// Тёмная тема
    pub fn dark() -> Self {
        Self {
            name: "dark".to_string(),
            background_color: Color::new("#1E1E1E"),
            node_background: Color::new("#2D2D2D"),
            node_border: Color::new("#569CD6"),
            text_color: Color::new("#D4D4D4"),
            arrow_color: Color::new("#569CD6"),
            font_family: "Consolas, monospace".to_string(),
            font_size: 13.0,
            line_width: 1.0,
            corner_radius: 3.0,
            shadow: false,
            handwritten: false,
        }
    }
    
    /// Рукописный стиль
    pub fn sketchy() -> Self {
        Self {
            name: "sketchy".to_string(),
            background_color: Color::new("#FFFFF0"),
            node_background: Color::new("#FFFACD"),
            node_border: Color::new("#2F4F4F"),
            text_color: Color::new("#2F4F4F"),
            arrow_color: Color::new("#2F4F4F"),
            font_family: "Comic Sans MS, cursive".to_string(),
            font_size: 14.0,
            line_width: 2.0,
            corner_radius: 8.0,
            shadow: false,
            handwritten: true,
        }
    }
    
    /// Cerulean (голубая)
    pub fn cerulean() -> Self {
        Self {
            name: "cerulean".to_string(),
            background_color: Color::new("#FFFFFF"),
            node_background: Color::new("#E3F2FD"),
            node_border: Color::new("#1976D2"),
            text_color: Color::new("#0D47A1"),
            arrow_color: Color::new("#1976D2"),
            font_family: "Segoe UI, Arial, sans-serif".to_string(),
            font_size: 13.0,
            line_width: 1.5,
            corner_radius: 4.0,
            shadow: true,
            handwritten: false,
        }
    }
    
    /// Загружает тему по имени
    pub fn by_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "default" => Some(Self::default_theme()),
            "minimal" => Some(Self::minimal()),
            "dark" => Some(Self::dark()),
            "sketchy" | "sketchy-outline" => Some(Self::sketchy()),
            "cerulean" => Some(Self::cerulean()),
            _ => None,
        }
    }
}

/// SkinParam параметры
#[derive(Debug, Clone, Default)]
pub struct SkinParams {
    params: std::collections::HashMap<String, String>,
}

impl SkinParams {
    /// Создаёт пустые параметры
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Устанавливает параметр
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.params.insert(key.into(), value.into());
    }
    
    /// Получает параметр
    pub fn get(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }
    
    /// Применяет параметры к теме
    pub fn apply_to(&self, theme: &mut Theme) {
        if let Some(v) = self.get("backgroundColor") {
            theme.background_color = Color::new(v);
        }
        if let Some(v) = self.get("defaultFontName") {
            theme.font_family = v.clone();
        }
        if let Some(v) = self.get("defaultFontSize") {
            if let Ok(size) = v.parse() {
                theme.font_size = size;
            }
        }
        if let Some(v) = self.get("handwritten") {
            theme.handwritten = v == "true";
        }
        if let Some(v) = self.get("shadowing") {
            theme.shadow = v == "true";
        }
        // TODO: Добавить больше параметров
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_theme() {
        let theme = Theme::default();
        assert_eq!(theme.name, "default");
    }
    
    #[test]
    fn test_theme_by_name() {
        assert!(Theme::by_name("dark").is_some());
        assert!(Theme::by_name("unknown").is_none());
    }
    
    #[test]
    fn test_skin_params() {
        let mut params = SkinParams::new();
        params.set("backgroundColor", "#FF0000");
        
        let mut theme = Theme::default();
        params.apply_to(&mut theme);
        
        assert_eq!(theme.background_color.to_css(), "#FF0000");
    }
}
