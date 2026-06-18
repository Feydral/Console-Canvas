mod canvas;
mod input;
mod math;

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

    vx: f32,
    vy: f32,

    mass: f32,
    radius: f32,
}

const PARTICLE_COUNT: usize = 3000;

const G: f32 = 100.0;
const RESTITUTION: f32 = 0.4;
const DAMPING: f32 = 0.99;
const COLLISION_PASSES: usize = 4;

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();

    let font = Font::load_from_file("assets/default_font.txt", 8);

    let mut last_frame = std::time::Instant::now();

    let mut display_fps = 0.0;
    let mut fps = 0;

    let mut particles = Vec::with_capacity(PARTICLE_COUNT);

    let cols = 100;

    for i in 0..PARTICLE_COUNT {
        particles.push(Particle {
            x: 20.0 + (i % cols) as f32 * 5.0,
            y: 20.0 + (i / cols) as f32 * 5.0,

            vx: 0.0,
            vy: 0.0,

            mass: 1.0,
            radius: 0.5,
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

        for i in 0..particles.len() {
            for j in (i + 1)..particles.len() {
                let dx = particles[j].x - particles[i].x;
                let dy = particles[j].y - particles[i].y;

                let dist_sq = dx * dx + dy * dy + 1.0;

                let dist = dist_sq.sqrt();

                let nx = dx / dist;
                let ny = dy / dist;

                let force = G * particles[i].mass * particles[j].mass / dist_sq;

                let ai = force / particles[i].mass;
                let aj = force / particles[j].mass;

                particles[i].vx += nx * ai * dt;
                particles[i].vy += ny * ai * dt;

                particles[j].vx -= nx * aj * dt;
                particles[j].vy -= ny * aj * dt;
            }
        }

        for p in &mut particles {
            p.vx *= DAMPING;
            p.vy *= DAMPING;

            p.x += p.vx * dt;
            p.y += p.vy * dt;
        }

        for _ in 0..COLLISION_PASSES {
            for i in 0..particles.len() {
                for j in (i + 1)..particles.len() {
                    let dx = particles[j].x - particles[i].x;
                    let dy = particles[j].y - particles[i].y;

                    let dist_sq = dx * dx + dy * dy;

                    if dist_sq < 0.0001 {
                        continue;
                    }

                    let dist = dist_sq.sqrt();

                    let min_dist = particles[i].radius + particles[j].radius;

                    if dist >= min_dist {
                        continue;
                    }

                    let nx = dx / dist;
                    let ny = dy / dist;

                    let overlap = min_dist - dist;

                    particles[i].x -= nx * overlap * 0.5;
                    particles[i].y -= ny * overlap * 0.5;

                    particles[j].x += nx * overlap * 0.5;
                    particles[j].y += ny * overlap * 0.5;

                    let rvx = particles[j].vx - particles[i].vx;
                    let rvy = particles[j].vy - particles[i].vy;

                    let rel_normal = rvx * nx + rvy * ny;

                    if rel_normal >= 0.0 {
                        continue;
                    }

                    let inv_mass_a = 1.0 / particles[i].mass;
                    let inv_mass_b = 1.0 / particles[j].mass;

                    let impulse = -(1.0 + RESTITUTION) * rel_normal / (inv_mass_a + inv_mass_b);

                    let ix = impulse * nx;
                    let iy = impulse * ny;

                    particles[i].vx -= ix * inv_mass_a;
                    particles[i].vy -= iy * inv_mass_a;

                    particles[j].vx += ix * inv_mass_b;
                    particles[j].vy += iy * inv_mass_b;
                }
            }

            for p in &mut particles {
                if p.x < p.radius {
                    p.x = p.radius;

                    if p.vx < 0.0 {
                        p.vx = -p.vx * RESTITUTION;
                    }
                }

                if p.x > width - p.radius {
                    p.x = width - p.radius;

                    if p.vx > 0.0 {
                        p.vx = -p.vx * RESTITUTION;
                    }
                }

                if p.y < p.radius {
                    p.y = p.radius;

                    if p.vy < 0.0 {
                        p.vy = -p.vy * RESTITUTION;
                    }
                }

                if p.y > height - p.radius {
                    p.y = height - p.radius;

                    if p.vy > 0.0 {
                        p.vy = -p.vy * RESTITUTION;
                    }
                }
            }
        }

        canvas.clear();

        for p in &particles {
            canvas.set_pixel(p.x as u32, p.y as u32, mathi::rgb_to_u32(255, 255, 255));
        }

        let blue = mathi::rgb_to_u32(127, 127, 255);
        let red = mathi::rgb_to_u32(255, 127, 127);

        canvas
            .draw(&font)
            .at(5, 5)
            .color(red)
            .align(Align::Left)
            .text("Gravity Simulation");

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
