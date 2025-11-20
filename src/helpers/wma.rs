
use std::fs::File;
use std::io::{self, Read};

pub fn read_u8(f: &mut File) -> io::Result<u8> {
    let mut b = [0u8; 1];
    f.read_exact(&mut b)?;
    Ok(b[0])
}

pub fn read_u16_le(f: &mut File) -> io::Result<u16> {
    let mut b = [0u8; 2];
    f.read_exact(&mut b)?;
    Ok(u16::from_le_bytes(b))
}

pub fn read_u32_le(f: &mut File) -> io::Result<u32> {
    let mut b = [0u8; 4];
    f.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

pub fn read_u64_le(f: &mut File) -> io::Result<u64> {
    let mut b = [0u8; 8];
    f.read_exact(&mut b)?;
    Ok(u64::from_le_bytes(b))
}
