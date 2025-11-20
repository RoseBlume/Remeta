#[cfg(any(feature = "id3v1", feature = "id3v2"))]
mod mp3;


#[cfg(feature = "id3v1")]
pub use mp3::trim_id3v1_text;

#[cfg(feature = "id3v2")]
pub use mp3::{synchsafe_to_u32, decode_text_frame};

#[cfg(feature = "m4a")]
mod m4a;

#[cfg(feature = "m4a")]
pub use m4a::extract_m4a_text;



#[cfg(feature = "flac")]
mod flac;

#[cfg(feature = "flac")]
pub use flac::{parse_vorbis_comments};

#[cfg(feature = "wma")]
mod wma;

#[cfg(feature = "wma")]
pub use wma::{read_u64_le, read_u16_le, read_u32_le, read_u8};