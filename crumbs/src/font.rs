//let slice = unsafe { std::slice::from_raw_parts(some_pointer, count_of_items) };

/* PC Screen Font as used by Linux Console */
extern {
    static _binary_font_psf_start : *const u8;
}

// PC Screen Font (ps2)
#[repr(C, packed)]
struct Psf {
    magic: u32,
    version: u32,
    headersize: u32,
    flags: u32,
    numglyphs: u32,
    bytes_per_glyph: u32,
    height: u32,
    width: u32,
}

pub struct Font {
    numglyphs: u32,
    glyph_base: *const u8,
}

impl Font {
    pub fn new() -> Font {
        let psf = unsafe { _binary_font_psf_start as *const Psf };
        let numglyphs = unsafe { (*psf).numglyphs };
        let headersize = unsafe { (*psf).headersize as isize };
        let glyph_base = unsafe { (psf as *const u8).offset(headersize) };

        Font { numglyphs, glyph_base }
    }
}