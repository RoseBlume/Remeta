use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::metadata::video::VideoMetadata;

pub fn parse(f: &mut File) -> io::Result<VideoMetadata> {
    let mut metadata = VideoMetadata::default();
    f.seek(SeekFrom::Start(0))?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    // Attempt to find 'moov' atom for metadata
    let moov_pos = buf.windows(4).position(|w| w == b"moov");
    if let Some(pos) = moov_pos {
        // This is simplistic: duration and timescale are often in 'mvhd' atom inside 'moov'
        if let Some(mvhd_pos) = buf[pos..].windows(4).position(|w| w == b"mvhd") {
            let mvhd_start = pos + mvhd_pos + 8; // skip size/type
            if buf.len() > mvhd_start + 12 {
                let timescale = u32::from_be_bytes(buf[mvhd_start + 4..mvhd_start + 8].try_into().unwrap());
                let duration = u32::from_be_bytes(buf[mvhd_start + 8..mvhd_start + 12].try_into().unwrap());
                if timescale > 0 {
                    metadata.duration_ms = Some((duration as u64 * 1000) / timescale as u64);
                }
            }
        }
    }

    // Use filename if no title found
    metadata.title = Some("Unknown".to_string());

    Ok(metadata)
}
