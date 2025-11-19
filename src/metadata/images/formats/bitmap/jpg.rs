use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::metadata::images::ImageMetadata;

pub fn parse(f: &mut File) -> io::Result<ImageMetadata> {
    f.seek(SeekFrom::Start(0))?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    if buf.len() < 2 || &buf[0..2] != [0xFF, 0xD8] {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a JPEG file"));
    }

    // Extract dimensions from SOF0 marker (0xFFC0)
    let mut width = None;
    let mut height = None;
    let mut i = 0;
    while i + 5 < buf.len() {
        if buf[i] == 0xFF && buf[i + 1] == 0xC0 {
            height = Some(u16::from_be_bytes([buf[i + 3], buf[i + 4]]) as u32);
            width = Some(u16::from_be_bytes([buf[i + 5], buf[i + 6]]) as u32);
            break;
        }
        i += 1;
    }

    Ok(ImageMetadata {
        title: None,
        dimensions: width.zip(height),
        color_depth: Some(24),
        format: Some("jpg".to_string()),
    })
}
