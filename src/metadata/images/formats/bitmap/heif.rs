use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use crate::metadata::images::ImageMetadata;

/// Reads a big-endian u32 from the file.
fn read_u32_be(f: &mut File) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    f.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

/// Reads a 4-byte type code.
fn read_type(f: &mut File) -> io::Result<[u8; 4]> {
    let mut buf = [0u8; 4];
    f.read_exact(&mut buf)?;
    Ok(buf)
}

/// Parse width/height from an 'ispe' (Image Spatial Extents) box.
fn parse_ispe_box(f: &mut File, box_end: u64) -> io::Result<(u32, u32)> {
    // Skip version + flags
    f.seek(SeekFrom::Current(4))?;

    let width = read_u32_be(f)?;
    let height = read_u32_be(f)?;

    // Seek to end of box
    f.seek(SeekFrom::Start(box_end))?;

    Ok((width, height))
}

pub fn parse(f: &mut File) -> io::Result<ImageMetadata> {
    f.seek(SeekFrom::Start(0))?;

    let mut dimensions: Option<(u32, u32)> = None;

    loop {
        // Read box header: size + type
        let size = match read_u32_be(f) {
            Ok(s) => s,
            Err(_) => break, // EOF
        };

        let box_type = read_type(f)?;

        let start = f.stream_position()?;
        let end = if size >= 8 {
            start + (size as u64 - 8)
        } else {
            break;
        };

        match &box_type {
            // 'meta' box: contains item properties
            b"meta" => {
                // Skip version + flags
                f.seek(SeekFrom::Current(4))?;
            }

            // Item Property Container
            b"ipco" => {
                // Scan sub-boxes inside ipco
                while f.stream_position()? < end {
                    let sub_size = read_u32_be(f)?;
                    let sub_type = read_type(f)?;

                    let sub_start = f.stream_position()?;
                    let sub_end = sub_start + (sub_size as u64 - 8);

                    if &sub_type == b"ispe" {
                        if let Ok((w, h)) = parse_ispe_box(f, sub_end) {
                            dimensions = Some((w, h));
                        }
                    } else {
                        f.seek(SeekFrom::Start(sub_end))?;
                    }
                }
            }

            _ => {
                f.seek(SeekFrom::Start(end))?;
            }
        }

        if f.stream_position()? >= f.metadata()?.len() {
            break;
        }
    }

    Ok(ImageMetadata {
        title: None,
        dimensions,
        color_depth: Some(24), // HEIF typically stores 8 bits per channel
        format: Some("heif".to_string()),
    })
}
