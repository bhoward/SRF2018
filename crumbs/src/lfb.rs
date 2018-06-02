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

use core::sync::atomic::{compiler_fence, Ordering};

#[derive(Debug)]
pub enum LfbError {
    MailboxError,
}

const BLACK_PIXEL: u32 = 0x0000_0000;
const WHITE_PIXEL: u32 = 0x00FF_FFFF;

pub struct Lfb { // TODO change these types
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    lfb: *mut u32,
    font: Font,
}

impl Lfb {
    pub fn new() -> Result<Lfb, LfbError> {
        let mut mbox = mbox::Mbox::new();

        mbox.buffer[0] = 35 * 4;
        mbox.buffer[1] = mbox::REQUEST;

        mbox.buffer[2] = 0x48003; // set physical width/height
        mbox.buffer[3] = 8;
        mbox.buffer[4] = 8;
        mbox.buffer[5] = 1024; // FrameBufferInfo.width
        mbox.buffer[6] = 768;  // FrameBufferInfo.height

        mbox.buffer[7] = 0x48004; // set virtual width/height
        mbox.buffer[8] = 8;
        mbox.buffer[9] = 8;
        mbox.buffer[10] = 1024; // FrameBufferInfo.virtual_width
        mbox.buffer[11] = 768;  // FrameBufferInfo.virtual_height

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
        mbox.buffer[24] = 1; // RGB

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
        let lfb = (mbox.buffer[28] & 0x3FFF_FFFF) as *mut u32;
        let font = Font::new();

        Ok(Lfb { width, height, pitch, lfb, font })
   }

    pub fn print(&self, x: u32, y: u32, msg: &str) {
        let mut x = x;
        let mut y = y;

        // TODO check bounds on x and y
        for c in msg.chars() {
            if c == '\n' {
                x = 0;
                y += 1;
            } else if c == '\r' {
                x = 0;
            } else {
                self.print_char(x, y, c);
                x += 1;
            }
        }
    }

    pub fn print_char(&self, x: u32, y: u32, c: char) {
        let f = &self.font;
        let glyph = f.get_glyph(c);
        let offs = y * f.height * self.pitch + x * (f.width + 1) * 4;

        for row in 0 .. f.height {
            let line = offs + row * self.pitch;

            for col in 0 .. f.width {
                let pixel = line + col * 4;

                unsafe {
                    *self.lfb.offset(pixel as isize) =
                        if glyph.bit_at(row, col) {
                            WHITE_PIXEL
                         } else {
                             BLACK_PIXEL
                         };
                }
            }
        }
    }

    pub fn line(&self) {
        // TODO this is just for testing right now...
        for i in 0 .. 100 {
            unsafe { *self.lfb.offset(i as isize) = WHITE_PIXEL };
        }

        let loc = self.width * 10;

        for i in loc .. loc + 100 {
            unsafe { *self.lfb.offset(i as isize) = WHITE_PIXEL };
        }
    }
}


