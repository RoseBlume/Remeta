use std::fs::File;
use std::io::{self, Seek, SeekFrom, Read};

pub fn compute(f: &mut File) -> io::Result<u64> {
    f.seek(SeekFrom::Start(0))?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    // Look for 'moov' atom
    if let Some(pos) = buf.windows(4).position(|w| w == b"moov") {
        if let Some(mvhd_pos) = buf[pos..].windows(4).position(|w| w == b"mvhd") {
            let mvhd_start = pos + mvhd_pos + 8; // skip size/type
            if buf.len() > mvhd_start + 12 {
                let timescale = u32::from_be_bytes(buf[mvhd_start + 4..mvhd_start + 8].try_into().unwrap());
                let duration = u32::from_be_bytes(buf[mvhd_start + 8..mvhd_start + 12].try_into().unwrap());
                if timescale > 0 {
                    return Ok((duration as u64 * 1000) / timescale as u64);
                }
            }
        }
    }

    Ok(0)
}
