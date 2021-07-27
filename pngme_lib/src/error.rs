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

#[derive(Debug)]
pub enum ChunkParseError {
    ChunkTypeParseError(ChunkTypeParseError),
    NotEnoughBytesToParseLength,
    NotEnoughBytesToParseType,
    LengthDoesNotFitToU32,
    EncodedLengthNotEqualToActual,
    CouldNotParseCrc,
    CrcMismatch,
}

impl From<ChunkTypeParseError> for ChunkParseError {
    fn from(err: ChunkTypeParseError) -> ChunkParseError {
        ChunkParseError::ChunkTypeParseError(err)
    }
}

impl From<ChunkParseError> for String {
    fn from(err: ChunkParseError) -> String {
        "".to_owned()
    }
}

impl fmt::Display for ChunkParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl std::error::Error for ChunkParseError {}
