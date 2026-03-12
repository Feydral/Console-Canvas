#![allow(unused_imports)]

use crossterm::event::KeyCode;

use crate::canvas::{Canvas, color::Color, input::Input};

mod canvas;

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();

    for t in 0.. {
        if let Err(input) = input.update() {
            eprintln!("Frame {}: {:?}", t, input);
        }

        if input.is_key_pressed(KeyCode::Esc) {
            break;
        }

        canvas.clear();

        for x in 0..canvas.width() {
            for y in 0..canvas.height() {
                canvas.set_pixel(x, y, Color::new(x as u8, y as u8, t));
            }
        }

        canvas.render();
    }
}