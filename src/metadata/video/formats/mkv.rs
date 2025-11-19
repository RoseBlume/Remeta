use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::metadata::video::VideoMetadata;

pub fn parse(f: &mut File) -> io::Result<VideoMetadata> {
    let mut metadata = VideoMetadata::default();
    f.seek(SeekFrom::Start(0))?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    // MKV (Matroska) uses EBML; parse Duration if available
    // Simplified: look for a 0x44 0x89 tag for Duration (float)
    let duration_tag = buf.windows(2).position(|w| w == [0x44, 0x89]);
    if let Some(pos) = duration_tag {
        if buf.len() >= pos + 8 {
            let duration_bytes = &buf[pos + 2..pos + 10];
            let duration = f64::from_le_bytes(duration_bytes.try_into().unwrap());
            metadata.duration_ms = Some((duration * 1000.0) as u64);
        }
    }

    metadata.title = Some("Unknown".to_string());
    Ok(metadata)
}
