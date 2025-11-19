use std::fs::File;
use std::io;

use crate::ImageMetadata;

#[cfg(any(feature = "avif", feature = "heif"))]
use std::io::{Read, Seek};

#[cfg(any(
    feature = "jpg", 
    feature = "png", 
    feature = "gif", 
    feature = "webp", 
    feature = "ico", 
    feature = "tiff", 
    feature = "bmp", 
    feature = "heif", 
    feature = "avif"
))]

mod bitmap;

#[cfg(any(feature = "svg", feature = "eps", feature = "pdf"))]
mod vector;

pub fn detect_and_parse(header: &[u8], f: &mut File) -> io::Result<ImageMetadata> {
    Ok(match header {
        // JPG
        #[cfg(feature = "jpg")]
        [0xFF, 0xD8, ..] => bitmap::jpg::parse(f)?,

        // PNG
        #[cfg(feature = "png")]
        [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, ..] => bitmap::png::parse(f)?,

        // GIF (87a / 89a)
        #[cfg(feature = "gif")]
        [b'G', b'I', b'F', b'8', b'7' | b'9', b'a', ..] => bitmap::gif::parse(f)?,

        // BMP ("BM")
        #[cfg(feature = "bmp")]
        [0x42, 0x4D, ..] => bitmap::bmp::parse(f)?,

        // TIFF (little endian or big endian magic)
        #[cfg(feature = "tiff")]
        [0x49, 0x49, 0x2A, 0x00, ..] | 
        [0x4D, 0x4D, 0x00, 0x2A, ..] => bitmap::tiff::parse(f)?,

        // ICO ("00 00 01 00")
        #[cfg(feature = "ico")]
        [0x00, 0x00, 0x01, 0x00, ..] => bitmap::ico::parse(f)?,

        // WebP ("RIFF....WEBP")
        #[cfg(feature = "webp")]
        [0x52, 0x49, 0x46, 0x46, _, _, _, _, b'W', b'E', b'B', b'P', ..] => bitmap::webp::parse(f)?,

        // HEIF ("....ftypheic" or similar)
        // Many HEIF files match pattern:
        // 00 00 00 ?? 'f' 't' 'y' 'p' <brand>
        // ISO Base Media File Formats (AVIF, HEIF, HEIC, AVIS)
        #[cfg(any(feature = "avif", feature = "heif"))]
        [0x00, 0x00, 0x00, _, b'f', b't', b'y', b'p', ..] => {
            // Read brand from ftyp box (bytes 8..12)
            let mut brand = [0u8; 4];
            f.seek(std::io::SeekFrom::Start(8))?;
            f.read_exact(&mut brand)?;

            match &brand {
                #[cfg(feature = "avif")]
                b"avif" | b"avis" => bitmap::avif::parse(f)?,

                #[cfg(feature = "heif")]
                b"heic" | b"heix" | b"mif1" | b"msf1" => bitmap::heif::parse(f)?,

                // Unknown brand in ISO BMFF
                _ => return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown ISO-BMFF image type",
                ))
            }
        },
        
        // SVG starts with "<svg" or XML declaration "< ? xml ... >"
        #[cfg(feature = "svg")]
        [b'<', ..] => vector::svg::parse(f)?,

        // PDF: "%PDF" header
        #[cfg(feature = "pdf")]
        [0x25, 0x50, 0x44, 0x46, ..] => vector::pdf::parse(f)?,

        // EPS: "%!PS"
        #[cfg(feature = "eps")]
        [0x25, 0x21, 0x50, 0x53, ..] => vector::eps::parse(f)?,

        _ => ImageMetadata::default(),
    })
}