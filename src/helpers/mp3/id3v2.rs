
pub fn synchsafe_to_u32(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32 & 0x7F) << 21)
        | ((bytes[1] as u32 & 0x7F) << 14)
        | ((bytes[2] as u32 & 0x7F) << 7)
        | (bytes[3] as u32 & 0x7F)
}

pub fn decode_text_frame(data: &[u8]) -> Option<String> {
    if data.is_empty() { return None; }
    match data[0] {
        0 => Some(String::from_utf8_lossy(&data[1..]).trim_matches(char::from(0)).to_string()),
        1 => {
            let utf16: Vec<u16> = data[1..]
                .chunks(2)
                .filter_map(|b| if b.len() == 2 { Some(u16::from_be_bytes([b[0], b[1]])) } else { None })
                .collect();
            Some(String::from_utf16_lossy(&utf16).trim_matches(char::from(0)).to_string())
        }
        _ => None,
    }
}