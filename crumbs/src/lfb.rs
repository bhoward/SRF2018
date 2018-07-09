/*
 * MIT License
 *
 * Copyright (c) 2018 Brian Howard <bhoward@depauw.edu>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use mbox;
use font::Font;
use colors::*;

use core::sync::atomic::{compiler_fence, Ordering};
use core::cmp::{min, max};

#[derive(Debug)]
pub enum LfbError {
    MailboxError,
}

pub struct Lfb { // TODO change these types
    pub width: u32,
    pub height: u32,
    pub pixels_per_line: u32,
    lfb: *mut u32,
    pub font: Font,
}

impl Lfb {
    pub fn new() -> Result<Lfb, LfbError> {
        let mut mbox = mbox::Mbox::new();

        mbox.buffer[0] = 35 * 4;
        mbox.buffer[1] = mbox::REQUEST;

        mbox.buffer[2] = 0x48003; // set physical width/height
        mbox.buffer[3] = 8;
        mbox.buffer[4] = 8;
        mbox.buffer[5] = 800; // FrameBufferInfo.width
        mbox.buffer[6] = 480;  // FrameBufferInfo.height

        mbox.buffer[7] = 0x48004; // set virtual width/height
        mbox.buffer[8] = 8;
        mbox.buffer[9] = 8;
        mbox.buffer[10] = 800; // FrameBufferInfo.virtual_width
        mbox.buffer[11] = 480;  // FrameBufferInfo.virtual_height

        mbox.buffer[12] = 0x48009; // set virtual offset
        mbox.buffer[13] = 8;
        mbox.buffer[14] = 8;
        mbox.buffer[15] = 0; // FrameBufferInfo.x_offset
        mbox.buffer[16] = 0; // FrameBufferInfo.y_offset

        mbox.buffer[17] = 0x48005; // set depth
        mbox.buffer[18] = 4;
        mbox.buffer[19] = 4;
        mbox.buffer[20] = 32; // FrameBufferInfo.depth

        mbox.buffer[21] = 0x48006; // set pixel order
        mbox.buffer[22] = 4;
        mbox.buffer[23] = 4;
        mbox.buffer[24] = 0; // BGR -- Blue is first byte (little-endian)
        // using BGR works on both RPi and QEMU;
        // using 1 for RGB switches order on QEMU but not on actual RPi

        mbox.buffer[25] = 0x40001; // get framebuffer
        mbox.buffer[26] = 8;
        mbox.buffer[27] = 8;
        mbox.buffer[28] = 4096; // FrameBufferInfo.pointer
        mbox.buffer[29] = 0;    // FrameBufferInfo.size

        mbox.buffer[30] = 0x40008; // get pitch
        mbox.buffer[31] = 4;
        mbox.buffer[32] = 4;
        mbox.buffer[33] = 0; // FrameBufferInfo.pitch

        mbox.buffer[34] = mbox::tag::LAST;

        // Insert a compiler fence that ensures that all stores to the
        // mbox buffer are finished before the GPU is signaled (which is
        // done by a store operation as well).
        compiler_fence(Ordering::Release);

        // send the message to the GPU and receive answer
        if mbox.call(mbox::channel::PROP).is_err() || mbox.buffer[20] != 32 || mbox.buffer[28] == 0 {
            return Err(LfbError::MailboxError);
        }

        let width = mbox.buffer[5];
        let height = mbox.buffer[6];
        let pitch = mbox.buffer[33];
        let pixels_per_line = pitch / 4;
        let lfb = (mbox.buffer[28] & 0x3FFF_FFFF) as *mut u32;
        let font = Font::new();

        Ok(Lfb { width, height, pixels_per_line, lfb, font })
   }

    pub fn print(&self, x: u32, y: u32, msg: &str, color: u32) {
        let mut x = x;

        // TODO check bounds on x and y
        for c in msg.chars() {
            self.print_char(x, y, c, color);
            x += self.font.width + 1;
        }
    }

    pub fn print_char(&self, x: u32, y: u32, c: char, color: u32) {
        let glyph = self.font.get_glyph(c);

        for row in 0 .. self.font.height {
            for col in 0 .. self.font.width {
                if glyph.bit_at(row, col) {
                    self.set_pixel(x + col, y + row, color, 1.0);
                }
            }
        }
    }

    fn set_pixel(&self, x: u32, y: u32, color: u32, alpha: f64) {
        let pixel: isize = (y * self.pixels_per_line + x) as isize;
        if alpha == 1.0 {
            unsafe {
                *self.lfb.offset(pixel) = color;
            }
        } else {
            let curr_color = unsafe { *self.lfb.offset(pixel) };
            let curr_red = curr_color & RED_PIXEL;
            let curr_green = curr_color & GREEN_PIXEL;
            let curr_blue = curr_color & BLUE_PIXEL;
            let red = color & RED_PIXEL;
            let green = color & GREEN_PIXEL;
            let blue = color & BLUE_PIXEL;
            let new_red = (curr_red as f64) * (1.0 - alpha) + (red as f64) * alpha;
            let new_green = (curr_green as f64) * (1.0 - alpha) + (green as f64) * alpha;
            let new_blue = (curr_blue as f64) * (1.0 - alpha) + (blue as f64) * alpha;
            let new_color =
                  (new_red as u32) & RED_PIXEL
                | (new_green as u32) & GREEN_PIXEL
                | (new_blue as u32) & BLUE_PIXEL;
            unsafe {
                *self.lfb.offset(pixel) = new_color;
            }
        }
    }

    pub fn line(&self, x0: u32, y0: u32, x1: u32, y1: u32, color: u32) {
        let dx = (x1 as i32) - (x0 as i32);
        let dy = (y1 as i32) - (y0 as i32);
        if dx == 0 {
            // vertical line
            let top = min(y0, y1);
            let bot = max(y0, y1);
            for y in top .. bot {
                self.set_pixel(x0, y, color, 1.0);
            }
        } else if dy == 0 {
            // horizontal line
            let left = min(x0, x1);
            let right = max(x0, x1);
            for x in left .. right {
                self.set_pixel(x, y0, color, 1.0);
            }
        } else if dx.abs() == dy.abs() {
            // diagonal line
            let left = min(x0, x1);
            let right = max(x0, x1);
            let y_step = dy / dx; // +1/-1
            let mut y = if left == x0 { y0 } else { y1 };
            for x in left .. right {
                self.set_pixel(x, y, color, 1.0);
                y = ((y as i32) + y_step) as u32;
            }
        } else if dx.abs() > dy.abs() {
            // slope < 1
            let left = min(x0, x1);
            let right = max(x0, x1);
            let y_step = (dy as f64) / (dx as f64);
            let mut y = if left == x0 { y0 as f64 } else { y1 as f64 };
            for x in left .. right {
                let y_int = y as u32;
                let frac = y - (y_int as f64);
                if frac == 0.0 {
                    self.set_pixel(x, y_int, color, 1.0);
                } else {
                    self.set_pixel(x, y_int, color, 1.0 - frac);
                    self.set_pixel(x, y_int + 1, color, frac);
                }
                y += y_step;
            }
        } else {
            // slope > 1
            let top = min(y0, y1);
            let bot = max(y0, y1);
            let x_step = (dx as f64) / (dy as f64);
            let mut x = if top == y0 { x0 as f64 } else { x1 as f64 };
            for y in top .. bot {
                let x_int = x as u32;
                let frac = x - (x_int as f64);
                if frac == 0.0 {
                    self.set_pixel(x_int, y, color, 1.0);
                } else {
                    self.set_pixel(x_int, y, color, 1.0 - frac);
                    self.set_pixel(x_int + 1, y, color, frac);
                }
                x += x_step;
            }
        }
    }

    pub fn rect(&self, x: u32, y: u32, width: u32, length: u32, color: u32) {
        for curr_y in y .. (y + length) {
            for curr_x in x .. (x + width) {
                self.set_pixel(curr_x, curr_y, color, 1.0);
            }
        }
    }

    pub fn cool_rect(&self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        for curr_y in y .. (y + height) {
            for curr_x in x .. (x + width) {
                let alpha = 1.0 - 0.75 * (((curr_y - y) * (curr_x - x)) as f64) / ((height * width) as f64);
                self.set_pixel(curr_x, curr_y, color, alpha);
            }
        }
    }
}
