#[cfg(any(feature = "wav", feature = "id3v1", feature = "id3v2", feature = "flac", feature = "m4a", feature = "ogg", feature = "wma"))]
mod audio;
#[cfg(any(feature = "wav", feature = "id3v1", feature = "id3v2", feature = "flac", feature = "m4a", feature = "ogg", feature = "wma"))]
pub use audio::SongMetadata;

#[cfg(any(feature = "mp4", feature = "mkv", feature = "avi"))]
mod video;
#[cfg(any(feature = "mp4", feature = "mkv", feature = "avi"))]
pub use video::VideoMetadata;

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
mod images;

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
pub use images::ImageMetadata;