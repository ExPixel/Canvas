use canvas::math::Color;
use canvas::{Context, Window};
use canvas::context::Vert;

pub fn main() {
    let mut window = Window::new("Canvas Example", 640.0, 480.0);
    let mut context = Context::new();

    let width   = 64.0;
    let height  = 64.0;
    let left    = 640.0 / 2.0 - width / 2.0;
    let top     = 480.0 / 2.0 + height / 2.0;

    while window.running() {
        window.handle_events();
        context.set_display_size(window.width(), window.height());
        context.clear();
        context.rect(Color::RED, left - 32.0, top, width, height);
        context.rect(Color::BLUE, left + 32.0, top, width, height);
        context.rect(Color::GREEN, left, top - 32.0, width, height);
        context.flush_verts();
        window.flip();
    }
}
