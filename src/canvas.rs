pub mod color;
pub mod input;

use crossterm::terminal;
use std::io::{Write, stdout};
use crate::canvas::color::Color;

pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
    out: Vec<u8>,
}

impl Canvas {
    pub fn new() -> Self {
        let (w, h_half) = terminal::size().unwrap();
        let h = h_half * 2;
        let size = w as usize * h as usize;

        print!("\x1b[3J\x1b[H\x1b[?25l\x1b[?1049h");
        stdout().flush().unwrap();

        Self {
            width: w as u32,
            height: h as u32,
            pixels: vec![Color::BLACK; size],
            out: Vec::with_capacity(w as usize * h_half as usize * 25),
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.pixels[y as usize * self.width as usize + x as usize] = color;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x >= self.width || y >= self.height {
            return Color::BLACK;
        }
        self.pixels[y as usize * self.width as usize + x as usize]
    }

    pub fn clear(&mut self) {
        self.pixels.fill(Color::BLACK);
    }

    pub fn render(&mut self) {
        if let Ok((w, h_half)) = terminal::size() {
            let h = h_half * 2;
            if w as u32 != self.width() || h as u32 != self.height() {
                self.resize();
            }
        }
        
        self.out.clear();
        self.out.extend_from_slice(b"\x1b[?2026h\x1b[H");
        
        let rows = self.height / 2;
        let mut last_fg = Color::BLACK;
        let mut last_bg = Color::BLACK;
        
        self.out.extend_from_slice(b"\x1b[38;2;0;0;0m\x1b[48;2;0;0;0m");
        
        for row in 0..rows {
            let inv      = rows - 1 - row;
            let y_top    = inv * 2 + 1;
            let y_bottom = inv * 2;
            
            for x in 0..self.width {
                let fg = self.get_pixel(x, y_top);
                let bg = self.get_pixel(x, y_bottom);
                
                if fg != last_fg {
                    write!(&mut self.out, "\x1b[38;2;{};{};{}m", fg.r, fg.g, fg.b).unwrap();
                    last_fg = fg;
                }
                if bg != last_bg {
                    write!(&mut self.out, "\x1b[48;2;{};{};{}m", bg.r, bg.g, bg.b).unwrap();
                    last_bg = bg;
                }
                
                self.out.extend_from_slice("▀".as_bytes());
            }
        }
        
        self.out.extend_from_slice(b"\x1b[0m\x1b[?2026l");
        let mut stdout = stdout();
        stdout.write_all(&self.out).unwrap();
        stdout.flush().unwrap();
    }
    
    fn resize(&mut self) {
        let (w, h_half) = terminal::size().unwrap();
        let h = h_half * 2;
        let new_size = w as usize * h as usize;
        
        self.width = w as u32;
        self.height = h as u32;
        
        self.pixels.clear();
        self.pixels.resize(new_size, Color::BLACK);
        
        self.out.clear();
        self.out.reserve(w as usize * h_half as usize * 20);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        print!("\x1b[?25h\x1b[?1049l\x1b[2J\x1b[3J");
        stdout().flush().unwrap();
    }
}