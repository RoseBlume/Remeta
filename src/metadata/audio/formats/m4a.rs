use std::fs::File;
use std::io::{self, Read};

use crate::SongMetadata;
use crate::helpers;

pub fn parse(f: &mut File) -> io::Result<SongMetadata> {
    let mut meta = SongMetadata::default();
    let mut data = Vec::new();
    f.read_to_end(&mut data)?;
    let mut i = 0;
    while i + 8 <= data.len() {
        let size = u32::from_be_bytes(data[i..i + 4].try_into().unwrap()) as usize;
        if size < 8 || i + size > data.len() {
            break;
        }
        let atom = &data[i + 4..i + 8];
        if atom == b"\xa9nam" {
            meta.title = helpers::extract_m4a_text(&data[i + 8..i + size]);
        } else if atom == b"\xa9ART" {
            meta.artist = helpers::extract_m4a_text(&data[i + 8..i + size]);
        } else if atom == b"\xa9alb" {
            meta.album = helpers::extract_m4a_text(&data[i + 8..i + size]);
        } else if atom == b"\xa9gen" {
            meta.genre = helpers::extract_m4a_text(&data[i + 8..i + size]);
        }
        i += size;
    }
    Ok(meta)
}