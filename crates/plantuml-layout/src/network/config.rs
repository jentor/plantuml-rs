//! Конфигурация layout для Network диаграмм

/// Конфигурация для Network layout engine
#[derive(Debug, Clone)]
pub struct NetworkLayoutConfig {
    /// Отступ от краёв диаграммы
    pub padding: f64,
    /// Высота полосы сети
    pub network_band_height: f64,
    /// Вертикальный отступ между сетями
    pub network_spacing: f64,
    /// Ширина сервера
    pub server_width: f64,
    /// Высота сервера
    pub server_height: f64,
    /// Горизонтальный отступ между серверами
    pub server_spacing: f64,
    /// Высота заголовка сети
    pub network_header_height: f64,
    /// Размер шрифта
    pub font_size: f64,
    /// Цвет фона сети
    pub network_bg_color: &'static str,
    /// Цвет фона сервера
    pub server_bg_color: &'static str,
    /// Цвет фона группы
    pub group_bg_color: &'static str,
}

impl Default for NetworkLayoutConfig {
    fn default() -> Self {
        Self {
            padding: 20.0,
            network_band_height: 120.0,
            network_spacing: 40.0,
            server_width: 100.0,
            server_height: 60.0,
            server_spacing: 30.0,
            network_header_height: 25.0,
            font_size: 12.0,
            network_bg_color: "#E2E2F0",
            server_bg_color: "#FEFECE",
            group_bg_color: "#FFAAAA33",
        }
    }
}
