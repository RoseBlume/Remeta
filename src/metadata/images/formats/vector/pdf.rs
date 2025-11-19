use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::metadata::images::ImageMetadata;

pub fn parse(f: &mut File) -> io::Result<ImageMetadata> {
    f.seek(SeekFrom::Start(0))?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    // Look for MediaBox: [x0 y0 x1 y1]
    let mut dimensions = None;

    if let Some(idx) = buf.find("/MediaBox") {
        if let Some(start) = buf[idx..].find('[') {
            if let Some(end) = buf[idx..].find(']') {
                let arr = &buf[idx + start + 1 .. idx + end];
                let nums: Vec<f32> = arr
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect();

                if nums.len() == 4 {
                    let width = (nums[2] - nums[0]) as u32;
                    let height = (nums[3] - nums[1]) as u32;
                    dimensions = Some((width, height));
                }
            }
        }
    }

    Ok(ImageMetadata {
        title: None,
        dimensions,
        color_depth: None,     // vector
        format: Some("pdf".into()),
    })
}
