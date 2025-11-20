use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

pub fn compute(f: &mut File) -> io::Result<u64> {
    f.seek(SeekFrom::Start(0))?;

    let mut sample_rate: Option<u32> = None;
    let mut final_granule: u64 = 0;

    loop {
        let mut hdr = [0u8; 27]; // OggS + header
        if f.read(&mut hdr)? != 27 {
            break; // EOF
        }
        if &hdr[0..4] != b"OggS" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "not ogg"));
        }

        // granule pos = bytes 6..14
        let granule = u64::from_le_bytes(hdr[6..14].try_into().unwrap());

        let segments = hdr[26];
        let mut seg_table = vec![0u8; segments as usize];
        f.read_exact(&mut seg_table)?;

        let packet_size: usize = seg_table.iter().map(|&x| x as usize).sum();
        let mut packet = vec![0u8; packet_size];
        f.read_exact(&mut packet)?;

        // Read sample rate from the Identification Header (packet type 1)
        if sample_rate.is_none() && !packet.is_empty() && packet[0] == 1 {
            if packet.len() >= 12 {
                let sr = u32::from_le_bytes(packet[8..12].try_into().unwrap());
                sample_rate = Some(sr);
            }
        }

        // Granule position on last non -1 page is total samples
        if granule != u64::MAX {
            final_granule = granule;
        }
    }

    let sr = sample_rate.ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "missing sample rate",
    ))?;

    if sr == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid sample rate",
        ));
    }

    Ok((final_granule * 1000) / sr as u64)
}
