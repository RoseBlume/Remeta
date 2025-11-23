use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::SongMetadata;

pub fn parse(f: &mut File) -> io::Result<SongMetadata> {
    let mut meta = SongMetadata::default();

    f.seek(SeekFrom::Start(12))?; // skip RIFF/WAVE header

    let mut buf = [0u8; 8];
    let mut byte_rate: Option<u32> = None;
    let mut data_size: Option<u32> = None;

    // reusable buffer for chunks
    // let chunk_buf = [0u8; 1024];

    while f.read(&mut buf)? == 8 {
        let chunk_id = &buf[0..4];
        let chunk_size = u32::from_le_bytes(buf[4..8].try_into().unwrap()) as u64;
        let next = f.seek(SeekFrom::Current(0))? + chunk_size;

        match chunk_id {
            b"fmt " => {
                // read only needed bytes
                let mut fmt_buf = [0u8; 16];
                f.read_exact(&mut fmt_buf[..chunk_size.min(16) as usize])?;
                byte_rate = Some(u32::from_le_bytes(fmt_buf[8..12].try_into().unwrap()));
                f.seek(SeekFrom::Start(next))?;
            }
            b"data" => {
                data_size = Some(chunk_size as u32);
                f.seek(SeekFrom::Start(next))?;
            }
            b"LIST" => {
                let mut list_type = [0u8; 4];
                f.read_exact(&mut list_type)?;
                if &list_type == b"INFO" {
                    let mut remaining = chunk_size - 4;
                    while remaining >= 8 {
                        if f.read(&mut buf)? != 8 { break; }
                        let sub_id = &buf[0..4];
                        let sub_size = u32::from_le_bytes(buf[4..8].try_into().unwrap()) as usize;
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
            }
            _ => {
                let _  = f.seek(SeekFrom::Start(next))?;
            }
        }
    }

    if let (Some(br), Some(ds)) = (byte_rate, data_size) {
        meta.duration_ms = Some((ds as u64 * 1000) / br as u64);
    }

    Ok(meta)
}

