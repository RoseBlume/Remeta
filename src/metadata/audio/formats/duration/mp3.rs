use std::fs::File;
use std::io::{Read, Seek, SeekFrom, self};
use std::cmp::min;
use crate::helpers;
pub fn compute(f: &mut File) -> io::Result<u64> {

    let total_size = f.metadata()?.len();

    // read whole file into memory chunk-by-chunk for scanning
    f.seek(SeekFrom::Start(0))?;
    let mut all = Vec::with_capacity(min(total_size as usize, 16_000_000));
    f.read_to_end(&mut all)?;

    let mut pos = 0usize;

    // skip ID3v2 if present
    if all.len() >= 10 && &all[0..3] == b"ID3" {
        let tag_size = helpers::synchsafe_to_u32(&all[6..10]) as usize;
        pos = 10 + tag_size;
    }

    // helper tables
    let bitrate_table_mpeg1_layer3: [u32; 16] = [0,32,40,48,56,64,80,96,112,128,160,192,224,256,320,0];
    let bitrate_table_mpeg2_layer3: [u32; 16] = [0,8,16,24,32,40,48,56,64,80,96,112,128,144,160,0];

    let mut total_samples: u128 = 0;
    let mut last_sample_rate: u32 = 0;

    // To avoid pathological loops, set a max iterations proportional to file size.
    let max_iterations = all.len() * 2;

    let mut iterations = 0usize;
    while pos + 4 <= all.len() && iterations < max_iterations {
        iterations += 1;

        let b1 = all[pos];
        let b2 = all[pos + 1];

        // sync: 11 bits set -> first byte 0xFF and top 3 bits of second are 1 (0xE0)
        if b1 == 0xFF && (b2 & 0xE0) == 0xE0 {
            if pos + 4 > all.len() {
                break;
            }
            let header = &all[pos..pos+4];
            let version_bits = (header[1] >> 3) & 0x03;
            let layer_bits = (header[1] >> 1) & 0x03;
            let bitrate_index = (header[2] >> 4) & 0x0F;
            let sample_rate_index = (header[2] >> 2) & 0x03;
            let padding = ((header[2] >> 1) & 0x01) as u32;
            // channel mode (for Xing offset heuristics if needed)
            // let channel_mode = (header[3] >> 6) & 0x03;

            // determine MPEG version
            // 00 -> MPEG 2.5, 01 -> reserved, 10 -> MPEG2, 11 -> MPEG1
            let mpeg_version = match version_bits {
                0 => 2.5,
                2 => 2.0,
                3 => 1.0,
                _ => {
                    // reserved — treat as invalid
                    pos += 1;
                    continue;
                }
            };

            // determine layer (we only handle Layer III here; if not layer III try to skip)
            let layer = match layer_bits {
                1 => 3, // layer III
                2 => 2,
                3 => 1,
                _ => {
                    pos += 1;
                    continue;
                }
            };

            // We only reliably support Layer III; if not layer III, try to parse generically but be cautious.
            if layer != 3 {
                // attempt to skip non-layer-III frames: advance by 1 and continue (lenient)
                pos += 1;
                continue;
            }

            // sample rate mapping
            let sample_rate = match mpeg_version {
                1.0 => match sample_rate_index {
                    0 => 44100u32,
                    1 => 48000u32,
                    2 => 32000u32,
                    _ => { pos += 1; continue; }
                },
                2.0 => match sample_rate_index {
                    0 => 22050u32,
                    1 => 24000u32,
                    2 => 16000u32,
                    _ => { pos += 1; continue; }
                },
                2.5 => match sample_rate_index {
                    0 => 11025u32,
                    1 => 12000u32,
                    2 => 8000u32,
                    _ => { pos += 1; continue; }
                },
                _ => { pos += 1; continue; }
            };

            // bitrate (kbps)
            let bitrate_kbps = if mpeg_version == 1.0  {
                // MPEG1
                bitrate_table_mpeg1_layer3.get(bitrate_index as usize).copied().unwrap_or(0)
            } else {
                // MPEG2/2.5
                bitrate_table_mpeg2_layer3.get(bitrate_index as usize).copied().unwrap_or(0)
            };

            if bitrate_kbps == 0 || sample_rate == 0 {
                // invalid header values; skip 1 byte and continue (lenient)
                pos += 1;
                continue;
            }

            // compute frame length in bytes for Layer III
            // formula:
            // MPEG1 Layer III: frame_size = floor(144000 * bitrate_kbps / sample_rate) + padding
            // MPEG2/2.5 Layer III: frame_size = floor(72000 * bitrate_kbps / sample_rate) + padding
            let frame_size = if mpeg_version == 1.0  {
                ((144000u32 * bitrate_kbps) / sample_rate) + padding
            } else {
                ((72000u32 * bitrate_kbps) / sample_rate) + padding
            } as usize;

            if frame_size == 0 {
                pos += 1;
                continue;
            }

            // samples per frame
            let samples_per_frame = if mpeg_version == 1.0  {
                1152u32
            } else {
                576u32
            };

            // Sanity: ensure we won't overflow and that frame fits
            if pos + frame_size > all.len() {
                // If frame would extend past EOF, break
                // but still add the final partial frame's samples proportionally? We'll stop.
                break;
            }

            // accumulate
            total_samples += samples_per_frame as u128;
            last_sample_rate = sample_rate;
            // advance by frame_size
            pos += frame_size;
        } else {
            // no sync — lenient: advance by 1
            pos += 1;
        }
    }

    // If we parsed frames and have a sample rate, compute duration
    if total_samples > 0 && last_sample_rate > 0 {
        let duration_ms = (total_samples * 1000u128) / (last_sample_rate as u128);
        // clamp to u64
        let duration_u64 = if duration_ms > u128::from(u64::MAX) {
            u64::MAX
        } else {
            duration_ms as u64
        };
        return Ok(duration_u64);
    }

    // fallback: estimate using file size and a typical bitrate (128kbps)
    if total_size > 0 {
        let audio_bytes = total_size;
        let bitrate = 128_000u64; // bits per second
        let duration_ms = (audio_bytes * 8 * 1000) / bitrate;
        return Ok(duration_ms);
    }

    Err(io::Error::new(io::ErrorKind::InvalidData, "Could not determine MP3 duration"))
}