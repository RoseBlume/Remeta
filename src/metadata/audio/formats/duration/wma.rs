use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::helpers::{read_u64_le};
const ASF_FILE_PROPERTIES: [u8; 16] = [
    0xA1,0xDC,0xAB,0x8C,0x47,0xA9,0xCF,0x11,
    0x8E,0xE4,0x00,0xC0,0x0C,0x20,0x53,0x65,
];



pub fn compute(f: &mut File) -> io::Result<u64> {
    f.seek(SeekFrom::Start(0))?;

    loop {
        let mut guid = [0u8; 16];
        if f.read(&mut guid)? != 16 {
            break; // EOF
        }

        let size = read_u64_le(f)?; // object size

        if guid == ASF_FILE_PROPERTIES {
            // skip 40 bytes to reach "play duration" (offset +40 inside object)
            f.seek(SeekFrom::Current(40))?;

            let play_duration_100ns = read_u64_le(f)?;

            // Convert to milliseconds
            return Ok(play_duration_100ns / 10_000);
        } else {
            // skip entire object minus bytes we already consumed (24)
            f.seek(SeekFrom::Current(size as i64 - 24))?;
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "no File Properties object",
    ))
}


