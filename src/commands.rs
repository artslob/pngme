use crate::args;
use crate::chunk::Chunk;
use crate::png;
use std::convert::TryFrom;
use std::error::Error;
use std::fs;

fn read_png(file_path: String) -> crate::Result<png::Png> {
    let bytes = fs::read(file_path)?;
    let image = png::Png::try_from(&bytes[..])?;
    Ok(image)
}

pub fn print(cmd: args::Print) -> crate::Result<()> {
    let image = read_png(cmd.file_path)?;
    for (i, chunk) in image.chunks().iter().enumerate() {
        println!("[{}] {}", i + 1, chunk);
    }
    Ok(())
}

pub fn decode(cmd: args::Decode) -> crate::Result<()> {
    let image = read_png(cmd.file_path)?;
    let chunk = image.chunk_by_type(&cmd.chunk_type);
    match chunk {
        None => {
            println!("Chunk with type {:?} not found", cmd.chunk_type);
        }
        Some(chunk) => {
            println!("{}", chunk);
            match chunk.data_as_string() {
                Ok(s) => println!("Data: {}", s),
                Err(e) => {
                    println!("Data: <could not parse data as UTF-8>")
                }
            }
        }
    }
    Ok(())
}
