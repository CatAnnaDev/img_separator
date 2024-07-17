use std::io::{BufRead, Seek, SeekFrom};

use crate::{Endian, ImageResult, ImageSize, utils};

// PNG
pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0x10))?;

    Ok(ImageSize {
        width: utils::read_u32(reader, &Endian::Big)? as usize,
        height: utils::read_u32(reader, &Endian::Big)? as usize,
    })
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"\x89PNG")
}