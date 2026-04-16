mod canvas;
mod input;
mod math;

use crossterm::event::KeyCode;

use crate::{canvas::{Canvas, font::Font}, input::Input, math::mathi};

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();

    let font = Font::load_from_file("assets/default_font.txt", 8);

    let program = r#"
        fn main() {
            println!("Start of program");
    
            let a = 10;
            let b = 25;
            let sum = a + b;
    
            println!("Calculation:");
            println!("\ta = {}", a);
            println!("\tb = {}", b);
            println!("\tSum = {}", sum);
    
            if sum > 30 {
                println!("Result is greater than 30");
            } else {
                println!("Result is less than or equal to 30");
            }
    
            for i in 0..5 {
                println!("Loop iteration {}", i);
            }
    
            println!("End of program");
        }
    "#;

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

        canvas.draw_text(&font, 5, 5, program, mathi::rgb_to_u32(127, 127, 127));
    
        canvas.render();
    }
}