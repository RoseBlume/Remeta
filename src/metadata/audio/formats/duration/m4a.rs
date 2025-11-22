use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

#[inline(always)]
fn be_u32(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) << 24)
        | ((bytes[1] as u32) << 16)
        | ((bytes[2] as u32) << 8)
        |  (bytes[3] as u32)
}

#[inline(always)]
fn be_u64(bytes: &[u8]) -> u64 {
    ((bytes[0] as u64) << 56)
        | ((bytes[1] as u64) << 48)
        | ((bytes[2] as u64) << 40)
        | ((bytes[3] as u64) << 32)
        | ((bytes[4] as u64) << 24)
        | ((bytes[5] as u64) << 16)
        | ((bytes[6] as u64) << 8)
        |  (bytes[7] as u64)
}

pub fn compute(f: &mut File) -> io::Result<u64> {
    f.seek(SeekFrom::Start(0))?;

    // Read file to memory
    let mut data = Vec::new();
    f.read_to_end(&mut data)?;
    let len = data.len();

    let mut i = 0usize;

    while i + 8 <= len {
        let size = be_u32(&data[i..i + 4]) as usize;

        if size < 8 || i + size > len {
            break;
        }

        // 'moov' box?
        if &data[i + 4..i + 8] == b"moov" {
            let moov_end = i + size;
            let mut j = i + 8;

            while j + 8 <= moov_end {
                let sub_size = be_u32(&data[j..j + 4]) as usize;

                if sub_size < 8 || j + sub_size > len {
                    break;
                }

                // 'mvhd' box?
                if &data[j + 4..j + 8] == b"mvhd" {
                    let version = data[j + 8];

                    if version == 1 {
                        // version 1: 64-bit duration
                        if j + 36 > len {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "mvhd truncated",
                            ));
                        }

                        let timescale = be_u32(&data[j + 24..j + 28]);
                        let duration = be_u64(&data[j + 28..j + 36]);

                        if timescale == 0 {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "invalid timescale",
                            ));
                        }

                        return Ok((duration * 1000) / timescale as u64);
                    } else {
                        // version 0: 32-bit duration
                        if j + 28 > len {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "mvhd truncated v0",
                            ));
                        }

                        let timescale = be_u32(&data[j + 20..j + 24]);
                        let duration = be_u32(&data[j + 24..j + 28]) as u64;

                        if timescale == 0 {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "invalid timescale",
                            ));
                        }

                        return Ok((duration * 1000) / timescale as u64);
                    }
                }

                j += sub_size;
            }
        }

        i += size;
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No m4a duration",
    ))
}
