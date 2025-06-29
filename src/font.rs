use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Clone)]
pub struct Font {
    pub width: u8,
    pub height: u8,
    pub count: u8,
    glyphs: Box<[Glyph]>,
}

#[derive(Clone)]
pub struct Glyph {
    pub data: Option<Box<[u8]>>,
}

impl Font {
    pub fn from_bytes(bytes: &[u8]) -> Option<Font> {
        if bytes.len() < 3 {
            return None;
        }

        let width = bytes[0];
        let height = bytes[1];
        let count = bytes[2];

        let glyph_size = width * height;

        if bytes.len() - 3 < glyph_size as usize * count as usize {
            return None;
        }

        let mut offset: usize = 3;
        let mut glyphs = Vec::with_capacity(count as usize);

        while offset < bytes.len() {
            glyphs.push(Glyph {
                data: Some(
                    bytes[offset..offset + glyph_size as usize]
                        .to_vec()
                        .into_boxed_slice(),
                ),
            });
            offset += glyph_size as usize;
        }

        return Some(Font {
            width,
            height,
            count,
            glyphs: glyphs.into_boxed_slice(),
        });
    }

    pub fn retrieve_glyph(&self, idx: u8) -> Option<&Glyph> {
        if idx >= self.count {
            return None;
        }

        return Some(&self.glyphs[idx as usize]);
    }
}
