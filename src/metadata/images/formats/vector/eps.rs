use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use crate::metadata::images::ImageMetadata;

pub fn parse(f: &mut File) -> io::Result<ImageMetadata> {
    f.seek(SeekFrom::Start(0))?;

    let reader = BufReader::new(f);
    let mut dimensions = None;

    for line in reader.lines() {
        let line = line?;
        if let Some(rest) = line.strip_prefix("%%BoundingBox:") {
            let nums: Vec<i32> = rest
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();

            if nums.len() == 4 {
                let width = (nums[2] - nums[0]) as u32;
                let height = (nums[3] - nums[1]) as u32;
                dimensions = Some((width, height));
            }
            break;
        }
    }

    Ok(ImageMetadata {
        title: None,
        dimensions,
        color_depth: None,    // vector
        format: Some("eps".into()),
    })
}
