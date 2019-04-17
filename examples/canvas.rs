use canvas::math::Color;
use canvas::{Context, Window};
use canvas::context::Vert;

pub fn main() {
    let mut window = Window::new("Canvas Example", 640.0, 480.0);
    let mut context = Context::new();

    let width   = 64.0;
    let height  = 64.0;

    let mut rotation = 0.0;

    while window.running() {
        let left = window.width() / 2.0 - width / 2.0;
        let top = window.height() / 2.0 - height / 2.0;
        window.handle_events();
        context.set_display_size(window.width(), window.height());
        context.set_origin(window.width() / 2.0, window.height() / 2.0);
        context.set_rotation_deg(rotation);
        context.clear();
        context.rect(Color::RED, left - 32.0, top, width, height);
        context.rect(Color::BLUE, left + 32.0, top, width, height);
        context.rect(Color::GREEN, left, top - 32.0, width, height);
        for i in 0..3 {
            let mut d = rotation * 2.0 * ((i + 1) as f32);
            while d >= 360.0 { d -= 360.0; }
            context.set_rotation_deg(d);
            context.rect(Color::WHITE, left + (width / 4.0), top - (height / 4.0), width / 2.0, height / 2.0);
        }
        context.flush_verts();
        window.flip();

        rotation += 0.10;
        while rotation >= 360.0 {
            rotation -= 360.0;
        }
    }
}
