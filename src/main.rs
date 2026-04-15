mod canvas;
mod input;
mod math;

use crossterm::event::KeyCode;

use crate::{canvas::{Canvas, font::Font}, input::Input, math::mathi};

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();

    let font = Font::load_default();

    for t in 0.. {
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

        for (index, c) in ('A'..'Z').enumerate() {
            canvas.draw_character(&font, index as u32 * 10, 10, c, mathi::rgb_to_u32(255, 0, 0));
        }

        canvas.render();
    }
}