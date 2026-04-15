mod canvas;
mod input;
mod math;

use crossterm::event::KeyCode;

use crate::{canvas::{Canvas, font::Font}, input::Input, math::mathi};

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();

    let font = Font::load_default();

    loop {
        let _ = input.update();
        if input.is_key_down(KeyCode::Esc) {
            break;
        }

        let new_w = Canvas::terminal_width();
        let new_h = Canvas::terminal_height();

        if new_w as u32 != canvas.width() || new_h as u32 != canvas.height() {
            canvas.resize(new_w, new_h);
        }

        canvas.clear();

        canvas.draw_text(&font, 10, 10, "ABCDEFGHIJKLMNOPQRSTUVWXYZ", mathi::rgb_to_u32(255, 0, 0));

        canvas.draw_text(&font, 10, 20, "abcdefghijklmnopqrstuvwxyz", mathi::rgb_to_u32(0, 255, 0));

        canvas.draw_text(&font, 10, 30, "0123456789", mathi::rgb_to_u32(0, 0, 255));

        canvas.draw_text(&font, 10, 40, "()[]{}<>", mathi::rgb_to_u32(255, 255, 0));

        canvas.draw_text(&font, 10, 50, ".,;:!?-_#", mathi::rgb_to_u32(255, 127, 255));

        canvas.draw_text(&font, 10, 60, "Hello, world!", mathi::rgb_to_u32(200, 200, 200));

        canvas.draw_text(&font, 10, 70, "Rust 1.75 - memory safe!", mathi::rgb_to_u32(127, 200, 255));

        canvas.draw_text(&font, 10, 80, "a1b2c3d4e5f6g7h8i9j0", mathi::rgb_to_u32(255, 100, 100));

        canvas.draw_text(&font, 10, 90, "(()()){}[]!!??,,..;;", mathi::rgb_to_u32(180, 180, 180));

        canvas.draw_text(&font, 10, 100, "THE QUICK BROWN FOX JUMPS", mathi::rgb_to_u32(255, 180, 80));

        canvas.render();
    }
}