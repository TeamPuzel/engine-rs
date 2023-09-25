#![no_std]
#![allow(unused, dead_code)]
mod engine;
use engine::Renderer;
use engine::types::{Input, Color};

#[no_mangle]
extern fn main() {
    engine::init(frame)
}

fn frame(renderer: &Renderer, input: &Input) {
    renderer.pixel(0, 0, Color::Red);
    renderer.pixel(1, 0, Color::Green);
}