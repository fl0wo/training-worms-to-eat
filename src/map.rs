use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};

pub fn draw_grid_lines(d: &mut RaylibDrawHandle) {
    // draw 10 hor and 10 ver lines (color rgb(170, 166, 157))
    let w = d.get_screen_width();
    let h = d.get_screen_height();

    let n_lines = 10;

    let line_color = Color::new(170, 166, 157, 50);

    for i in 0..n_lines {
        let x = (w / n_lines) * i;
        let y = (h / n_lines) * i;

        d.draw_line(x, 0, x, h, line_color);
        d.draw_line(0, y, w, y, line_color);
    }
}