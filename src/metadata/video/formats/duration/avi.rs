use std::fs::File;
use std::io::{self, Seek, SeekFrom, Read};

pub fn compute(f: &mut File) -> io::Result<u64> {
    f.seek(SeekFrom::Start(0))?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    // Look for 'avih' header
    if let Some(pos) = buf.windows(4).position(|w| w == b"avih") {
        if buf.len() >= pos + 24 {
            let microsec_per_frame = u32::from_le_bytes(buf[pos + 8..pos + 12].try_into().unwrap());
            let total_frames = u32::from_le_bytes(buf[pos + 16..pos + 20].try_into().unwrap());
            if microsec_per_frame > 0 {
                return Ok((total_frames as u64 * microsec_per_frame as u64) / 1000);
            }
        }
    }

    Ok(0)
}
