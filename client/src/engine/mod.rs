#![allow(dead_code)]
mod runtime;
pub mod types;
use types::*;

pub use runtime::FrameFn;
pub fn init(frame: FrameFn) {
    unsafe { runtime::FRAME_FN = Some(frame) }
}

pub struct Renderer;
impl Renderer {
    pub fn pixel(&self, x: u8, y: u8, color: Color) {
        unsafe { runtime::pixel(x, y, color) }
    }
    pub fn clear(&self) {
        unsafe { runtime::clear(Color::Black) }
    }
    pub fn clear_with(&self, color: Color) {
        unsafe { runtime::clear(color) }
    }
}