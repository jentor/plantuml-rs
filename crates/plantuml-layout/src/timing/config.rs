//! Конфигурация layout для Timing Diagrams

/// Конфигурация layout для Timing Diagrams
#[derive(Debug, Clone)]
pub struct TimingLayoutConfig {
    /// Отступ от краёв
    pub padding: f64,
    /// Ширина области имён участников
    pub participant_label_width: f64,
    /// Высота одного участника (lane)
    pub lane_height: f64,
    /// Вертикальный отступ между lanes
    pub lane_spacing: f64,
    /// Масштаб времени (пикселей на единицу времени)
    pub time_scale: f64,
    /// Высота состояния для robust
    pub robust_state_height: f64,
    /// Высота линии для concise
    pub concise_line_height: f64,
    /// Размер шрифта меток
    pub label_font_size: f64,
    /// Размер шрифта временных меток
    pub time_font_size: f64,
}

impl Default for TimingLayoutConfig {
    fn default() -> Self {
        Self {
            padding: 20.0,
            participant_label_width: 120.0,
            lane_height: 60.0,
            lane_spacing: 20.0,
            time_scale: 3.0, // 3 пикселя на единицу времени
            robust_state_height: 30.0,
            concise_line_height: 20.0,
            label_font_size: 12.0,
            time_font_size: 10.0,
        }
    }
}
