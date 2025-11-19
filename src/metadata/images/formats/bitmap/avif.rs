use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use crate::metadata::images::ImageMetadata;

/// Parse an AVIF file.
/// AVIF is a HEIF container using AV1-encoded images.
/// We inspect ftyp → meta → iprp → ispe boxes.
///
/// Layout:
///   - ftyp
///   - meta
///       - iprp
///           - ipco
///               - ispe (width/height)
pub fn parse(f: &mut File) -> io::Result<ImageMetadata> {
    let mut meta = ImageMetadata::default();

    f.seek(SeekFrom::Start(0))?;

    loop {
        let box_start = f.stream_position()?;
        let mut header = [0u8; 8];

        if f.read(&mut header)? < 8 {
            break;
        }

        let size = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
        let name = &header[4..8];

        if size < 8 {
            break;
        }
        let payload = size as u64 - 8;

        match name {
            b"ftyp" => {
                let mut buf = vec![0u8; payload.min(64) as usize];
                f.read_exact(&mut buf)?;

                if !buf.windows(4).any(|w| w == b"avif" || w == b"avis") {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Not AVIF"));
                }
                meta.format = Some("AVIF".to_string());
            }

            b"meta" => {
                parse_meta_box(f, box_start, size, &mut meta)?;
            }

            _ => {
                f.seek(SeekFrom::Start(box_start + size as u64))?;
            }
        }
    }

    Ok(meta)
}

fn parse_meta_box(
    f: &mut File,
    meta_start: u64,
    meta_size: u32,
    meta: &mut ImageMetadata
) -> io::Result<()> {

    let mut pos = meta_start + 12;
    let meta_end = meta_start + meta_size as u64;

    while pos + 8 <= meta_end {
        f.seek(SeekFrom::Start(pos))?;

        let mut header = [0u8; 8];
        f.read_exact(&mut header)?;
        let size = u32::from_be_bytes(header[0..4].try_into().unwrap()) as u64;
        let name = &header[4..8];

        if size < 8 {
            break;
        }

        if name == b"iprp" {
            parse_iprp(f, pos, size, meta)?;
        }

        pos += size;
    }

    Ok(())
}

fn parse_iprp(
    f: &mut File,
    iprp_start: u64,
    iprp_size: u64,
    meta: &mut ImageMetadata
) -> io::Result<()> {

    let mut pos = iprp_start + 8;
    let iprp_end = iprp_start + iprp_size;

    while pos + 8 <= iprp_end {
        f.seek(SeekFrom::Start(pos))?;

        let mut h = [0u8; 8];
        f.read_exact(&mut h)?;
        let size = u32::from_be_bytes([h[0], h[1], h[2], h[3]]) as u64;
        let name = &h[4..8];

        if size < 8 {
            break;
        }

        if name == b"ipco" {
            parse_ipco(f, pos, size, meta)?;
        }

        pos += size;
    }

    Ok(())
}

fn parse_ipco(
    f: &mut File,
    ipco_start: u64,
    ipco_size: u64,
    meta: &mut ImageMetadata
) -> io::Result<()> {

    let mut pos = ipco_start + 8;
    let ipco_end = ipco_start + ipco_size;

    while pos + 8 <= ipco_end {
        f.seek(SeekFrom::Start(pos))?;

        let mut h = [0u8; 8];
        f.read_exact(&mut h)?;
        let size = u32::from_be_bytes(h[0..4].try_into().unwrap()) as u64;
        let name = &h[4..8];

        if size < 8 {
            break;
        }

        if name == b"ispe" {
            parse_ispe(f, pos, size, meta)?;
        }

        pos += size;
    }

    Ok(())
}

fn parse_ispe(
    f: &mut File,
    box_start: u64,
    box_size: u64,
    meta: &mut ImageMetadata
) -> io::Result<()> {

    // ispe contents:
    //   version 1 byte
    //   flags   3 bytes
    //   width  (u32)
    //   height (u32)

    if box_size < 20 {
        return Ok(());
    }

    f.seek(SeekFrom::Start(box_start + 8))?;

    let mut buf = [0u8; 12];
    f.read_exact(&mut buf)?;

    let width = u32::from_be_bytes(buf[4..8].try_into().unwrap());
    let height = u32::from_be_bytes(buf[8..12].try_into().unwrap());

    meta.dimensions = Some(((width as u32), (height as u32)));
    // meta.height = Some(height as u64);

    Ok(())
}
