use std::fs::File;
use std::io::{Read, Seek, SeekFrom, self};

pub fn compute(f: &mut File) -> io::Result<u64> {
    let mut data = Vec::new();
    f.seek(SeekFrom::Start(0))?;
    f.read_to_end(&mut data)?;

    let mut i = 0usize;
    while i + 8 <= data.len() {
        let size = u32::from_be_bytes(data[i..i + 4].try_into().unwrap()) as usize;
        if size < 8 || i + size > data.len() {
            break;
        }
        if &data[i + 4..i + 8] == b"moov" {
            // search for mvhd inside moov
            let mut j = i + 8;
            while j + 8 <= i + size {
                let sub_size = u32::from_be_bytes(data[j..j + 4].try_into().unwrap()) as usize;
                if sub_size < 8 || j + sub_size > data.len() {
                    break;
                }
                if &data[j + 4..j + 8] == b"mvhd" {
                    let version = data[j + 8];
                    if version == 1 {
                        // 64-bit duration: fields at j+24..j+28 timescale, j+28..j+36 duration
                        if j + 36 > data.len() {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "mvhd truncated"));
                        }
                        let timescale = u32::from_be_bytes(data[j + 24..j + 28].try_into().unwrap());
                        let duration = u64::from_be_bytes(data[j + 28..j + 36].try_into().unwrap());
                        if timescale == 0 {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid timescale"));
                        }
                        return Ok((duration * 1000) / timescale as u64);
                    } else {
                        // version 0: 32-bit duration at j+24..j+28
                        if j + 28 > data.len() {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "mvhd truncated v0"));
                        }
                        let timescale = u32::from_be_bytes(data[j + 20..j + 24].try_into().unwrap());
                        let duration = u32::from_be_bytes(data[j + 24..j + 28].try_into().unwrap()) as u64;
                        if timescale == 0 {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid timescale"));
                        }
                        return Ok((duration * 1000) / timescale as u64);
                    }
                }
                j += sub_size;
            }
        }
        i += size;
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "No m4a duration"))
}