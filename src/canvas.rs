pub mod font;

use crossterm::terminal;
use std::io::{Write, stdout};

use crate::canvas::font::Font;

pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
    out: Vec<u8>,
}

#[allow(dead_code)]
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
            pixels: vec![0; size],
            out: Vec::with_capacity(w as usize * h_half as usize * 25),
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.width || y >= self.height { return; }
        self.pixels[y as usize * self.width as usize + x as usize] = color;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
        if x >= self.width || y >= self.height { return 0; }
        self.pixels[y as usize * self.width as usize + x as usize]
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn draw_text(&mut self, text: &str, x: u32, y: u32, color: u32, align: Align, font: &Font) {
        let glyph_height = font.glyph_height() as u32;
        let tab_size = 4;

        let mut cursor_y = y;

        for line in text.split('\n') {
            let mut line_width = 0i32;
            let mut chars = line.chars().peekable();

            while let Some(c) = chars.next() {
                match c {
                    '\t' => {
                        let tab_w = (glyph_height * tab_size) as i32;
                        line_width += tab_w;
                    }
                    '\\' => {
                        match chars.peek() {
                            Some('n') | Some('t') | Some('\\') => {
                                let c2 = chars.next().unwrap();
                                let actual = match c2 {
                                    'n' => '\n',
                                    't' => '\t',
                                    '\\' => '\\',
                                    _ => c2,
                                };

                                if let Some(g) = font.glyphs.get(&actual) {
                                    line_width += g.len() as i32 / glyph_height as i32;
                                }
                            }
                            _ => {
                                if let Some(g) = font.glyphs.get(&'\\') {
                                    line_width += g.len() as i32 / glyph_height as i32;
                                }
                            }
                        }
                    }
                    _ => {
                        if let Some(g) = font.glyphs.get(&c) {
                            line_width += g.len() as i32 / glyph_height as i32;
                        }
                    }
                }
            }

            let mut cursor_x = match align {
                Align::Left => x as i32,
                Align::Right => x as i32 - line_width,
            };

            let mut chars = line.chars().peekable();

            while let Some(c) = chars.next() {
                match c {
                    '\t' => {
                        let tab_w = (glyph_height * tab_size) as i32;
                        let offset = cursor_x - x as i32;
                        cursor_x = x as i32 + ((offset / tab_w) + 1) * tab_w;
                    }

                    '\\' => {
                        let draw_char = match chars.peek() {
                            Some('n') => {
                                chars.next();
                                '\n'
                            }
                            Some('t') => {
                                chars.next();
                                '\t'
                            }
                            Some('\\') => {
                                chars.next();
                                '\\'
                            }
                            _ => '\\',
                        };

                        if draw_char == '\n' {
                            break;
                        }

                        if draw_char == '\t' {
                            let tab_w = (glyph_height * tab_size) as i32;
                            let offset = cursor_x - x as i32;
                            cursor_x = x as i32 + ((offset / tab_w) + 1) * tab_w;
                            continue;
                        }

                        if let Some(g) = font.glyphs.get(&draw_char) {
                            let w = g.len() as i32 / glyph_height as i32;
                            self.draw_character(draw_char, cursor_x as u32, cursor_y, color, Align::Left, font);
                            cursor_x += w;
                        }
                    }

                    _ => {
                        if let Some(g) = font.glyphs.get(&c) {
                            let w = g.len() as i32 / glyph_height as i32;
                            self.draw_character(c, cursor_x as u32, cursor_y, color, Align::Left, font);
                            cursor_x += w;
                        }
                    }
                }
            }

            cursor_y += glyph_height + 2;
        }
    }

    pub fn draw_character(&mut self, c: char, x: u32, y: u32, color: u32, align: Align, font: &Font) {
        let Some(glyph) = font.glyphs.get(&c) else {
            return;
        };

        let height = font.glyph_height() as i32;
        let width = glyph.len() as i32 / height;

        let x = x as i32;
        let y = y as i32;

        let start_x = match align {
            Align::Left  => x,
            Align::Right => x - width,
        };

        for gy in 0..height {
            for gx in 0..width {
                let idx = (gy * width + gx) as usize;

                if glyph[idx] == 1 {
                    let dx = start_x + gx;
                    let dy = y + gy;

                    if dx < 0 || dy < 0 ||
                       dx >= self.width as i32 ||
                       dy >= self.height as i32
                    {
                        continue;
                    }

                    let draw_y = self.height as i32 - 1 - dy;
                    self.set_pixel(dx as u32, draw_y as u32, color);
                }
            }
        }
    }

    pub fn draw_uint(&mut self, value: u32, x: u32, y: u32, color: u32, align: Align, font: &Font) {
        let glyph_height = font.glyph_height() as i32;
        let digits: Vec<char> = value.to_string().chars().collect();

        let mut total_width = 0i32;
        for &c in &digits {
            if let Some(glyph) = font.glyphs.get(&c) {
                total_width += glyph.len() as i32 / glyph_height;
            }
        }

        let mut cursor_x = match align {
            Align::Left  => x as i32,
            Align::Right => x as i32 - total_width,
        };

        for &c in &digits {
            if let Some(glyph) = font.glyphs.get(&c) {
                let w = glyph.len() as i32 / glyph_height;
                self.draw_character(c, cursor_x as u32, y, color, Align::Left, font);
                cursor_x += w;
            }
        }
    }

    pub fn draw_int(&mut self, value: i32, x: u32, y: u32, color: u32, align: Align, font: &Font, always_show_sign: bool) {
        let glyph_height = font.glyph_height() as i32;

        let (sign_char, abs_val): (char, u32) = if value < 0 {
            ('-', value.wrapping_abs() as u32)
        } else {
            ('+', value as u32)
        };

        let show_sign = value < 0 || always_show_sign;
        let digits: Vec<char> = abs_val.to_string().chars().collect();

        let mut total_width = 0i32;

        if show_sign {
            if let Some(glyph) = font.glyphs.get(&sign_char) {
                total_width += glyph.len() as i32 / glyph_height;
            }
        }

        for &c in &digits {
            if let Some(glyph) = font.glyphs.get(&c) {
                total_width += glyph.len() as i32 / glyph_height;
            }
        }

        let mut cursor_x = match align {
            Align::Left => x as i32,
            Align::Right => x as i32 - total_width,
        };

        if show_sign {
            if let Some(glyph) = font.glyphs.get(&sign_char) {
                let w = glyph.len() as i32 / glyph_height;
                self.draw_character(sign_char, cursor_x as u32, y, color, Align::Left, font);
                cursor_x += w;
            }
        }

        for &c in &digits {
            if let Some(glyph) = font.glyphs.get(&c) {
                let w = glyph.len() as i32 / glyph_height;
                self.draw_character(c, cursor_x as u32, y, color, Align::Left, font);
                cursor_x += w;
            }
        }
    }

    pub fn draw_float(&mut self, value: f32, x: u32, y: u32, color: u32, align: Align, font: &Font, decimals: usize, always_show_sign: bool) {
        let glyph_height = font.glyph_height() as i32;

        let mut s = format!("{:+.*}", decimals + 5, value);

        if let Some(dot) = s.find('.') {
            let end = dot + 1 + decimals;
            if s.len() > end {
                s.truncate(end);
            }
        }

        if !always_show_sign && s.starts_with('+') {
            s.remove(0);
        }

        let chars: Vec<char> = s.chars().collect();

        let mut total_width = 0i32;
        for &c in &chars {
            if let Some(glyph) = font.glyphs.get(&c) {
                total_width += glyph.len() as i32 / glyph_height;
            }
        }

        let mut cursor_x = match align {
            Align::Left => x as i32,
            Align::Right => x as i32 - total_width,
        };

        for &c in &chars {
            if let Some(glyph) = font.glyphs.get(&c) {
                let w = glyph.len() as i32 / glyph_height;
                self.draw_character(c, cursor_x as u32, y, color, Align::Left, font);
                cursor_x += w;
            }
        }
    }

    pub fn render(&mut self) {
        self.out.clear();
        self.out.extend_from_slice(b"\x1b[?2026h\x1b[H");

        let rows = self.height / 2;
        let mut last_fg = 0;
        let mut last_bg = 0;

        self.out.extend_from_slice(b"\x1b[38;2;0;0;0m\x1b[48;2;0;0;0m");

        for row in 0..rows {
            let inv = rows - 1 - row;
            let y_top = inv * 2 + 1;
            let y_bottom = inv * 2;

            for x in 0..self.width {
                let fg = self.get_pixel(x, y_top);
                let bg = self.get_pixel(x, y_bottom);

                if fg != last_fg {
                    write!(&mut self.out, "\x1b[38;2;{};{};{}m", (fg >> 24) as u8, (fg >> 16) as u8, (fg >> 8) as u8).unwrap();
                    last_fg = fg;
                }
                if bg != last_bg {
                    write!(&mut self.out, "\x1b[48;2;{};{};{}m", (bg >> 24) as u8, (bg >> 16) as u8, (bg >> 8) as u8).unwrap();
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

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
        self.pixels.clear();
        self.pixels.resize((new_width * new_height) as usize, 0);
        self.out.clear();
        self.out.reserve(new_width as usize * (new_height / 2) as usize * 20);
    }

    pub fn width(&self) -> u32 { self.width  }
    pub fn height(&self) -> u32 { self.height }

    pub fn terminal_width() -> u32 {
        terminal::size().expect("terminal::size()").0 as u32
    }
    pub fn terminal_height() -> u32 {
        2 * terminal::size().expect("terminal::size()").1 as u32
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        print!("\x1b[?25h\x1b[?1049l\x1b[2J\x1b[3J");
        stdout().flush().unwrap();
    }
}

pub enum Align { Left, Right }