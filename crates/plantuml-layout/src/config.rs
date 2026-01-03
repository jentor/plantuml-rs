//! Конфигурация layout

/// Конфигурация layout
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    /// Отступ между элементами по горизонтали
    pub horizontal_spacing: f64,
    /// Отступ между элементами по вертикали
    pub vertical_spacing: f64,
    /// Отступ от края диаграммы
    pub margin: f64,
    /// Минимальная ширина узла
    pub min_node_width: f64,
    /// Минимальная высота узла
    pub min_node_height: f64,
    /// Размер шрифта по умолчанию
    pub default_font_size: f64,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            horizontal_spacing: 80.0,
            vertical_spacing: 40.0,
            margin: 20.0,
            min_node_width: 100.0,
            min_node_height: 40.0,
            default_font_size: 14.0,
        }
    }
}

impl LayoutConfig {
    /// Создаёт конфигурацию по умолчанию
    pub fn new() -> Self {
        Self::default()
    }

    /// Конфигурация для sequence diagrams
    pub fn sequence() -> Self {
        Self {
            horizontal_spacing: 100.0,
            vertical_spacing: 30.0,
            margin: 20.0,
            min_node_width: 80.0,
            min_node_height: 40.0,
            default_font_size: 13.0,
        }
    }

    /// Конфигурация для class diagrams
    pub fn class() -> Self {
        Self {
            horizontal_spacing: 120.0,
            vertical_spacing: 60.0,
            margin: 30.0,
            min_node_width: 120.0,
            min_node_height: 60.0,
            default_font_size: 12.0,
        }
    }
}
