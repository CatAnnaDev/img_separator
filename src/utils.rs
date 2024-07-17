use std::io::{BufRead, Seek};
use std::time::Instant;

use crate::{Endian, ImageResult};

pub fn read_u16<R: BufRead + Seek>(reader: &mut R, endianness: &Endian) -> ImageResult<u16> {
    let mut buf = [0; 2];
    reader.read_exact(&mut buf)?;
    match endianness {
        Endian::Little => Ok(((buf[1] as u16) << 8) | (buf[0] as u16)),
        Endian::Big => Ok(((buf[0] as u16) << 8) | (buf[1] as u16)),
    }
}

pub fn read_u32<R: BufRead + Seek>(reader: &mut R, endianness: &Endian) -> ImageResult<u32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;

    match endianness {
        Endian::Little => Ok(((buf[3] as u32) << 24)
            | ((buf[2] as u32) << 16)
            | ((buf[1] as u32) << 8)
            | (buf[0] as u32)),
        Endian::Big => Ok(((buf[0] as u32) << 24)
            | ((buf[1] as u32) << 16)
            | ((buf[2] as u32) << 8)
            | (buf[3] as u32)),
    }
}

pub fn read_u24<R: BufRead + Seek>(reader: &mut R, endianness: &Endian) -> ImageResult<u32> {
    let mut buf = [0; 3];
    reader.read_exact(&mut buf)?;

    match endianness {
        Endian::Little => Ok(((buf[2] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[0] as u32)),
        Endian::Big => Ok(((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32)),
    }
}

pub fn time_to_human_time(time: Instant) -> String {
    let seconds = time.elapsed().as_secs() % 60;
    let minutes = (time.elapsed().as_secs() / 60) % 60;
    let hours = (time.elapsed().as_secs() / 60) / 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}