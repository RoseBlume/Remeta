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
#[cfg(feature = "ogg")]
mod ogg;
#[cfg(feature = "wma")]
mod wma;

#[cfg(any(feature = "wav", feature = "id3v1", feature = "id3v2", feature = "flac", feature = "m4a", feature = "ogg", feature = "wma"))]
mod duration;

pub fn detect_and_parse(header: &[u8], f: &mut File) -> io::Result<SongMetadata> {
    Ok(match &header[0..4] {
        #[cfg(feature = "wav")]
        b"RIFF" if &header[8..12] == b"WAVE" => {
            let mut m = wav::parse(f)?;
            m.duration_ms = duration::wav::compute(f).ok();
            m
        }

        #[cfg(feature = "flac")]
        b"fLaC" => {
            let mut m = flac::parse(f)?;
            m.duration_ms = duration::flac::compute(f).ok();
            m
        }

        // WMA / ASF GUID starts with 0x30 0x26 0xB2 0x75 -> "0&\xB2u"
        #[cfg(feature = "wma")]
        b"0&\xB2u" => {
            let mut m = wma::parse(f)?;
            m.duration_ms = duration::wma::compute(f).ok();
            m
        }

        // OGG always starts with "OggS"
        #[cfg(feature = "ogg")]
        b"OggS" => {
            let mut m = ogg::parse(f)?;
            m.duration_ms = duration::ogg::compute(f).ok();
            m
        }

        #[cfg(feature = "id3v2")]
        b"ID3\x03" | b"ID3\x04" => {
            let mut m = mp3v2::parse(f)?;
            m.duration_ms = duration::mp3::compute(f).ok();
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
            #[cfg(feature = "m4a")]
            {
                m.duration_ms = duration::m4a::compute(f).ok();
            }

            #[cfg(any(feature = "id3v1", feature = "id3v2"))]
            {
                m.duration_ms = m.duration_ms.or_else(|| duration::mp3::compute(f).ok());
            }

            #[cfg(feature = "ogg")]
            {
                if m.duration_ms.is_none() {
                    m.duration_ms = duration::ogg::compute(f).ok();
                }
            }

            #[cfg(feature = "wma")]
            {
                if m.duration_ms.is_none() {
                    m.duration_ms = duration::wma::compute(f).ok();
                }
            }

            m
        }
    })
}
