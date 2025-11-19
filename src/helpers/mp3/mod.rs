
#[cfg(feature = "id3v1")]
mod id3v1;

#[cfg(feature = "id3v1")]
pub use id3v1::trim_id3v1_text;

#[cfg(feature = "id3v2")]
mod id3v2;

#[cfg(feature = "id3v2")]
pub use id3v2::{
    synchsafe_to_u32,
    decode_text_frame
};
