use std::fs::File;
use std::io::{self, Seek, SeekFrom, Read};

pub fn compute(f: &mut File) -> io::Result<u64> {
    f.seek(SeekFrom::Start(0))?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    // MKV Duration: look for 0x44 0x89 (simplified)
    if let Some(pos) = buf.windows(2).position(|w| w == [0x44, 0x89]) {
        if buf.len() >= pos + 10 {
            let duration_bytes = &buf[pos + 2..pos + 10];
            let duration = f64::from_le_bytes(duration_bytes.try_into().unwrap());
            return Ok((duration * 1000.0) as u64);
        }
    }

    Ok(0)
}
