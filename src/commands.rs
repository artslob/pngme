use crate::args;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png;
use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

fn read_png(file_path: &str) -> crate::Result<png::Png> {
    let bytes = fs::read(file_path)?;
    let image = png::Png::try_from(&bytes[..])?;
    Ok(image)
}

fn data_as_string(chunk: &Chunk) -> String {
    match chunk.data_as_string() {
        Ok(s) => format!("Data: {}", s),
        Err(_) => "Could not parse data as UTF-8".to_string(),
    }
}

pub fn print(cmd: args::Print) -> crate::Result<()> {
    let image = read_png(&cmd.file_path)?;
    for (i, chunk) in image.chunks().iter().enumerate() {
        println!("[{}] {}", i + 1, chunk);
    }
    Ok(())
}

pub fn decode(cmd: args::Decode) -> crate::Result<()> {
    let image = read_png(&cmd.file_path)?;
    let chunk_type = ChunkType::from_str(&cmd.chunk_type)?;
    let chunk = image.chunk_by_type(&chunk_type);
    match chunk {
        None => {
            println!("Chunk with type {:?} not found", cmd.chunk_type);
        }
        Some(chunk) => {
            println!("{}", chunk);
            println!("{}", data_as_string(chunk));
        }
    }
    Ok(())
}

pub fn remove(cmd: args::Remove) -> crate::Result<()> {
    let mut image = read_png(&cmd.file_path)?;
    let chunk_type = ChunkType::from_str(&cmd.chunk_type)?;
    let chunk = image.remove_chunk(&chunk_type)?;
    println!("{}", chunk);
    println!("{}", data_as_string(&chunk));
    let output_path = cmd.output_file.unwrap_or(cmd.file_path);
    fs::write(output_path, image.as_bytes())?;
    Ok(())
}

pub fn encode(cmd: args::Encode) -> crate::Result<()> {
    let mut image = read_png(&cmd.file_path)?;
    let chunk_type = ChunkType::from_str(&cmd.chunk_type)?;
    image.append_chunk(Chunk::new(chunk_type, cmd.message.as_bytes()));
    let output_path = cmd.output_file.unwrap_or(cmd.file_path);
    fs::write(output_path, image.as_bytes())?;
    Ok(())
}
