mod canvas;
mod input;
mod math;

use std::collections::HashMap;

use crossterm::event::KeyCode;

use crate::{
    canvas::{Canvas, draw::Align, font::Font},
    input::Input,
    math::mathi,
};

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,

    px: f32,
    py: f32,

    vx: f32,
    vy: f32,

    density: f32,
}

const PARTICLE_COUNT: usize = 3000;

const RADIUS: f32 = 1.5;
const DIAMETER: f32 = RADIUS * 2.0;

const GRAVITY: f32 = 100.0;
const PRESSURE: f32 = 0.35;
const VISCOSITY: f32 = 0.05;

const SOLVER_ITERATIONS: usize = 4;

const CELL_SIZE: f32 = DIAMETER * 2.0;

type Cell = (i32, i32);

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();

    let font = Font::load_from_file("assets/default_font.txt", 8);

    let mut last_frame = std::time::Instant::now();

    let mut display_fps = 0.0;
    let mut fps = 0;

    let mut particles = Vec::with_capacity(PARTICLE_COUNT);

    for i in 0..PARTICLE_COUNT {
        particles.push(Particle {
            x: (i % canvas.width() as usize) as f32,
            y: (i % canvas.height() as usize) as f32,

            px: 0.0,
            py: 0.0,

            vx: 0.0,
            vy: 0.0,

            density: 0.0,
        });
    }

    loop {
        let _ = input.update();

        if input.is_key_pressed(KeyCode::Esc) {
            break;
        }

        let now = std::time::Instant::now();

        let mut dt = now.duration_since(last_frame).as_secs_f32();

        last_frame = now;

        dt = dt.min(0.033);

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

        let width = canvas.width() as f32;
        let height = canvas.height() as f32;

        for p in &mut particles {
            p.vy -= GRAVITY * dt;

            p.px = p.x + p.vx * dt;
            p.py = p.y + p.vy * dt;
        }

        for _ in 0..SOLVER_ITERATIONS {
            let mut grid: HashMap<Cell, Vec<usize>> = HashMap::new();

            for (i, p) in particles.iter().enumerate() {
                let cx = (p.px / CELL_SIZE) as i32;
                let cy = (p.py / CELL_SIZE) as i32;

                grid.entry((cx, cy)).or_default().push(i);
            }

            for p in &mut particles {
                p.density = 0.0;
            }

            for i in 0..particles.len() {
                let cx = (particles[i].px / CELL_SIZE) as i32;

                let cy = (particles[i].py / CELL_SIZE) as i32;

                for oy in -1..=1 {
                    for ox in -1..=1 {
                        if let Some(list) = grid.get(&(cx + ox, cy + oy)) {
                            for &j in list {
                                if i == j {
                                    continue;
                                }

                                let dx = particles[j].px - particles[i].px;

                                let dy = particles[j].py - particles[i].py;

                                let dist_sq = dx * dx + dy * dy;

                                if dist_sq > (DIAMETER * 2.0) * (DIAMETER * 2.0) {
                                    continue;
                                }

                                let dist = dist_sq.sqrt();

                                let q = 1.0 - dist / (DIAMETER * 2.0);

                                particles[i].density += q.max(0.0);
                            }
                        }
                    }
                }
            }

            for i in 0..particles.len() {
                let cx = (particles[i].px / CELL_SIZE) as i32;

                let cy = (particles[i].py / CELL_SIZE) as i32;

                for oy in -1..=1 {
                    for ox in -1..=1 {
                        if let Some(list) = grid.get(&(cx + ox, cy + oy)) {
                            for &j in list {
                                if i >= j {
                                    continue;
                                }

                                let dx = particles[j].px - particles[i].px;

                                let dy = particles[j].py - particles[i].py;

                                let dist_sq = dx * dx + dy * dy;

                                if dist_sq < 0.0001 {
                                    continue;
                                }

                                let dist = dist_sq.sqrt();

                                let min_dist = DIAMETER;

                                if dist >= min_dist {
                                    continue;
                                }

                                let nx = dx / dist;
                                let ny = dy / dist;

                                let overlap = min_dist - dist;

                                let pressure = overlap * PRESSURE;

                                let corr = pressure * 0.5;

                                particles[i].px -= nx * corr;

                                particles[i].py -= ny * corr;

                                particles[j].px += nx * corr;

                                particles[j].py += ny * corr;
                            }
                        }
                    }
                }
            }

            for p in &mut particles {
                if p.px < RADIUS {
                    p.px = RADIUS;
                }

                if p.py < RADIUS {
                    p.py = RADIUS;
                }

                if p.px > width - RADIUS {
                    p.px = width - RADIUS;
                }

                if p.py > height - RADIUS {
                    p.py = height - RADIUS;
                }
            }
        }

        let mut grid: HashMap<Cell, Vec<usize>> = HashMap::new();

        for (i, p) in particles.iter().enumerate() {
            let cx = (p.px / CELL_SIZE) as i32;
            let cy = (p.py / CELL_SIZE) as i32;

            grid.entry((cx, cy)).or_default().push(i);
        }

        let mut vel_delta = vec![(0.0f32, 0.0f32); particles.len()];

        for i in 0..particles.len() {
            let cx = (particles[i].px / CELL_SIZE) as i32;
            let cy = (particles[i].py / CELL_SIZE) as i32;

            for oy in -1..=1 {
                for ox in -1..=1 {
                    if let Some(list) = grid.get(&(cx + ox, cy + oy)) {
                        for &j in list {
                            if i == j {
                                continue;
                            }

                            let dx = particles[j].px - particles[i].px;

                            let dy = particles[j].py - particles[i].py;

                            let dist_sq = dx * dx + dy * dy;

                            if dist_sq > (DIAMETER * 2.0) * (DIAMETER * 2.0) {
                                continue;
                            }

                            vel_delta[i].0 += (particles[j].vx - particles[i].vx) * VISCOSITY;

                            vel_delta[i].1 += (particles[j].vy - particles[i].vy) * VISCOSITY;
                        }
                    }
                }
            }
        }

        for i in 0..particles.len() {
            particles[i].vx = (particles[i].px - particles[i].x) / dt;

            particles[i].vy = (particles[i].py - particles[i].y) / dt;

            particles[i].vx += vel_delta[i].0;
            particles[i].vy += vel_delta[i].1;

            particles[i].x = particles[i].px;
            particles[i].y = particles[i].py;
        }

        canvas.clear();

        for p in &particles {
            let density = (p.density * 25.0).clamp(100.0, 255.0) as u8;

            canvas.set_pixel(
                p.x as u32,
                p.y as u32,
                mathi::rgb_to_u32(density / 4, density / 2, density),
            );
        }

        let blue = mathi::rgb_to_u32(127, 127, 255);
        let red = mathi::rgb_to_u32(255, 127, 127);

        canvas
            .draw(&font)
            .at(5, 5)
            .color(red)
            .align(Align::Left)
            .text("PBD Water Prototype");

        let x = canvas.width().saturating_sub(5);
        let y = canvas.height().saturating_sub(13);

        canvas
            .draw(&font)
            .at(x, y)
            .color(blue)
            .align(Align::Right)
            .uint(fps);

        canvas.render();
    }
}
