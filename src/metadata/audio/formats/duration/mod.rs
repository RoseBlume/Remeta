#[cfg(feature = "wav")]
pub mod wav;

#[cfg(feature = "flac")]
pub mod flac;

#[cfg(any(feature = "id3v1", feature = "id3v2"))]
pub mod mp3;

#[cfg(feature = "m4a")]
pub mod m4a;

#[cfg(feature = "ogg")]
pub mod ogg;

#[cfg(feature = "wma")]
pub mod wma;