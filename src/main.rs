mod canvas;
mod input;
mod math;

use crossterm::event::KeyCode;

use crate::{canvas::Canvas, input::Input, math::mathi};

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();

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

        for x in 0..canvas.width() {
            for y in 0..canvas.height() {
                canvas.set_pixel(x, y, mathi::rgb_to_u32(x as u8, y as u8, t as u8));
            }
        }

        canvas.render();
    }
}