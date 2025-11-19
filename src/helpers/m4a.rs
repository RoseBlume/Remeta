#[cfg(feature = "m4a")]
pub fn extract_m4a_text(data: &[u8]) -> Option<String> {
    let mut i = 0;
    while i + 8 <= data.len() {
        let size = u32::from_be_bytes(data[i..i + 4].try_into().unwrap()) as usize;
        if size < 8 || i + size > data.len() {
            break;
        }
        if &data[i + 4..i + 8] == b"data" {
            // skip possible data header: often 8 (data header) + 8 (meta) => text starts at i+16
            let start = if i + 16 <= i + size { i + 16 } else { i + 8 };
            let text = String::from_utf8_lossy(&data[start..i + size]);
            return Some(text.trim_matches(char::from(0)).to_string());
        }
        i += size;
    }
    None
}