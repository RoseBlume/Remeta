#[test]
fn test_all_files() {
    use std::fs::File;
    use std::io::Result;
    use remeta::{VideoMetadata, ImageMetadata, SongMetadata};

    // Audio
    let song = SongMetadata::from_file("tests/song.flac").expect("Failed to find file");
    println!("{:?}", song);

    // Video
    let video = VideoMetadata::from_file("tests/movie.mp4").expect("Failed to find file");
    println!("{:?}", video);

    // Images (bitmap or vector)
    let video  = ImageMetadata::from_file("tests/image.png").expect("Failed to find file");
    println!("{:?}", video);
}