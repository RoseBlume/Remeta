
# Remeta

This crate provides metadata extraction for **audio, video, bitmap, and vector image formats**. It supports reading artist/title info, durations, resolutions, and more.

---

## Example Usage

```rust
use std::fs::File;
use std::io::Result;
use remeta::{VideoMetadata, ImageMetadata, SongMetadata};

// Audio
let song = SongMetadata::from_file("song.mp3")?;
println!("{:?}", song);

// Video
let video = VideoMetadata::from_file("movie.mp4")?;
println!("{:?}", video);

// Images (bitmap or vector)
let video  = ImageMetadata::from_file("movie.mp4")?;
println!("{:?}", video);
```

---

## Features

This crate uses **optional Cargo features** to enable parsing only the formats you need.

### Audio

* **flac**

* **id3v1**

* **id3v2**

* **m4a**

* **wav**

* **flac-duration**

* **mp3-duration**

* **m4a-duration**

* **wav-duration**

### Video

* **mp4**

* **mkv**

* **avi**

* **mp4-duration**

* **mkv-duration**

* **avi-duration**

### Images

#### Bitmap formats

* **jpg**
* **png**
* **gif**
* **webp**
* **ico**
* **tiff**
* **bmp**
* **heif**
* **avif**

#### Vector formats

* **svg**
* **pdf**
* **eps**

---

## Cargo Feature Flags

Enable only the formats you need to reduce compile time and binary size. Example:

```toml
[dependencies.remeta]
version = "0.1"
features = ["mp3v2", "wav", "jpg", "png", "svg"]
```

You can also enable all features for convenience:

```toml
[dependencies.remeta]
version = "0.1"
features = ["audio-all", "video-all", "images-all", "vector-all"]
```

---

## Notes

* **Durations** are only computed if the corresponding `-duration` feature is enabled.
* **Vector images** may provide only width/height and format; color depth is generally unavailable.
* **HEIF/AVIF** detection requires the file to have a valid ISO BMFF container with the correct brand (`heic`, `avif`, etc.).

