use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use crate::SongMetadata;

use crate::helpers;
pub fn parse(f: &mut File) -> io::Result<SongMetadata> {
    let len = f.seek(SeekFrom::End(0))?;
    if len < 128 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "no id3v1"));
    }
    f.seek(SeekFrom::End(-128))?;
    let mut buf = [0u8; 128];
    f.read_exact(&mut buf)?;
    if &buf[0..3] != b"TAG" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "no TAG header"));
    }

    let title = helpers::trim_id3v1_text(&buf[3..33]);
    let artist = helpers::trim_id3v1_text(&buf[33..63]);
    let album = helpers::trim_id3v1_text(&buf[63..93]);
    let genre = Some(format!("{}", buf[127]));

    Ok(SongMetadata {
        artist,
        title,
        album,
        genre,
        duration_ms: None,
    })
}