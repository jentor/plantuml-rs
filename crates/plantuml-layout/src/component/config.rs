//! Конфигурация layout для Component Diagrams

/// Конфигурация Component Layout Engine
#[derive(Debug, Clone)]
pub struct ComponentLayoutConfig {
    /// Отступ от края диаграммы
    pub margin: f64,
    /// Ширина компонента
    pub component_width: f64,
    /// Высота компонента
    pub component_height: f64,
    /// Вертикальный отступ между элементами
    pub vertical_spacing: f64,
    /// Горизонтальный отступ между элементами
    pub horizontal_spacing: f64,
    /// Радиус интерфейса (кружок)
    pub interface_radius: f64,
    /// Ширина пакета
    pub package_padding: f64,
    /// Высота заголовка пакета
    pub package_header_height: f64,
    /// Радиус скругления
    pub corner_radius: f64,
    /// Размер иконки компонента
    pub icon_size: f64,
}

impl Default for ComponentLayoutConfig {
    fn default() -> Self {
        Self {
            margin: 30.0,
            component_width: 140.0,
            component_height: 60.0,
            vertical_spacing: 50.0,
            horizontal_spacing: 60.0,
            interface_radius: 10.0,
            package_padding: 20.0,
            package_header_height: 25.0,
            corner_radius: 5.0,
            icon_size: 16.0,
        }
    }
}
