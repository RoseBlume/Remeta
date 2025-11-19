use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::metadata::images::ImageMetadata;

pub fn parse(f: &mut File) -> io::Result<ImageMetadata> {
    f.seek(SeekFrom::Start(0))?;

    let mut header = [0u8; 6];
    f.read_exact(&mut header)?; 
    // 0–1: reserved (0)
    // 2–3: type (1 = icon)
    // 4–5: count

    let count = u16::from_le_bytes(header[4..6].try_into().unwrap());

    let mut best_w = 0;
    let mut best_h = 0;
    let mut best_bpp = 0;

    for _ in 0..count {
        let mut entry = [0u8; 16];
        f.read_exact(&mut entry)?;

        let w = entry[0] as u32;
        let h = entry[1] as u32;
        let bpp = u16::from_le_bytes(entry[6..8].try_into().unwrap()) as u8;

        // 0 means 256 in ICO format
        let w = if w == 0 { 256 } else { w };
        let h = if h == 0 { 256 } else { h };

        // Keep the largest icon
        if w * h > best_w * best_h {
            best_w = w;
            best_h = h;
            best_bpp = bpp;
        }
    }

    Ok(ImageMetadata {
        title: None,
        dimensions: Some((best_w, best_h)),
        color_depth: Some(best_bpp),
        format: Some("ico".to_string()),
    })
}
