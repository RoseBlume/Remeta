use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::SongMetadata;
use crate::helpers;
pub fn parse(f: &mut File) -> io::Result<SongMetadata> {
    let mut header = [0u8; 4];
    f.read_exact(&mut header)?;

    // Rewind so page parser can process full page
    f.seek(SeekFrom::Start(0))?;

    let mut meta = SongMetadata::default();

    loop {
        let mut capture = [0u8; 4];
        if f.read(&mut capture)? != 4 {
            break;
        }
        if &capture != b"OggS" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "bad ogg page"));
        }

        // Ogg page header (27 bytes after "OggS")
        let mut rest = [0u8; 23];
        f.read_exact(&mut rest)?;

        let seg_count = rest[22];
        let mut lacing = vec![0u8; seg_count as usize];
        f.read_exact(&mut lacing)?;

        let data_size: usize = lacing.iter().map(|&x| x as usize).sum();
        let mut data = vec![0u8; data_size];
        f.read_exact(&mut data)?;

        // Look for Vorbis Comment Header packet type = 3 ('\x03')
        if !data.is_empty() && data[0] == 3 {
            // Skip first 7 bytes: 1 type + 6 "vorbis"
            if data.len() > 7 && &data[1..7] == b"vorbis" {
                helpers::parse_vorbis_comments(&mut meta, &data[7..]);
                break;
            }
        }
    }

    Ok(meta)
}
