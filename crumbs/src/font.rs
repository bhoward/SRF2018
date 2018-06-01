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
    pub data: *const u8,
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

    pub fn get_glyph(&self, n: u8) -> Glyph {
        let n = n as u32;

        if n >= self.numglyphs {
            panic!("character out of range");
        }

        let glyph_offset = (n * self.bytes_per_glyph) as isize;
        let data = unsafe { self.glyph_base.offset(glyph_offset) };
        Glyph { data }
    }
}