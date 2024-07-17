#![feature(os_str_display)]

use std::{fmt, fs, io};
use std::error::Error;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader, ErrorKind, Seek};
use std::path::{Path, PathBuf};

use crate::ImageType::{Gif, Jpeg, Png, Webp};

mod utils;
mod png;
mod jpg;
mod webp;
mod gif;

#[macro_export]
macro_rules! time_it {
    ($context:literal, $s:block) => {
        let timer = std::time::Instant::now();
        $s
        println!("{}: {}", $context, utils::time_to_human_time(timer));
    };
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if !path.to_str().unwrap().contains("horizontal")&&
                    !path.to_str().unwrap().contains("vertical")&&
                    !path.to_str().unwrap().contains("square")
                {
                    println!("Path: {}", path.display());
                    visit_dirs(&path, cb)?;
                }
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn process(name: &DirEntry) {
    if let Some(parent) = name.path().parent() {
        let horizontal_path = PathBuf::from(&parent).join("horizontal");
        let vertical_path = PathBuf::from(&parent).join("vertical");
        let square_path = PathBuf::from(&parent).join("square");
        let _ = fs::create_dir_all(&horizontal_path);
        let _ = fs::create_dir_all(&vertical_path);
        let _ = fs::create_dir_all(&square_path);
        let p = name.path();
        match size(&p) {
            Ok(e) => {
                if e.width > e.height {
                    let _ = fs::copy(&p, &horizontal_path.join(&p.file_name().unwrap()));
                    let _ = fs::remove_file(&p);
                } else if e.width == e.height {
                    let _ = fs::copy(&p, &square_path.join(&p.file_name().unwrap()));
                    let _ = fs::remove_file(&p);
                } else {
                    let _ = fs::copy(&p, &vertical_path.join(&p.file_name().unwrap()));
                    let _ = fs::remove_file(&p);
                }
            }
            Err(e) => eprintln!("Error! {e}\n {}", p.display())
        }
    }
}


// only jpeg jpg webp png gif image
fn main() -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from("/Users/blap/Downloads/test");
    time_it!("Global time:", {visit_dirs(&path, &process).expect("euh ?")});


    println!("End");
    Ok(())
}

fn size<P: AsRef<Path>>(path: P) -> ImageResult<ImageSize> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    dispatch_header(&mut reader)
}


fn dispatch_header<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    match image_type_select(reader)? {
        Jpeg => jpg::size(reader),
        Png => png::size(reader),
        Webp => webp::size(reader),
        Gif => gif::size(reader),
    }
}

enum ImageType {
    Png,
    Jpeg,
    Webp,
    Gif,
}


fn image_type_select<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageType> {
    let mut header = [0; 12];
    reader.read_exact(&mut header)?;

    if header.len() < 2 {
        return Err(
            io::Error::new(ErrorKind::UnexpectedEof, "Not enough data").into(),
        );
    }

    if jpg::matches(&header) {
        return Ok(Jpeg);
    }

    if png::matches(&header) {
        return Ok(Png);
    }

    if webp::matches(&header) {
        return Ok(Webp);
    }
    if gif::matches(&header) {
        return Ok(Gif);
    }

    Err(ImageError::NotSupported)
}


struct ImageSize {
    height: usize,
    width: usize,
}

enum Endian {
    Little,
    Big,
}

#[derive(Debug)]
enum ImageError {
    NotSupported,
    CorruptedImage,
    IoError(std::io::Error),
}

impl Error for ImageError {}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ImageError::*;
        match self {
            NotSupported => f.write_str("Could not decode image"),
            CorruptedImage => f.write_str("Hit end of file before finding size"),
            IoError(error) => error.fmt(f),
        }
    }
}

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> ImageError {
        ImageError::IoError(err)
    }
}

type ImageResult<T> = Result<T, ImageError>;


