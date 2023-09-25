use core::panic::*;
use super::types::{Color, RGB, Input};
use super::Renderer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

extern {
    fn draw(buf: *const u8);
    fn random() -> f32;
}

pub type FrameFn = fn(&Renderer, &Input) -> ();

static mut DISPLAY: [[RGB; 128]; 128] = [[Color::Black.rgb(); 128]; 128];
pub static mut INPUT: Input = Input::new();
pub static mut FRAME_FN: Option<FrameFn> = None;
static mut RENDERER: Renderer = Renderer;

pub unsafe fn pixel(x: u8, y: u8, color: Color) {
    DISPLAY[y as usize][x as usize] = color.rgb();
}

pub unsafe fn clear(color: Color) {
    DISPLAY = [[color.rgb(); 128]; 128];
}

#[no_mangle]
unsafe extern fn frame() {
    FRAME_FN.unwrap()(&RENDERER, &INPUT);
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