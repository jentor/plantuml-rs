//! Примитивы для рисования фигур

use crate::Point;

/// Строит путь для прямоугольника со скруглёнными углами
pub fn rounded_rect_path(x: f64, y: f64, width: f64, height: f64, radius: f64) -> String {
    if radius <= 0.0 {
        return format!("M{},{} h{} v{} h{} Z", x, y, width, height, -width);
    }

    let r = radius.min(width / 2.0).min(height / 2.0);

    format!(
        "M{},{} h{} a{},{} 0 0 1 {},{} v{} a{},{} 0 0 1 {},{} h{} a{},{} 0 0 1 {},{} v{} a{},{} 0 0 1 {},{} Z",
        x + r, y,
        width - 2.0 * r,
        r, r, r, r,
        height - 2.0 * r,
        r, r, -r, r,
        -(width - 2.0 * r),
        r, r, -r, -r,
        -(height - 2.0 * r),
        r, r, r, -r
    )
}

/// Строит путь для стрелки
pub fn arrow_path(from: Point, to: Point, head_size: f64) -> String {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let angle = dy.atan2(dx);

    let head_angle = std::f64::consts::PI / 6.0; // 30 градусов

    let x1 = to.x - head_size * (angle - head_angle).cos();
    let y1 = to.y - head_size * (angle - head_angle).sin();
    let x2 = to.x - head_size * (angle + head_angle).cos();
    let y2 = to.y - head_size * (angle + head_angle).sin();

    format!(
        "M{},{} L{},{} M{},{} L{},{} L{},{}",
        from.x, from.y, to.x, to.y, to.x, to.y, x1, y1, x2, y2
    )
}

/// Строит путь для ромба (diamond)
pub fn diamond_path(cx: f64, cy: f64, size: f64) -> String {
    let half = size / 2.0;
    format!(
        "M{},{} L{},{} L{},{} L{},{} Z",
        cx,
        cy - half, // Верх
        cx + half,
        cy, // Право
        cx,
        cy + half, // Низ
        cx - half,
        cy // Лево
    )
}

/// Строит путь для актёра (человечек)
pub fn actor_path(cx: f64, cy: f64, scale: f64) -> String {
    let head_r = 8.0 * scale;
    let body_h = 20.0 * scale;
    let arms_w = 20.0 * scale;
    let legs_h = 15.0 * scale;

    // Голова (круг)
    let head = format!(
        "M{},{} a{},{} 0 1 0 {},0 a{},{} 0 1 0 {},0",
        cx - head_r,
        cy - body_h - head_r,
        head_r,
        head_r,
        head_r * 2.0,
        head_r,
        head_r,
        -head_r * 2.0
    );

    // Тело
    let body = format!("M{},{} L{},{}", cx, cy - body_h, cx, cy);

    // Руки
    let arms = format!(
        "M{},{} L{},{}",
        cx - arms_w / 2.0,
        cy - body_h / 2.0,
        cx + arms_w / 2.0,
        cy - body_h / 2.0
    );

    // Ноги
    let legs = format!(
        "M{},{} L{},{} M{},{} L{},{}",
        cx,
        cy,
        cx - arms_w / 3.0,
        cy + legs_h,
        cx,
        cy,
        cx + arms_w / 3.0,
        cy + legs_h
    );

    format!("{} {} {} {}", head, body, arms, legs)
}

/// Строит путь для базы данных (цилиндр)
pub fn database_path(x: f64, y: f64, width: f64, height: f64) -> String {
    let ellipse_h = height * 0.15;

    // Верхний эллипс
    let top = format!(
        "M{},{} a{},{} 0 1 0 {},0 a{},{} 0 1 0 {},0",
        x,
        y + ellipse_h,
        width / 2.0,
        ellipse_h,
        width,
        width / 2.0,
        ellipse_h,
        -width
    );

    // Боковые линии
    let sides = format!(
        "M{},{} L{},{} M{},{} L{},{}",
        x,
        y + ellipse_h,
        x,
        y + height - ellipse_h,
        x + width,
        y + ellipse_h,
        x + width,
        y + height - ellipse_h
    );

    // Нижний эллипс (половина)
    let bottom = format!(
        "M{},{} a{},{} 0 0 0 {},0",
        x,
        y + height - ellipse_h,
        width / 2.0,
        ellipse_h,
        width
    );

    format!("{} {} {}", top, sides, bottom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rounded_rect() {
        let path = rounded_rect_path(0.0, 0.0, 100.0, 50.0, 5.0);
        assert!(path.contains("M5,0"));
        assert!(path.contains("a5,5"));
    }

    #[test]
    fn test_diamond() {
        let path = diamond_path(50.0, 50.0, 20.0);
        assert!(path.contains("M50,40")); // Верх
        assert!(path.contains("L60,50")); // Право
    }
}
