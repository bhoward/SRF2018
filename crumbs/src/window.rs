use lfb::Lfb;
use colors::*;

const TITLE_BAR_HEIGHT: u32 = 16;

pub struct Window {
    title: &'static str,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Window {
    pub fn new(title: &'static str, x: u32, y: u32, width: u32, height: u32) -> Window {
        Window {title, x, y, width, height}
    }

    pub fn show(&self, lfb: &Lfb) {
        lfb.rect(self.x, self.y, self.width, TITLE_BAR_HEIGHT, TITLE_BAR_COLOR);
        lfb.rect(self.x, self.y + TITLE_BAR_HEIGHT, self.width, self.height, WHITE_PIXEL);

        lfb.print(self.x, self.y, self.title, WHITE_PIXEL);
    }
}