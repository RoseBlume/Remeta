use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::metadata::images::ImageMetadata;

pub fn parse(f: &mut File) -> io::Result<ImageMetadata> {
    f.seek(SeekFrom::Start(0))?;
    let mut buf = [0u8; 10]; // enough to read Logical Screen Descriptor
    f.read_exact(&mut buf)?;

    if &buf[0..3] != b"GIF" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a GIF file"));
    }

    let width = u16::from_le_bytes([buf[6], buf[7]]) as u32;
    let height = u16::from_le_bytes([buf[8], buf[9]]) as u32;

    Ok(ImageMetadata {
        title: None,
        dimensions: Some((width, height)),
        color_depth: Some(8), // typical for GIF
        format: Some("gif".to_string()),
    })
}
