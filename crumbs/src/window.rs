use lfb::Lfb;

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

    pub fn show(&self, lfb: Lfb) {
        lfb.rect(self.x, self.y, self.width, self.height);
    }
}