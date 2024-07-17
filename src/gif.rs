use std::io::{BufRead, Seek, SeekFrom};
use crate::{Endian, ImageResult, ImageSize};
use crate::utils::read_u16;

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(6))?;

    Ok(ImageSize {
        width: read_u16(reader, &Endian::Little)? as usize,
        height: read_u16(reader, &Endian::Little)? as usize,
    })
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"GIF8")
}