# Console Canvas 🎨

A lightweight Rust library for rendering pixels directly in the terminal, using ANSI true-color escape codes and a per-frame pixel buffer.

## Key features

- 🖼️ **Pixel rendering** — set and read individual pixels on a canvas that maps to your terminal size
- ✏️ **Bitmap font system** — custom `.ccfont` format with a fluent draw API for text and numbers
- ⌨️ **Input handling** — tracks key-down, key-held, and key-up states per frame; uses the Kitty keyboard protocol on supported terminals

## Usage

### Pixels

Colors are packed as `0xRRGGBBAA`.

```rust
let mut canvas = Canvas::new();
let color = mathi::rgb_to_u32(255, 127, 0);

// Write a pixel
canvas.set_pixel(10, 20, color);

// Read it back
let pixel = canvas.get_pixel(10, 20);
```

Call `canvas.clear()` at the start of each frame to reset the buffer, and `canvas.render()` at the end to flush it to the terminal.

### Fonts

Fonts are loaded from `.ccfont` files. Two are included: `default` and `default_bold`. Once loaded, text is drawn using a fluent builder — you can specify position, color, and alignment.

```rust
let font = Font::load_from_file("assets/default.ccfont");
let bold_font = Font::load_from_file("assets/default_bold.ccfont");

canvas
    .draw(&font)
    .at(10, 10)
    .color(mathi::rgb_to_u32(255, 255, 255))
    .align(Align::Left)
    .text("Hello, world!");
```

The builder also supports `.uint()`, `.int()`, and `.float()` for rendering numbers directly without formatting them yourself.

### Full example

```rust
let mut canvas = Canvas::new();
let mut input = Input::new();
let font = Font::load_from_file("assets/default.ccfont");

loop {
    input.update().unwrap();
    if input.is_key_pressed(KeyCode::Esc) { break; }

    canvas.clear();
    canvas.set_pixel(5, 5, mathi::rgb_to_u32(255, 0, 0));
    canvas
        .draw(&font)
        .at(10, 10)
        .text("Hello!");
    canvas.render();
}

canvas.end();
```

## Dependencies

- [`crossterm`](https://github.com/crossterm-rs/crossterm) — terminal control and input
- [`glam`](https://github.com/bitshifter/glam-rs) — vector math
