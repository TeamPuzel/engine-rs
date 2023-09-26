#![no_std]
#![allow(unused, dead_code)]
extern crate alloc;
mod engine;
mod sprites;
use alloc::vec::Vec;
use engine::{Renderer, Game, random};
use engine::types::{Input, Color};

#[no_mangle]
extern fn main() {
    engine::set_display_size(320, 200);
    engine::init(State::default());
}

struct Position { x: f32, y: f32 }

struct State {
    layer1: Vec<Position>,
    layer2: Vec<Position>,
    layer3: Vec<Position>,
    tick: usize
}

const L1_SPEED: f32 = 0.75;
const L2_SPEED: f32 = 0.5;
const L3_SPEED: f32 = 0.25;

impl Default for State {
    fn default() -> Self {
        let mut buf = Self {
            layer1: Vec::with_capacity(1024),
            layer2: Vec::with_capacity(1024),
            layer3: Vec::with_capacity(1024),
            tick: 0
        };
        for _ in 0..1024 {
            advance_stars(&mut buf.layer1, L1_SPEED);
            advance_stars(&mut buf.layer2, L2_SPEED);
            advance_stars(&mut buf.layer3, L3_SPEED);
        }
        return buf
    }
}

fn advance_stars(stars: &mut Vec<Position>, by: f32) {
    let mut remove: Option<usize> = None;
    for (i, star) in stars.iter_mut().enumerate() {
        star.y += by;
        if star.y >= 200.0 {
            remove = Some(i);
        }
    }
    if let Some(remove) = remove { stars.swap_remove(remove); }
    stars.push(Position { x: random(0..320) as f32, y: 0.0 });
}

fn draw_stars(renderer: &mut Renderer, stars: &Vec<Position>, color: Color) {
    for star in stars { 
        if random(0..100) < 50 {
            renderer.pixel(star.x as u32, star.y as u32, color)
        }
    }
}

impl Game for State {
    fn frame(&mut self, renderer: &mut Renderer, input: &Input) {
        renderer.clear();
        
        advance_stars(&mut self.layer1, L1_SPEED);
        advance_stars(&mut self.layer2, L2_SPEED);
        advance_stars(&mut self.layer3, L3_SPEED);
        
        draw_stars(renderer, &mut self.layer1, Color::White);
        draw_stars(renderer, &mut self.layer2, Color::Blue);
        draw_stars(renderer, &mut self.layer3, Color::DarkBlue);
        
        self.tick += 1;
    }
}