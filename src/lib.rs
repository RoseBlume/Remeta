//! # Remeta
//!
//! This crate provides metadata extraction for **audio, video, bitmap, and vector image formats**.
//! It supports reading artist/title info, durations, resolutions, and more.
//!
//! ## Example Usage
//!
//! ```rust
//! use std::fs::File;
//! use std::io::Result;
//! use remeta::{VideoMetadata, ImageMetadata, SongMetadata};
//!
//! // Audio
//! let song = SongMetadata::from_file("song.mp3")?;
//! println!("{:?}", song);
//!
//! // Video
//! let video = VideoMetadata::from_file("movie.mp4")?;
//! println!("{:?}", video);
//!
//! // Images (bitmap or vector)
//! let image = ImageMetadata::from_file("image.png")?;
//! println!("{:?}", image);
//! ```
//!
//! ## Features
//!
//! This crate uses **optional Cargo features** to enable parsing only the formats you need.
//!
//! ### Audio
//!
//! * **flac**
//! * **id3v1**
//! * **id3v2**
//! * **m4a**
//! * **wav**
//!
//! ### Video
//!
//! * **mp4**
//! * **mkv**
//! * **avi**
//!
//! ### Images
//!
//! #### Bitmap formats
//!
//! * **jpg**
//! * **png**
//! * **gif**
//! * **webp**
//! * **ico**
//! * **tiff**
//! * **bmp**
//! * **heif**
//! * **avif**
//!
//! #### Vector formats
//!
//! * **svg**
//! * **pdf**
//! * **eps**
//!
//! ## Cargo Feature Flags
//!
//! Enable only the formats you need to reduce compile time and binary size. Example:
//!
//! ```toml
//! [dependencies.remeta]
//! version = "0.1"
//! features = ["flac", "wav", "jpg", "png", "svg"]
//! ```
//!
//! You can also enable grouped features for convenience:
//!
//! ```toml
//! [dependencies.remeta]
//! version = "0.1"
//! features = ["music", "videos", "images"]
//! ```
//!
//! Or enable all bitmap or vector image formats:
//!
//! ```toml
//! [dependencies.remeta]
//! version = "0.1"
//! features = ["bitmap-all", "vector-all"]
//! ```
//!
//! ## Notes
//!
//! * **Durations** are only computed if the corresponding feature is supported by the format.
//! * **Vector images** may provide only width/height and format; color depth is generally unavailable.
//! * **HEIF/AVIF** detection requires the file to have a valid ISO BMFF container with the correct brand (`heic`, `avif`, etc.).

mod metadata;

#[cfg(any(feature = "flac", feature = "id3v1", feature = "id3v2", feature = "m4a"))]
mod helpers;

#[cfg(any(feature = "flac", feature = "id3v1", feature = "id3v2", feature = "m4a", feature = "wav"))]
pub use metadata::SongMetadata;

#[cfg(any(feature = "mp4", feature = "mkv", feature = "avi"))]
pub use metadata::VideoMetadata;

#[cfg(any(
    feature = "jpg",
    feature = "png",
    feature = "gif",
    feature = "webp",
    feature = "ico",
    feature = "tiff",
    feature = "bmp",
    feature = "heif",
    feature = "avif",
    feature = "svg",
    feature = "pdf",
    feature = "eps"
))]
pub use metadata::ImageMetadata;

