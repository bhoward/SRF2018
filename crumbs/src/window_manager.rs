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

        self.lfb.print(window.x + 1, window.y, window.title.as_str(), BLACK_PIXEL);
    }

    // This is purely an experiment...
    pub fn test(&self) {
        self.lfb.line(160, 100, 200, 100, BLACK_PIXEL);
        self.lfb.line(200, 100, 240, 140, BLACK_PIXEL);
        self.lfb.line(240, 140, 240, 180, BLACK_PIXEL);
        self.lfb.line(240, 180, 200, 220, BLACK_PIXEL);
        self.lfb.line(200, 220, 160, 220, BLACK_PIXEL);
        self.lfb.line(160, 220, 120, 180, BLACK_PIXEL);
        self.lfb.line(120, 180, 120, 140, BLACK_PIXEL);
        self.lfb.line(120, 140, 160, 100, BLACK_PIXEL);
        self.lfb.line(120, 140, 240, 180, BLACK_PIXEL);
        self.lfb.line(120, 180, 240, 140, BLACK_PIXEL);
        self.lfb.line(160, 100, 200, 220, BLACK_PIXEL);
        self.lfb.line(200, 100, 160, 220, BLACK_PIXEL);

        self.lfb.print(401, 316, "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F", BLACK_PIXEL);
        self.lfb.print(401, 331, "\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F", BLACK_PIXEL);
        self.lfb.print(401, 346, "\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F", BLACK_PIXEL);
        self.lfb.print(401, 361, "\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3A\x3B\x3C\x3D\x3E\x3F", BLACK_PIXEL);
        self.lfb.print(401, 376, "\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F", BLACK_PIXEL);
        self.lfb.print(401, 391, "\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F", BLACK_PIXEL);
        self.lfb.print(401, 406, "\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6A\x6B\x6C\x6D\x6E\x6F", BLACK_PIXEL);
        self.lfb.print(401, 421, "\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F", BLACK_PIXEL);
   }
}

