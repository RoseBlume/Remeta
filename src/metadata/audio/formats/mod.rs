use std::fs::File;
use std::io;

use crate::SongMetadata;

#[cfg(feature = "wav")]
mod wav;
#[cfg(feature = "flac")]
mod flac;
#[cfg(feature = "id3v1")]
mod mp3v1;
#[cfg(feature = "id3v2")]
mod mp3v2;
#[cfg(feature = "m4a")]
mod m4a;

#[cfg(any(feature = "wav-duration", feature = "mp3-duration", feature = "flac-duration", feature = "m4a-duration"))]
mod duration;

pub fn detect_and_parse(header: &[u8], f: &mut File) -> io::Result<SongMetadata> {
    Ok(match &header[0..4] {
        #[cfg(feature = "wav")]
        b"RIFF" if &header[8..12] == b"WAVE" => {
            let mut m = wav::parse(f)?;
            #[cfg(feature = "wav-duration")]
            {
                m.duration_ms = duration::wav::compute(f).ok();
            }
            m
        }
        #[cfg(feature = "flac")]
        b"fLaC" => {
            let mut m = flac::parse(f)?;
            #[cfg(feature = "flac-duration")]
            {
                m.duration_ms = duration::flac::compute(f).ok();
            }
            m
        }
        #[cfg(feature = "id3v2")]
        b"ID3\x03" | b"ID3\x04" => {
            let mut m = mp3v2::parse(f)?;
            #[cfg(feature = "mp3-duration")]
            {
                m.duration_ms = duration::mp3::compute(f).ok();
            }
            m
        }
        _ => {
            let mut m = SongMetadata::default();
            #[cfg(feature = "id3v1")]
            if let Ok(v1) = mp3v1::parse(f) {
                m = v1;
            }
            #[cfg(feature = "m4a")]
            if let Ok(m4) = m4a::parse(f) {
                m = m4;
            }
            // durations
            #[cfg(feature = "m4a-duration")]
            {
                m.duration_ms = duration::m4a::compute(f).ok();
            }
            #[cfg(feature = "mp3-duration")]
            {
                m.duration_ms = m.duration_ms.or_else(|| duration::mp3::compute(f).ok());
            }
            m
        }
    })
}
