use std::fs::File;
use std::io;

use crate::metadata::video::VideoMetadata;

#[cfg(feature = "mp4")]
mod mp4;
#[cfg(feature = "mkv")]
mod mkv;
#[cfg(feature = "avi")]
mod avi;

#[cfg(any(feature = "mp4-duration", feature = "mkv-duration", feature = "avi-duration"))]
mod duration;

pub fn detect_and_parse(header: &[u8], f: &mut File) -> io::Result<VideoMetadata> {
    Ok(match &header[0..4] {
        #[cfg(feature = "mp4")]
        b"\x00\x00\x00\x18" | b"\x00\x00\x00\x20" => { // common MP4 box headers
            let mut m = mp4::parse(f)?;
            #[cfg(feature = "mp4-duration")]
            {
                m.duration_ms = duration::mp4::compute(f).ok();
            }
            m
        }
        #[cfg(feature = "mkv")]
        b"\x1A\x45\xDF\xA3" => { // EBML header for MKV
            let mut m = mkv::parse(f)?;
            #[cfg(feature = "mkv-duration")]
            {
                m.duration_ms = duration::mkv::compute(f).ok();
            }
            m
        }
        #[cfg(feature = "avi")]
        b"RIFF" if &header[8..12] == b"AVI " => {
            let mut m = avi::parse(f)?;
            #[cfg(feature = "avi-duration")]
            {
                m.duration_ms = duration::avi::compute(f).ok();
            }
            m
        }
        _ => VideoMetadata::default(),
    })
}
