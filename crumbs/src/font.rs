#[link(name="font", kind="static")]
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
    pub numglyphs: u32,
    pub height: u32,
    pub width: u32,
    glyph_base: *const u8,
    bytes_per_glyph: u32,
}

pub struct Glyph {
    data: *const u8,
    bytes_per_line: u32,
}

impl Font {
    pub fn new() -> Font {
        let psf = unsafe { _binary_font_psf_start as *const Psf };
        let numglyphs = unsafe { (*psf).numglyphs };
        let height = unsafe { (*psf).height };
        let width = unsafe { (*psf).width };

        let headersize = unsafe { (*psf).headersize as isize };
        let glyph_base = unsafe { (psf as *const u8).offset(headersize) };
        let bytes_per_glyph = unsafe { (*psf).bytes_per_glyph };

        Font { numglyphs, height, width, glyph_base, bytes_per_glyph }
    }

    pub fn get_glyph(&self, c: char) -> Glyph {
        let n = c as u32;

        if n >= self.numglyphs {
            panic!("character out of range");
        }

        let glyph_offset = (n * self.bytes_per_glyph) as isize;
        let data = unsafe { self.glyph_base.offset(glyph_offset) };
        let bytes_per_line = (self.width + 7) / 8;
        Glyph { data, bytes_per_line }
    }
}

impl Glyph {
    pub fn bit_at(&self, row: u32, col: u32) -> bool {
        let byte_offset = row * self.bytes_per_line + col / 8;
        let mask: u8 = 1 << (7 - col % 8); // TODO is this correct for width not a multiple of 8?
        let byte = unsafe { *self.data.offset(byte_offset as isize) };
        byte & mask == mask
    }
}