use lfb::Lfb;
use colors::*;
use window::*;

use alloc::vec::Vec;
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
        self.lfb.cool_rect(window.x, window.y + TITLE_BAR_HEIGHT, window.width, window.height, WHITE_PIXEL);

        self.lfb.print(window.x, window.y, window.title.as_str(), BLACK_PIXEL);
    }

    // This is purely an experiment...
    pub fn test(&self) {
        self.lfb.line(160, 100, 200, 100, RED_PIXEL);
        self.lfb.line(200, 100, 240, 140, GREEN_PIXEL);
        self.lfb.line(240, 140, 240, 180, BLUE_PIXEL);
        self.lfb.line(240, 180, 200, 220, BLACK_PIXEL);
        self.lfb.line(200, 220, 160, 220, RED_PIXEL);
        self.lfb.line(160, 220, 120, 180, GREEN_PIXEL);
        self.lfb.line(120, 180, 120, 140, BLUE_PIXEL);
        self.lfb.line(120, 140, 160, 100, BLACK_PIXEL);
        self.lfb.line(120, 140, 240, 180, RED_PIXEL);
        self.lfb.line(120, 180, 240, 140, GREEN_PIXEL);
        self.lfb.line(160, 100, 200, 220, BLUE_PIXEL);
        self.lfb.line(200, 100, 160, 220, BLACK_PIXEL);
    }
}

