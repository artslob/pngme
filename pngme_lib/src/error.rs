use std::fmt;

#[derive(Debug)]
pub enum ChunkTypeParseError {
    NotAsciiChar(char),
    FromStrInvalidNumberOfChars(usize),
}

impl From<ChunkTypeParseError> for String {
    fn from(err: ChunkTypeParseError) -> String {
        "".to_owned()
    }
}

impl fmt::Display for ChunkTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl std::error::Error for ChunkTypeParseError {}
