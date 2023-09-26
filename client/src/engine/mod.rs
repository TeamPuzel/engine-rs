#![allow(dead_code)]
mod runtime;
pub mod types;
use core::ops::Range;

use alloc::boxed::Box;

use types::*;

pub fn init(state: impl Game + 'static) {
    unsafe { runtime::STATE = Some(Box::new(state)) }
}

pub fn random(range: Range<u32>) -> u32 {
    unsafe { runtime::randomInRange(range.start, range.end) }
}

pub struct Renderer;
impl Renderer {
    pub fn clear(&mut self) {
        unsafe { runtime::clear(Color::Black) }
    }
    pub fn clear_with(&mut self, color: Color) {
        unsafe { runtime::clear(color) }
    }
    pub fn pixel(&mut self, x: u8, y: u8, color: Color) {
        unsafe { runtime::pixel(x, y, color) }
    }
    pub fn rectangle(&mut self, x: u8, y: u8, w: u8, h: u8, color: Color, fill: bool) {
        for sx in 0..w {
            for sy in 0..h {
                if sx + x == x || sx + x == x + w - 1 || sy + y == y || sy + y == y + h - 1 || fill {
                    self.pixel(sx + x, sy + y, color)
                }
            }
        }
    }
    pub fn sprite(&mut self, x: u8, y: u8, data: &Sprite) {
        for (sy, column) in data.iter().enumerate() {
            for (sx, pixel) in column.iter().enumerate() {
                self.pixel(x + sx as u8, y + sy as u8, *pixel)
            }
        }
    }
}

pub trait Game {
    fn frame(&mut self, renderer: &mut Renderer, input: &Input);
}