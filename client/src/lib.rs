#![no_std]
#![allow(unused, dead_code)]
extern crate alloc;
mod engine;
use alloc::vec::Vec;
use engine::{Renderer, Game, random};
use engine::types::{Input, Color};

#[no_mangle]
extern fn main() {
    engine::init(State::default())
}

struct Position { x: f32, y: f32 }

struct State {
    layer1: Vec<Position>,
    layer2: Vec<Position>,
    layer3: Vec<Position>,
    tick: usize
}

impl Default for State {
    fn default() -> Self {
        Self {
            layer1: Vec::with_capacity(136),
            layer2: Vec::with_capacity(136),
            layer3: Vec::with_capacity(136),
            tick: 0
        }
    }
}

fn advance_stars(stars: &mut Vec<Position>, by: f32) {
    let mut remove: Option<usize> = None;
    for (i, star) in stars.iter_mut().enumerate() {
        star.y += by;
        if star.y >= 128.0 {
            remove = Some(i);
        }
    }
    if let Some(remove) = remove { stars.swap_remove(remove); }
    stars.push(Position { x: random(0..128) as f32, y: 0.0 });
}

fn draw_stars(renderer: &mut Renderer, stars: &Vec<Position>, color: Color) {
    for star in stars { 
        if random(0..10) < 6 {
            renderer.pixel(star.x as u8, star.y as u8, color)
        }
    }
}

impl Game for State {
    fn frame(&mut self, renderer: &mut Renderer, input: &Input) {
        renderer.clear();
        
        advance_stars(&mut self.layer1, 1.0);
        advance_stars(&mut self.layer2, 0.75);
        advance_stars(&mut self.layer3, 0.5);
        
        draw_stars(renderer, &mut self.layer1, Color::LightGray);
        draw_stars(renderer, &mut self.layer2, Color::Blue);
        draw_stars(renderer, &mut self.layer3, Color::DarkBlue);
        
        self.tick += 1;
    }
}