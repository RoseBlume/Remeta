use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::metadata::images::ImageMetadata;

pub fn parse(f: &mut File) -> io::Result<ImageMetadata> {
    f.seek(SeekFrom::Start(0))?;
    let mut buf = [0u8; 24]; // enough to read IHDR chunk
    f.read_exact(&mut buf)?;

    if &buf[0..8] != &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a PNG file"));
    }

    // IHDR chunk starts at byte 8
    let width = u32::from_be_bytes(buf[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(buf[20..24].try_into().unwrap());

    Ok(ImageMetadata {
        title: None,
        dimensions: Some((width, height)),
        color_depth: Some(24),
        format: Some("png".to_string()),
    })
}
