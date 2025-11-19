use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::metadata::images::ImageMetadata;

//
// WEBP FORMAT OVERVIEW
// --------------------
//
// A WebP file starts with:
//
//   "RIFF" <file_size: u32> "WEBP"
//
// Then a chunk, one of:
//   "VP8 " — lossy ANMF frame (Simple)
//   "VP8L" — lossless bitstream
//   "VP8X" — extended (may include alpha, EXIF, XMP, animation)
//
// Each chunk represents dimensions differently.
//

pub fn parse(f: &mut File) -> io::Result<ImageMetadata> {
    f.seek(SeekFrom::Start(0))?;

    let mut riff = [0u8; 12];
    f.read_exact(&mut riff)?;

    // Validate RIFF header
    if &riff[0..4] != b"RIFF" || &riff[8..12] != b"WEBP" {
        return Ok(ImageMetadata {
            format: Some("webp".into()),
            ..Default::default()
        });
    }

    // Now read the next chunk header
    let mut chunk_hdr = [0u8; 8];
    if f.read(&mut chunk_hdr)? < 8 {
        return Ok(ImageMetadata {
            format: Some("webp".into()),
            ..Default::default()
        });
    }

    let chunk_type = &chunk_hdr[0..4];
    let chunk_size = u32::from_le_bytes([
        chunk_hdr[4], chunk_hdr[5], chunk_hdr[6], chunk_hdr[7],
    ]);

    match chunk_type {
        // -------------------------------
        // Lossy VP8 chunk: "VP8 "
        // -------------------------------
        b"VP8 " => parse_vp8(f, chunk_size),

        // -------------------------------
        // Lossless VP8L chunk: "VP8L"
        // -------------------------------
        b"VP8L" => parse_vp8l(f),

        // -------------------------------
        // Extended chunk: "VP8X"
        // -------------------------------
        b"VP8X" => parse_vp8x(f),

        _ => Ok(ImageMetadata {
            format: Some("webp".into()),
            ..Default::default()
        }),
    }
}

//
// ------------ VP8 (lossy) ------------
//
fn parse_vp8(f: &mut File, _chunk_size: u32) -> io::Result<ImageMetadata> {
    // VP8 lossy uses a frame header located 10 bytes after chunk start:
    //
    //  3 bytes: Start Code (0x9D 0x01 0x2A)
    //  2 bytes: Width  (14 bits used)
    //  2 bytes: Height (14 bits used)
    //

    let mut hdr = [0u8; 10];
    f.read_exact(&mut hdr)?;

    if hdr[0..3] != [0x9D, 0x01, 0x2A] {
        return Ok(ImageMetadata {
            format: Some("webp".into()),
            ..Default::default()
        });
    }

    let width = u16::from_le_bytes([hdr[3], hdr[4]]) & 0x3FFF;
    let height = u16::from_le_bytes([hdr[5], hdr[6]]) & 0x3FFF;

    Ok(ImageMetadata {
        dimensions: Some((width as u32, height as u32)),
        color_depth: Some(24),
        format: Some("webp".into()),
        ..Default::default()
    })
}

//
// ------------ VP8L (lossless) ------------
//
fn parse_vp8l(f: &mut File) -> io::Result<ImageMetadata> {
    //
    // VP8L header (first 5 bytes after the chunk header):
    //
    // Byte 0: Signature = 0x2F
    // Bytes 1–4 encode width/height:
    //
    // width  = (bits 0..14)  + 1
    // height = (bits 15..29) + 1
    //

    let mut hdr = [0u8; 5];
    f.read_exact(&mut hdr)?;

    if hdr[0] != 0x2F {
        return Ok(ImageMetadata {
            format: Some("webp".into()),
            ..Default::default()
        });
    }

    let bits = u32::from_le_bytes([hdr[1], hdr[2], hdr[3], hdr[4]]);

    let width  = (bits & 0x3FFF) + 1;
    let height = ((bits >> 14) & 0x3FFF) + 1;

    Ok(ImageMetadata {
        dimensions: Some((width, height)),
        color_depth: Some(24),
        format: Some("webp".into()),
        ..Default::default()
    })
}

//
// ------------ VP8X (extended) ------------
//
fn parse_vp8x(f: &mut File) -> io::Result<ImageMetadata> {
    //
    // VP8X structure:
    //
    // 1 byte  : Feature flags
    // 3 bytes : Reserved
    // 3 bytes : Canvas Width  (minus 1)
    // 3 bytes : Canvas Height (minus 1)
    //

    let mut hdr = [0u8; 10];
    f.read_exact(&mut hdr)?;

    let w = u32::from_le_bytes([hdr[4], hdr[5], hdr[6], 0x00]) + 1;
    let h = u32::from_le_bytes([hdr[7], hdr[8], hdr[9], 0x00]) + 1;

    Ok(ImageMetadata {
        dimensions: Some((w, h)),
        color_depth: Some(24),
        format: Some("webp".into()),
        ..Default::default()
    })
}
