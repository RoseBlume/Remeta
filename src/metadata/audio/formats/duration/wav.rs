use std::fs::File;
use std::io::{Read, Seek, SeekFrom, self};

pub fn compute(f: &mut File) -> io::Result<u64> {
    f.seek(SeekFrom::Start(12))?;

    let mut fmt_found = false;
    let mut byte_rate = 0u32;
    let mut data_size = 0u32;

    let mut buf = [0u8; 8];

    while f.read(&mut buf)? == 8 {
        let id = &buf[0..4];
        let size = u32::from_le_bytes(buf[4..8].try_into().unwrap());
        let next = f.seek(SeekFrom::Current(0))? + size as u64;

        if id == b"fmt " {
            let mut fmt = vec![0u8; size as usize];
            f.read_exact(&mut fmt)?;
            if fmt.len() >= 12 {
                byte_rate = u32::from_le_bytes(fmt[8..12].try_into().unwrap());
                fmt_found = true;
            }
        } else if id == b"data" {
            data_size = size;
        } else {
            f.seek(SeekFrom::Start(next))?;
        }
    }

    if fmt_found && byte_rate > 0 {
        let duration_ms = (data_size as u64 * 1000) / byte_rate as u64;
        return Ok(duration_ms);
    }

    Err(io::Error::new(io::ErrorKind::InvalidData, "No WAV duration"))
}