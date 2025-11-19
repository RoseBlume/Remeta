use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::metadata::images::ImageMetadata;

pub fn parse(f: &mut File) -> io::Result<ImageMetadata> {
    f.seek(SeekFrom::Start(0))?;

    let mut header = [0u8; 26];
    f.read_exact(&mut header)?;

    // "BM" already matched in detect function
    // Next 4 bytes = file size
    // Next 4 bytes = reserved
    // Next 4 bytes = pixel array offset (unused here)
    // Next 4 bytes = DIB header size (should be >= 40)

    let dib_header_size = u32::from_le_bytes(header[14..18].try_into().unwrap());

    if dib_header_size < 40 {
        return Ok(ImageMetadata {
            title: None,
            dimensions: None,
            color_depth: None,
            format: Some("bmp".into()),
        });
    }

    let width = i32::from_le_bytes(header[18..22].try_into().unwrap()).abs() as u32;
    let height = i32::from_le_bytes(header[22..26].try_into().unwrap()).abs() as u32;

    // Read bits per pixel
    let mut bpp_bytes = [0u8; 2];
    f.seek(SeekFrom::Start(28))?;
    f.read_exact(&mut bpp_bytes)?;
    let depth = u16::from_le_bytes(bpp_bytes) as u8;

    Ok(ImageMetadata {
        title: None,
        dimensions: Some((width, height)),
        color_depth: Some(depth),
        format: Some("bmp".to_string()),
    })
}
