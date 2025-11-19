use std::fs::File;
use std::io::{self, Seek, SeekFrom, Read};
use std::path::Path;

#[derive(Debug, Default)]
pub struct VideoMetadata {
    pub title: Option<String>,
    pub director: Option<String>,
    pub resolution: Option<(u32, u32)>,
    pub duration_ms: Option<u64>,
    pub codec: Option<String>,
}

mod formats;

impl VideoMetadata {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path_ref = path.as_ref();
        let mut f = File::open(path_ref)?;

        let mut header = [0u8; 16]; // Videos may require more bytes for format detection
        if f.read(&mut header)? < 16 {
            return Ok(Self::default_with_filename(path_ref));
        }
        f.seek(SeekFrom::Start(0))?;

        let mut meta = formats::detect_and_parse(&header, &mut f)?;

        if meta.title.is_none() {
            meta.title = Some(Self::prettify_filename(path_ref));
        }

        Ok(meta)
    }

    fn default_with_filename(path: &Path) -> Self {
        let mut m = Self::default();
        m.title = Some(Self::prettify_filename(path));
        m
    }

    fn prettify_filename(path: &Path) -> String {
        let file_name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown");

        file_name
            .replace('_', " ")
            .replace('-', " ")
            .split_whitespace()
            .map(|w| {
                let mut chars = w.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
