use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use crate::SongMetadata;

use crate::helpers;
pub fn parse(f: &mut File) -> io::Result<SongMetadata> {
    let meta = f.metadata()?;
    let file_size = meta.len();

    // Always compute duration if possible
    let duration_ms = {
        let bitrate = 160_000u64; // Use EXIFTool's bitrate or your own default
        Some(((file_size * 8) as f64 / bitrate as f64 * 1000.0) as u64)
    };

    // If file too small for ID3v1, skip parsing it
    if file_size < 128 {
        return Ok(SongMetadata {
            artist: None,
            title: None,
            album: None,
            genre: None,
            duration_ms,
        });
    }

    // Try to read ID3v1 block
    f.seek(SeekFrom::End(-128))?;
    let mut buf = [0u8; 128];
    f.read_exact(&mut buf)?;

    // If footer isn't "TAG", the file simply doesn't have ID3v1
    if &buf[0..3] != b"TAG" {
        return Ok(SongMetadata {
            artist: None,
            title: None,
            album: None,
            genre: None,
            duration_ms,
        });
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
        duration_ms,
    })
}
