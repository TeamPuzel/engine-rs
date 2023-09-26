use core::panic::*;
use core::alloc::GlobalAlloc;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;

use super::types::{Color, RGB, Input};
use super::{Renderer, Game};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let msg = format!("{info}");
    unsafe { panicHandler(msg.as_ptr(), msg.len() as u32) }
}

const GB: usize = 1073741824;

static mut HEAP: [u8; GB / 64] = [0; GB / 64];
static mut OFFSET: usize = 0;
struct WasmAlloc;
unsafe impl GlobalAlloc for WasmAlloc {
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
static ALLOC: WasmAlloc = WasmAlloc;

extern {
    fn draw(buf: *const u8);
    fn panicHandler(info: *const u8, len: u32) -> !;
    pub fn random() -> f32;
    pub fn randomInRange(min: u32, max: u32) -> u32;
}

pub static mut DISPLAY: Vec<RGB> = Vec::new();
pub static mut INPUT: Input = Input::new();
pub static mut STATE: Option<Box<dyn Game>> = None;
static mut RENDERER: Renderer = Renderer;

pub unsafe fn clear(color: Color) {
    DISPLAY.fill(color.rgb());
}

pub unsafe fn pixel(x: u32, y: u32, color: Color) {
    if x < WIDTH as u32 && y < HEIGHT as u32 {
        DISPLAY[(x + y * WIDTH) as usize] = color.rgb();
    }
}

#[no_mangle]
unsafe extern fn frame() {
    STATE.as_mut().unwrap().frame(&mut RENDERER, &INPUT);
    draw(DISPLAY.as_ptr() as *const u8)
}

pub static mut WIDTH: u32 = 128;
pub static mut HEIGHT: u32 = 128;

#[no_mangle] unsafe extern fn display_width()  -> u32 { WIDTH }
#[no_mangle] unsafe extern fn display_height() -> u32 { HEIGHT }

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