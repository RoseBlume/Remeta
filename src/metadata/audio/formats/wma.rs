use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::SongMetadata;
use crate::helpers::{read_u64_le, read_u8, read_u16_le, read_u32_le};
const ASF_HEADER_GUID: [u8; 16] = [
    0x30,0x26,0xb2,0x75,0x8e,0x66,0xcf,0x11,
    0xa6,0xd9,0x00,0xaa,0x00,0x62,0xce,0x6c,
];

const ASF_CONTENT_DESC_GUID: [u8; 16] = [
    0x33,0x26,0xb2,0x75,0x8e,0x66,0xcf,0x11,
    0xa6,0xd9,0x00,0xaa,0x00,0x62,0xce,0x6c,
];

const ASF_EXT_CONTENT_DESC_GUID: [u8; 16] = [
    0x40,0xa4,0xd0,0xd2,0x07,0xe3,0xd2,0x11,
    0x97,0xf0,0x00,0xa0,0xc9,0x5e,0xa8,0x50,
];



// Read UTF-16LE string of given byte length
fn read_utf16le_string(f: &mut File, len: u16) -> io::Result<String> {
    let mut buf = vec![0u8; len as usize];
    f.read_exact(&mut buf)?;
    let mut out = String::new();

    for chunk in buf.chunks_exact(2) {
        let c = u16::from_le_bytes([chunk[0], chunk[1]]);
        if c == 0 { continue; }
        if let Some(ch) = char::from_u32(c as u32) {
            out.push(ch);
        }
    }
    Ok(out)
}

pub fn parse(f: &mut File) -> io::Result<SongMetadata> {
    let mut guid = [0u8; 16];
    f.read_exact(&mut guid)?;
    if guid != ASF_HEADER_GUID {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "not wma/asf"));
    }

    let _size = read_u64_le(f)?;     // object size
    let _objs = read_u32_le(f)?;     // number of child objects
    let _reserved = read_u8(f)?;     // reserved
    let _reserved2 = read_u8(f)?;

    let mut meta = SongMetadata::default();

    while let Ok(_) = f.read_exact(&mut guid) {
        let obj_size = read_u64_le(f)? as i64;

        if guid == ASF_CONTENT_DESC_GUID {
            let title_len = read_u16_le(f)?;
            let author_len = read_u16_le(f)?;
            let copyright_len = read_u16_le(f)?;
            let desc_len = read_u16_le(f)?;
            let rating_len = read_u16_le(f)?;

            if title_len > 0 {
                meta.title = read_utf16le_string(f, title_len).ok();
            } else { f.seek(SeekFrom::Current(0))?; }

            if author_len > 0 {
                meta.artist = read_utf16le_string(f, author_len).ok();
            } else { f.seek(SeekFrom::Current(0))?; }

            // Skip remaining strings
            f.seek(SeekFrom::Current(
                copyright_len as i64 +
                desc_len as i64 +
                rating_len as i64
            ))?;
        }
        else if guid == ASF_EXT_CONTENT_DESC_GUID {
            let desc_count = read_u16_le(f)?;

            for _ in 0..desc_count {
                let name_len = read_u16_le(f)?;
                let name = read_utf16le_string(f, name_len)?;

                // let data_type = read_u16_le(f)?;
                let data_len = read_u16_le(f)?;

                if let Ok(value) = read_utf16le_string(f, data_len) {
                    match name.as_str() {
                        "WM/AlbumTitle" => meta.album = Some(value),
                        "WM/Genre" => meta.genre = Some(value),
                        _ => {}
                    }
                } else {
                    f.seek(SeekFrom::Current(data_len as i64))?;
                }
            }
        }
        else {
            // Skip unknown object
            f.seek(SeekFrom::Current(obj_size - 24))?;
        }
    }

    Ok(meta)
}
