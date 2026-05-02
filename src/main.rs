mod canvas;
mod input;
mod math;

use crossterm::event::KeyCode;

use crate::{canvas::{Canvas, draw::Align, font::Font}, input::Input, math::mathi};

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();

    let font = Font::load_from_file("assets/default_font.txt", 8);

    let mut last_frame = std::time::Instant::now();
    let mut display_fps = 0.5;
    let mut fps = 0;

    loop {
        let _ = input.update();
        if input.is_key_pressed(KeyCode::Esc) {
            break;
        }

        let now = std::time::Instant::now();
        let dt = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;
        display_fps += dt;

        let new_w = Canvas::terminal_width();
        let new_h = Canvas::terminal_height();
        if new_w as u32 != canvas.width() || new_h as u32 != canvas.height() {
            canvas.resize(new_w, new_h);
        }

        if display_fps > 1.0 {
            display_fps = 0.0;
            fps = (1.0 / dt) as u32;
        }

        let blue = mathi::rgb_to_u32(127, 127, 255);
        let gray = mathi::rgb_to_u32(127, 127, 127);

        canvas.clear();

        canvas.draw(&font)
            .at(5, 5)
            .color(gray)
            .align(Align::Left)
            .text("Hello, world!");

        canvas.draw(&font)
            .at(11, 15)
            .color(gray)
            .align(Align::Right)
            .int(3, false);

        canvas.draw(&font)
            .at(11, 25)
            .color(gray)
            .align(Align::Right)
            .int(-7, false);

        let x = canvas.width().saturating_sub(5);
        let y = canvas.height().saturating_sub(13);

        canvas.draw(&font)
            .at(x, y)
            .color(blue)
            .align(Align::Right)
            .uint(fps);

        let y = canvas.height().saturating_sub(23);

        canvas.draw(&font)
            .at(x, y)
            .color(blue)
            .align(Align::Right)
            .float(dt, 5, false);

        canvas.render();
    }
}