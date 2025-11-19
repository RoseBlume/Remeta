use crate::SongMetadata;
// --- Shared helpers ---




pub fn parse_vorbis_comments(meta: &mut SongMetadata, data: &[u8]) {
    if data.len() < 8 { return; }
    let vendor_len = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
    let mut idx = 4 + vendor_len;
    if idx + 4 > data.len() { return; }
    let count = u32::from_le_bytes(data[idx..idx + 4].try_into().unwrap()) as usize;
    idx += 4;
    for _ in 0..count {
        if idx + 4 > data.len() { break; }
        let len = u32::from_le_bytes(data[idx..idx + 4].try_into().unwrap()) as usize;
        idx += 4;
        if idx + len > data.len() { break; }
        if let Ok(s) = String::from_utf8(data[idx..idx + len].to_vec()) {
            let parts: Vec<_> = s.splitn(2, '=').collect();
            if parts.len() == 2 {
                match parts[0].to_ascii_lowercase().as_str() {
                    "artist" => meta.artist = Some(parts[1].to_string()),
                    "title" => meta.title = Some(parts[1].to_string()),
                    "album" => meta.album = Some(parts[1].to_string()),
                    "genre" => meta.genre = Some(parts[1].to_string()),
                    _ => {}
                }
            }
        }
        idx += len;
    }
}


