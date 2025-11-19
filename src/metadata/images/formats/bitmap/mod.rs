

// Built-in formats (conditionally compiled)
#[cfg(feature = "jpg")]
pub mod jpg;

#[cfg(feature = "png")]
pub mod png;

#[cfg(feature = "gif")]
pub mod gif;

#[cfg(feature = "webp")]
pub mod webp;

#[cfg(feature = "ico")]
pub mod ico;

#[cfg(feature = "tiff")]
pub mod tiff;

#[cfg(feature = "bmp")]
pub mod bmp;

#[cfg(feature = "heif")]
pub mod heif;

#[cfg(feature = "avif")]
pub mod avif;


