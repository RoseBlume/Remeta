use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::metadata::images::ImageMetadata;

pub fn parse(f: &mut File) -> io::Result<ImageMetadata> {
    f.seek(SeekFrom::Start(0))?;

    let mut header = [0u8; 8];
    f.read_exact(&mut header)?;

    let le = match &header[0..2] {
        b"II" => true,
        b"MM" => false,
        _ => return Ok(ImageMetadata::default()),
    };

    let read_u16 = |b: &[u8]| if le { u16::from_le_bytes(b.try_into().unwrap()) } else { u16::from_be_bytes(b.try_into().unwrap()) };
    let read_u32 = |b: &[u8]| if le { u32::from_le_bytes(b.try_into().unwrap()) } else { u32::from_be_bytes(b.try_into().unwrap()) };

    let ifd_offset = read_u32(&header[4..8]) as u64;
    f.seek(SeekFrom::Start(ifd_offset))?;

    let mut count_bytes = [0u8; 2];
    f.read_exact(&mut count_bytes)?;
    let entry_count = read_u16(&count_bytes);

    let mut width = None;
    let mut height = None;
    let mut depth = None;

    for _ in 0..entry_count {
        let mut entry = [0u8; 12];
        f.read_exact(&mut entry)?;

        let tag = read_u16(&entry[0..2]);
        let value_offset = read_u32(&entry[8..12]);

        match tag {
            256 => width = Some(value_offset),
            257 => height = Some(value_offset),
            258 => depth = Some(value_offset as u8),
            _ => {}
        }
    }

    Ok(ImageMetadata {
        title: None,
        dimensions: match (width, height) {
            (Some(w), Some(h)) => Some((w, h)),
            _ => None,
        },
        color_depth: depth,
        format: Some("tiff".to_string()),
    })
}
