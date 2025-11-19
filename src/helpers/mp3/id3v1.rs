#[cfg(feature = "id3v1")]
pub fn trim_id3v1_text(b: &[u8]) -> Option<String> {
    let binding = String::from_utf8_lossy(b);
    let s = binding.trim_end_matches('\u{0}').trim();
    if s.is_empty() { None } else { Some(s.to_string()) }
}