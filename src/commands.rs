use std::fs;
use std::str::FromStr;

use crate::args;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png;
use std::io::{Read, Write};

fn print_chunk_to_stdout(chunk: &Chunk, raw: bool) -> crate::Result<()> {
    if raw {
        let mut out = std::io::stdout();
        out.write_all(chunk.data())?;
        out.flush()?;
    } else {
        println!("{}", chunk);
        let data = match chunk.data_as_string() {
            Ok(s) => format!("Data: {}", s),
            Err(_) => "Could not parse data as UTF-8".to_string(),
        };
        println!("{}", data);
    }
    Ok(())
}

pub fn print(cmd: args::Print) -> crate::Result<()> {
    let image = png::Png::from_file(&cmd.file_path)?;
    let indent = " ".repeat(4);
    for (i, chunk) in image.chunks().iter().enumerate() {
        println!("[{}] {}", i + 1, chunk);
        if cmd.verbose {
            let chunk_type = chunk.chunk_type();
            let is_reserved_bit_valid = chunk_type.is_reserved_bit_valid();
            let is_safe_to_copy = chunk_type.is_safe_to_copy();
            println!("{}is critical: {}", indent, chunk_type.is_critical());
            println!("{}is public: {}", indent, chunk_type.is_public());
            println!("{}has valid reserve bit: {}", indent, is_reserved_bit_valid);
            println!("{}is safe to copy: {}", indent, is_safe_to_copy);
            println!("{}crc as dec: {}", indent, chunk.crc());
            println!("{}crc as hex: {:x}", indent, chunk.crc());
        }
    }
    Ok(())
}

pub fn decode(cmd: args::Decode) -> crate::Result<()> {
    let image = png::Png::from_file(&cmd.file_path)?;
    let chunk_type = ChunkType::from_str(&cmd.chunk_type)?;
    let chunk = image.chunk_by_type(&chunk_type);
    match chunk {
        None => {
            println!("Chunk with type {:?} not found", cmd.chunk_type);
        }
        Some(chunk) => print_chunk_to_stdout(chunk, cmd.raw)?,
    }
    Ok(())
}

pub fn remove(cmd: args::Remove) -> crate::Result<()> {
    let mut image = png::Png::from_file(&cmd.file_path)?;
    let chunk_type = ChunkType::from_str(&cmd.chunk_type)?;
    let chunk = image.remove_chunk(&chunk_type)?;
    print_chunk_to_stdout(&chunk, cmd.raw)?;
    let output_path = cmd.output_file.unwrap_or(cmd.file_path);
    fs::write(output_path, image.as_bytes())?;
    Ok(())
}

pub fn encode(cmd: args::Encode) -> crate::Result<()> {
    let mut image = png::Png::from_file(&cmd.file_path)?;
    let chunk_type = ChunkType::from_str(&cmd.chunk_type)?;
    let buf: Vec<u8> = match &cmd.message {
        None => {
            let mut buf = Vec::new();
            std::io::stdin().lock().read_to_end(&mut buf)?;
            buf
        }
        // TODO when string is present also check nothing is piped on stdin
        Some(string) => string.as_bytes().iter().copied().collect(),
    };
    image.append_chunk(Chunk::new(chunk_type, &buf));
    let output_path = cmd.output_file.unwrap_or(cmd.file_path);
    fs::write(output_path, image.as_bytes())?;
    Ok(())
}
