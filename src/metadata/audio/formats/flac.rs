use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::helpers;
use crate::SongMetadata;

pub fn parse(f: &mut File) -> io::Result<SongMetadata> {
    let mut header = [0u8; 4];
    f.read_exact(&mut header)?;
    let mut meta = SongMetadata::default();
    loop {
        let mut block_header = [0u8; 4];
        if f.read(&mut block_header)? != 4 {
            break;
        }

        let last_block = (block_header[0] & 0x80) != 0;
        let block_type = block_header[0] & 0x7F;
        let block_len =
            ((block_header[1] as u32) << 16) | ((block_header[2] as u32) << 8) | block_header[3] as u32;

        if block_type == 4 {
            let mut data = vec![0u8; block_len as usize];
            f.read_exact(&mut data)?;
            helpers::parse_vorbis_comments(&mut meta, &data);
        } else {
            f.seek(SeekFrom::Current(block_len as i64))?;
        }

        if last_block {
            break;
        }
    }

    Ok(meta)
}