use std::fs::File;
use std::io::{self, Read};

use crate::SongMetadata;
use crate::helpers;

pub fn parse(f: &mut File) -> io::Result<SongMetadata> {
    let mut header = [0u8; 10];
    f.read_exact(&mut header)?;
    if &header[0..3] != b"ID3" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "no id3v2 header"));
    }

    let tag_size = helpers::synchsafe_to_u32(&header[6..10]) as usize;
    let mut tag_data = vec![0u8; tag_size];
    f.read_exact(&mut tag_data)?;

    let mut meta = SongMetadata::default();
    let mut i = 0;
    while i + 10 <= tag_data.len() {
        let id = &tag_data[i..i + 4];
        let size = u32::from_be_bytes(tag_data[i + 4..i + 8].try_into().unwrap()) as usize;
        if size == 0 || i + 10 + size > tag_data.len() {
            break;
        }
        let frame = &tag_data[i + 10..i + 10 + size];
        let text = helpers::decode_text_frame(frame);

        match id {
            b"TIT2" => meta.title = text,
            b"TPE1" => meta.artist = text,
            b"TALB" => meta.album = text,
            b"TCON" => meta.genre = text,
            _ => {}
        }

        i += 10 + size;
    }

    Ok(meta)
}