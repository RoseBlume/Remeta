use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use crate::metadata::images::ImageMetadata;

pub fn parse(f: &mut File) -> io::Result<ImageMetadata> {
    f.seek(SeekFrom::Start(0))?;

    let mut xml = String::new();
    f.read_to_string(&mut xml)?;

    let mut width = None;
    let mut height = None;

    // Find <svg ... width="..." height="...">
    if let Some(start) = xml.find("<svg") {
        if let Some(end) = xml[start..].find('>') {
            let tag = &xml[start..start + end];

            if let Some(w) = extract_attr(tag, "width") {
                width = parse_dimension(&w);
            }
            if let Some(h) = extract_attr(tag, "height") {
                height = parse_dimension(&h);
            }
        }
    }

    Ok(ImageMetadata {
        title: None,
        dimensions: width.zip(height),
        color_depth: None,      // vector
        format: Some("svg".into()),
    })
}

fn extract_attr(tag: &str, name: &str) -> Option<String> {
    let pattern = format!("{}=\"", name);
    let start = tag.find(&pattern)? + pattern.len();
    let end = tag[start..].find('"')?;
    Some(tag[start..start + end].to_string())
}

fn parse_dimension(val: &str) -> Option<u32> {
    val.trim()
        .trim_end_matches(|c: char| !c.is_numeric())
        .parse::<u32>()
        .ok()
}
