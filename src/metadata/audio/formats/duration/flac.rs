use std::fs::File;
use std::io::{Read, Seek, SeekFrom, self};

pub fn compute(f: &mut File) -> io::Result<u64> {
    f.seek(SeekFrom::Start(4))?;

    // iterate blocks until STREAMINFO (type 0)
    loop {
        let mut block_header = [0u8; 4];
        if f.read(&mut block_header)? != 4 {
            break;
        }
        let last_block = (block_header[0] & 0x80) != 0;
        let block_type = block_header[0] & 0x7F;
        let block_len =
            ((block_header[1] as u32) << 16) | ((block_header[2] as u32) << 8) | block_header[3] as u32;

        if block_type == 0 {
            let mut data = vec![0; block_len as usize];
            f.read_exact(&mut data)?;
            if data.len() < 18 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "STREAMINFO too small"));
            }

            // sample rate: 20 bits (bits 0..19 of the composite field starting at data[10])
            let sample_rate = ((data[10] as u32) << 12)
                | ((data[11] as u32) << 4)
                | ((data[12] as u32 & 0xF0) >> 4);

            // total samples: 36 bits (last 4 bits of data[12] and data[13..17])
            let total_samples =
                ((data[12] as u64 & 0x0F) << 32)
                    | ((data[13] as u64) << 24)
                    | ((data[14] as u64) << 16)
                    | ((data[15] as u64) << 8)
                    | (data[16] as u64);

            if sample_rate == 0 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid sample rate"));
            }

            let duration_ms = (total_samples * 1000) / sample_rate as u64;
            return Ok(duration_ms);
        } else {
            f.seek(SeekFrom::Current(block_len as i64))?;
        }

        if last_block {
            break;
        }
    }

    Err(io::Error::new(io::ErrorKind::InvalidData, "No STREAMINFO"))
}