use crate::args;
use crate::png;
use std::convert::TryFrom;
use std::fs;

pub fn print(cmd: args::Print) -> crate::Result<()> {
    let bytes = fs::read(cmd.file_path)?;
    let image = png::Png::try_from(&bytes[..])?;
    for (i, chunk) in image.chunks().iter().enumerate() {
        println!("[{}] {}", i + 1, chunk);
    }
    Ok(())
}
