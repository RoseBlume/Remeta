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
        } else if atom == b"mdhd" {
            // Parse media header to get timescale and duration.
            let content_start = i + 8;
            if content_start + 4 <= i + size {
                let version = data[content_start];
                if version == 1 {
                    // version 1: creation(8) + mod(8) + timescale(4) + duration(8)
                    let timescale_off = content_start + 20;
                    let duration_off = content_start + 24;
                    if timescale_off + 4 <= i + size && duration_off + 8 <= i + size {
                        let timescale = u32::from_be_bytes(
                            data[timescale_off..timescale_off + 4].try_into().unwrap(),
                        );
                        let duration = u64::from_be_bytes(
                            data[duration_off..duration_off + 8].try_into().unwrap(),
                        );
                        if timescale != 0 {
                            meta.duration_ms =
                                Some((duration.saturating_mul(1000)) / (timescale as u64));
                        }
                        else {
                            meta.duration_ms = Some(0);
                        }
                    }
                } 
                else {
                    // version 0: creation(4) + mod(4) + timescale(4) + duration(4)
                    let timescale_off = content_start + 12;
                    let duration_off = content_start + 16;
                    if timescale_off + 4 <= i + size && duration_off + 4 <= i + size {
                        let timescale = u32::from_be_bytes(
                            data[timescale_off..timescale_off + 4].try_into().unwrap(),
                        );
                        let duration = u32::from_be_bytes(
                            data[duration_off..duration_off + 4].try_into().unwrap(),
                        ) as u64;
                        if timescale != 0 {
                            meta.duration_ms = Some((duration.saturating_mul(1000)) / (timescale as u64));
                        }
                        else {
                            meta.duration_ms = Some(0);
                        }
                    }
                }
            }
        }
        i += size;
    }
    Ok(meta)
}


