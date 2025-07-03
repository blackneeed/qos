use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct Font {
    pub height: u8,
    pub count: u8,
    glyphs: Box<[Glyph]>,
}

pub struct Glyph {
    pub data: Option<Box<[u8]>>,
}

impl Font {
    pub fn from_bytes(bytes: &[u8]) -> Option<Font> {
        if bytes.len() < 2 {
            return None;
        }

        let height = bytes[0];
        let count = bytes[1];

        if bytes.len() - 2 < height as usize * count as usize {
            return None;
        }

        let mut offset: usize = 2;
        let mut glyphs = Vec::with_capacity(count as usize);

        while offset < bytes.len() {
            glyphs.push(Glyph {
                data: Some(
                    bytes[offset..offset + height as usize]
                        .to_vec()
                        .into_boxed_slice(),
                ),
            });
            offset += height as usize;
        }

        Some(Font {
            height,
            count,
            glyphs: glyphs.into_boxed_slice(),
        })
    }

    pub fn retrieve_glyph(&self, idx: u8) -> Option<&Glyph> {
        if idx >= self.count {
            return None;
        }

        Some(&self.glyphs[idx as usize])
    }
}
