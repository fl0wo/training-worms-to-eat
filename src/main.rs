use realms::*;

fn main()
{
    let mut w: Window = Window::new("Training Worms", 800, 800);
    let fill: Rect = Rect::fill(&w, Color::rgb(91, 23, 127));

    let mut little_square = Rect::new(
        Vec2f::new(100.0, 100.0),
        Vec2f::new(50.0, 50.0),
        Color::rgb(255, 255, 255)
    );

    while w.is_running() {
        w.new_frame();

        fill.draw(&mut w);
        little_square.draw(&mut w);

        little_square.pos.x += 1.0;

    }
}