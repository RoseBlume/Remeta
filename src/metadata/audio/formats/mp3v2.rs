use std::fs::File;
use std::io::{self, Read};

use crate::SongMetadata;
use crate::helpers;
// use std::io::Seek;

pub fn parse(f: &mut File) -> io::Result<SongMetadata> {
    let mut header = [0u8; 10];
    f.read_exact(&mut header)?;

    let tag_size = helpers::synchsafe_to_u32(&header[6..10]) as usize;
    let mut tag_data = vec![0u8; tag_size];
    f.read_exact(&mut tag_data)?;

    let mut meta = SongMetadata::default();
    let mut i = 0;
    while i + 10 <= tag_data.len() {
        let id = &tag_data[i..i + 4];
        let size = u32::from_be_bytes(tag_data[i + 4..i + 8].try_into().unwrap()) as usize;
        if size == 0 || i + 10 + size > tag_data.len() {
            break;
        }
        let frame = &tag_data[i + 10..i + 10 + size];
        let text = helpers::decode_text_frame(frame);

        match id {
            b"TIT2" => meta.title = text,
            b"TPE1" => meta.artist = text,
            b"TALB" => meta.album = text,
            b"TCON" => meta.genre = text,
            _ => {}
        }

        i += 10 + size;
    }

    // --- compute duration from first MPEG audio frame + file size ---
    // total file size
    let total_len = f.metadata()?.len() as usize;

    // detect ID3v1 at end (128 bytes starting with "TAG")
    let mut id3v1_size = 0usize;
    if total_len >= 128 {
        let mut tail = [0u8; 3];
        std::io::Seek::seek(f, std::io::SeekFrom::End(-128))?;
        if f.read_exact(&mut tail).is_ok() {
            if &tail == b"TAG" {
                id3v1_size = 128;
            }
        }
    }

    // seek to start of audio (just after the ID3v2 tag)
    let audio_start = 10usize + tag_size;
    std::io::Seek::seek(f, std::io::SeekFrom::Start(audio_start as u64))?;

    // search for MPEG frame header (sync 11 bits: 0xFF and next byte & 0xE0 == 0xE0)
    let mut buf = [0u8; 8192];
    let mut found_header: Option<[u8; 4]> = None;
    loop {
        let n = f.read(&mut buf)?;
        if n < 4 {
            break;
        }
        for j in 0..=(n - 4) {
            if buf[j] == 0xFF && (buf[j + 1] & 0xE0) == 0xE0 {
                found_header = Some([buf[j], buf[j + 1], buf[j + 2], buf[j + 3]]);
                break;
            }
        }
        if found_header.is_some() {
            break;
        }
        if n < buf.len() {
            break;
        }
    }

    if let Some(hdr) = found_header {
        // parse header
        let version_id = (hdr[1] & 0x18) >> 3; // 3 = MPEG1, 2 = MPEG2, 0 = MPEG2.5
        let layer_index = (hdr[1] & 0x06) >> 1; // 3 = Layer I, 2 = Layer II, 1 = Layer III
        let bitrate_index = (hdr[2] & 0xF0) >> 4;
        // sampling_rate_index = (hdr[2] & 0x0C) >> 2;
        // padding = (hdr[2] & 0x02) >> 1;

        // bitrate tables (kbps). index 0 = free, 15 = bad/invalid
        let bitrate_kbps = match (version_id, layer_index) {
            (3, 3) => { // MPEG1 Layer I
                [0,32,64,96,128,160,192,224,256,288,320,352,384,416,448,0][bitrate_index as usize]
            }
            (3, 2) => { // MPEG1 Layer II
                [0,32,48,56,64,80,96,112,128,160,192,224,256,320,384,0][bitrate_index as usize]
            }
            (3, 1) => { // MPEG1 Layer III
                [0,32,40,48,56,64,80,96,112,128,160,192,224,256,320,0][bitrate_index as usize]
            }
            (_, 3) => { // MPEG2/2.5 Layer I
                [0,32,48,56,64,80,96,112,128,144,160,176,192,224,256,0][bitrate_index as usize]
            }
            (_, 2) => { // MPEG2/2.5 Layer II
                [0,8,16,24,32,40,48,56,64,80,96,112,128,144,160,0][bitrate_index as usize]
            }
            (_, 1) => { // MPEG2/2.5 Layer III
                [0,8,16,24,32,40,48,56,64,80,96,112,128,144,160,0][bitrate_index as usize]
            }
            _ => 0
        };

        if bitrate_kbps > 0 {
            let audio_bytes = total_len.saturating_sub(audio_start + id3v1_size);
            let duration_ms = ((audio_bytes as f64 * 8.0) / (bitrate_kbps as f64 * 1000.0) * 1000.0) as u64;
            // store duration (milliseconds). adjust type if your SongMetadata uses a different type.
            meta.duration_ms = Some(duration_ms);
        }
        else {
            meta.duration_ms = None;
        }
    }

    Ok(meta)
}