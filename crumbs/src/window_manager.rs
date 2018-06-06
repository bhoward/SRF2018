use lfb::Lfb;
use colors::*;

pub struct WindowManager {
    bg_color: u32,
}

impl WindowManager {
    pub fn new() -> WindowManager {
        let bg_color = BG_COLOR;

        WindowManager {bg_color}
    }
    pub fn fill_bg(&self, lfb: &Lfb) {
        lfb.rect(0, 0, lfb.width - 1, lfb.height - 1, self.bg_color);
    }
}

