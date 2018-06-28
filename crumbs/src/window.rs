use lfb::Lfb;
use colors::*;
use window_manager::WindowManager;

use alloc::string::String;
use alloc::boxed::Box;
use alloc::boxed::PinBox;

pub const TITLE_BAR_HEIGHT: u32 = 16;

pub struct Window {
    pub title: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn new(title: &str, x: u32, y: u32, width: u32, height: u32) -> Window {
        Window {title: String::from(title), x, y, width, height}
    }
}