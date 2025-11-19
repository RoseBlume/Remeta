use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use crate::SongMetadata;

pub fn parse(f: &mut File) -> io::Result<SongMetadata> {
    let mut meta = SongMetadata::default();
    f.seek(SeekFrom::Start(12))?;

    let mut buf = [0u8; 8];
    while f.read(&mut buf)? == 8 {
        let chunk_id = &buf[0..4];
        let chunk_size = u32::from_le_bytes(buf[4..8].try_into().unwrap()) as u64;
        let next = f.seek(SeekFrom::Current(0))? + chunk_size;

        if chunk_id == b"LIST" {
            // Read list type (INFO or others)
            let mut list_type = [0u8; 4];
            f.read_exact(&mut list_type)?;
            if &list_type == b"INFO" {
                let mut remaining = chunk_size - 4;
                while remaining >= 8 {
                    let mut sub_header = [0u8; 8];
                    if f.read(&mut sub_header)? != 8 {
                        break;
                    }
                    let sub_id = &sub_header[0..4];
                    let sub_size =
                        u32::from_le_bytes(sub_header[4..8].try_into().unwrap()) as usize;

                    let mut data = vec![0u8; sub_size];
                    f.read_exact(&mut data)?;
                    let text = String::from_utf8_lossy(&data)
                        .trim_matches(char::from(0))
                        .trim()
                        .to_string();

                    match sub_id {
                        b"IART" => meta.artist = Some(text),
                        b"INAM" => meta.title = Some(text),
                        b"IPRD" => meta.album = Some(text),
                        b"IGNR" => meta.genre = Some(text),
                        _ => {}
                    }

                    remaining = remaining.saturating_sub((8 + sub_size) as u64);
                }
            } else {
                f.seek(SeekFrom::Start(next))?;
            }
        } else {
            f.seek(SeekFrom::Start(next))?;
        }
    }
    Ok(meta)
}