pub const BLACK_PIXEL: u32 = 0x0000_0000;
pub const WHITE_PIXEL: u32 = 0x00FF_FFFF;
pub const RED_PIXEL: u32 =   0x00FF_0000;
pub const GREEN_PIXEL: u32 = 0x0000_FF00;
pub const BLUE_PIXEL: u32 =  0x0000_00FF;

pub const TITLE_BAR_COLOR: u32 = 0x00888888;
pub const BG_COLOR: u32 = 0x004286F4;

pub fn rgb(red: u8, green: u8, blue: u8) -> u32 {
    let red = (red as u32) << 16;
    let green = (green as u32) << 8;
    let blue = blue as u32;
    red | green | blue
}