use std::io::{BufRead, Seek, SeekFrom};

use crate::{Endian, ImageError, ImageResult, ImageSize};
use crate::utils::read_u16;

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    let mut marker = [0; 2];
    let mut depth = 0i32;

    reader.seek(SeekFrom::Start(2))?;

    loop {
        reader.read_exact(&mut marker)?;

        if marker[0] != 0xFF {
            return Err(ImageError::CorruptedImage);
        }

        let page = marker[1];

        if (0xC0..=0xC3).contains(&page)
            || (0xC5..=0xC7).contains(&page)
            || (0xC9..=0xCB).contains(&page)
            || (0xCD..=0xCF).contains(&page)
        {
            if depth == 0 {
                reader.seek(SeekFrom::Current(3))?;
                break;
            }
        } else if page == 0xD8 {
            depth += 1;
        } else if page == 0xD9 {
            depth -= 1;
            if depth < 0 {
                return Err(ImageError::CorruptedImage);
            }
        }

        let page_size = read_u16(reader, &Endian::Big)? as i64;
        reader.seek(SeekFrom::Current(page_size - 2))?;
    }

    Ok(ImageSize {
        height: read_u16(reader, &Endian::Big)? as usize,
        width: read_u16(reader, &Endian::Big)? as usize,
    })
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"\xFF\xD8\xFF")
}