use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibMode2D};

pub fn draw_background(d: &mut RaylibMode2D<RaylibDrawHandle>) {
    // draw 10 hor and 10 ver lines (color rgb(170, 166, 157))
    let w = d.get_screen_width();
    let h = d.get_screen_height();

    let line_color = Color::new(170, 166, 157, 50);
    draw_lines(d, w, h, 4, line_color);

    let line_color = Color::new(170, 166, 157, 20);
    draw_lines(d, w, h, 8, line_color);
}

fn draw_lines(d: &mut RaylibMode2D<RaylibDrawHandle>, w: i32, h: i32, n_lines: i32, line_color: Color) {
    for i in 0..n_lines+1 {
        let x = (w / n_lines) * i;
        let y = (h / n_lines) * i;

        d.draw_line(
            x,
            0,
            x,
            h,
            line_color
        );
        d.draw_line(0, y, w, y, line_color);
    }
}