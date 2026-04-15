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

        canvas.draw_text(&font, 10, 10, ('A'..='Z').collect(), mathi::rgb_to_u32(127, 127, 255));
        canvas.draw_text(&font, 10, 20, ('0'..='9').collect(), mathi::rgb_to_u32(127, 127, 255));

        let symbols = ". , ; : - _ # ! ?";

        canvas.draw_text(&font, 10, 30, symbols.to_string(), mathi::rgb_to_u32(127, 127, 255));

        canvas.render();
    }
}