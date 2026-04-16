mod canvas;
mod input;
mod math;

use crossterm::event::KeyCode;

use crate::{canvas::{Canvas, font::Font}, input::Input, math::mathi};

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();

    let font = Font::load_from_file("assets/default_font.txt", 8);

    let program = 
r#"
mod canvas;
mod input;
mod math;

use crossterm::event::KeyCode;

use crate::{canvas::{Canvas, font::Font}, input::Input, math::mathi};

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();

    let font = Font::load_from_file("assets/default_font.txt", 8);

    let program = String::from("Hi ;)");

    let mut last_frame = std::time::Instant::now();
    let mut display_fps = 0.0; 
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

        canvas.clear();

        canvas.draw_text(&font, 5, 5, program, mathi::rgb_to_u32(127, 127, 127));
        canvas.draw_text(&font, canvas.width() - 23, canvas.height() - 13, &fps.to_string(), mathi::rgb_to_u32(127, 127, 127));

        canvas.render();
    }
}
"#;

    let program = program.trim_start_matches('\n');

    let mut vertical_offset: u32 = 0;

    let mut last_frame = std::time::Instant::now();
    let mut display_fps = 0.0; 
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

        if input.is_key_pressed(KeyCode::Up) {
            vertical_offset = vertical_offset.saturating_sub(1);
        }
        if input.is_key_pressed(KeyCode::Down) {
            vertical_offset += 1;
        }

        canvas.clear();

        canvas.draw_text(&font, 5, 5 - vertical_offset, program, mathi::rgb_to_u32(127, 127, 127));
        canvas.draw_text(&font, canvas.width() - 23, canvas.height() - 13, &fps.to_string(), mathi::rgb_to_u32(127, 127, 127));

        canvas.render();
    }
}