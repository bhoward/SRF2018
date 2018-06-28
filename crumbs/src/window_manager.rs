use lfb::Lfb;
use colors::*;
use window::*;

use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::boxed::PinBox;

pub struct WindowManager {
    bg_color: u32,
    lfb: PinBox<Lfb>,
    windows: Vec<Window>,
}

impl WindowManager {
    pub fn new(lfb: PinBox<Lfb>) -> WindowManager {
        let win_man = WindowManager {bg_color: BG_COLOR, lfb, windows: Vec::new()};

        win_man.fill_bg();

        win_man
    }

    pub fn fill_bg(&self) {
        self.lfb.rect(0, 0, self.lfb.width - 1, self.lfb.height - 1, self.bg_color);
    }

    pub fn show(&self, windows: Vec<Window>) {
        for window in windows{
            self.draw_window(&window);
        }
    }

    fn draw_window(&self, window: &Window) {
        self.lfb.rect(window.x, window.y, window.width, TITLE_BAR_HEIGHT, TITLE_BAR_COLOR);
        self.lfb.rect(window.x, window.y + TITLE_BAR_HEIGHT, window.width, window.height, WHITE_PIXEL);

        self.lfb.print(window.x, window.y, window.title.as_str(), BLACK_PIXEL);
    }
}

