use core::panic::*;
use core::alloc::GlobalAlloc;
use alloc::boxed::Box;

use super::types::{Color, RGB, Input};
use super::{Renderer, Game};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

static mut HEAP: [u8; 2_000_000] = [0; 2_000_000];
static mut OFFSET: usize = 0;
struct FixedBufferAlloc;
unsafe impl GlobalAlloc for FixedBufferAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr = HEAP.as_mut_ptr().add(OFFSET);
        OFFSET += layout.size();
        return ptr;
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // :)
    }
}

#[global_allocator]
static ALLOC: FixedBufferAlloc = FixedBufferAlloc;

extern {
    fn draw(buf: *const u8);
    pub fn random() -> f32;
    pub fn randomInRange(min: u32, max: u32) -> u32;
}

static mut DISPLAY: [[RGB; 128]; 128] = [[Color::Black.rgb(); 128]; 128];
pub static mut INPUT: Input = Input::new();
pub static mut STATE: Option<Box<dyn Game>> = None;
static mut RENDERER: Renderer = Renderer;

pub unsafe fn clear(color: Color) {
    DISPLAY = [[color.rgb(); 128]; 128];
}

pub unsafe fn pixel(x: u8, y: u8, color: Color) {
    if x < 128 && y < 128 {
        DISPLAY[y as usize][x as usize] = color.rgb();
    }
}

#[no_mangle]
unsafe extern fn frame() {
    STATE.as_mut().unwrap().frame(&mut RENDERER, &INPUT);
    draw(DISPLAY.as_ptr() as *const u8)
}

unsafe extern fn key_down(code: u32) { key(code, true) }
unsafe extern fn key_up(code: u32) { key(code, false) }

#[inline]
unsafe fn key(code: u32, value: bool) {
    match code {
        0 => INPUT.up    = value,
        1 => INPUT.left  = value,
        2 => INPUT.down  = value,
        3 => INPUT.right = value,
        4 => INPUT.a     = value,
        5 => INPUT.b     = value,
        _ => {}
    }
}