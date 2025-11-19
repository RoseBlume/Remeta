use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::metadata::video::VideoMetadata;

pub fn parse(f: &mut File) -> io::Result<VideoMetadata> {
    let mut metadata = VideoMetadata::default();
    f.seek(SeekFrom::Start(0))?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    // AVI: duration = total frames / frame rate
    // Will look for 'avih' chunk (AVI header)
    let avih_pos = buf.windows(4).position(|w| w == b"avih");
    if let Some(pos) = avih_pos {
        if buf.len() >= pos + 24 {
            let microsec_per_frame = u32::from_le_bytes(buf[pos + 8..pos + 12].try_into().unwrap());
            let total_frames = u32::from_le_bytes(buf[pos + 16..pos + 20].try_into().unwrap());
            if microsec_per_frame > 0 {
                metadata.duration_ms = Some((total_frames as u64 * microsec_per_frame as u64) / 1000);
            }
        }
    }

    metadata.title = Some("Unknown".to_string());
    Ok(metadata)
}
