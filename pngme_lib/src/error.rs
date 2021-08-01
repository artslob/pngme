// Error handling implemented like in https://blog.burntsushi.net/rust-error-handling
use std::fmt;

#[derive(Debug)]
pub enum ChunkTypeParseError {
    NotAsciiChar(char),
    FromStrInvalidNumberOfChars(usize),
}

impl fmt::Display for ChunkTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkTypeParseError::NotAsciiChar(ch) => {
                write!(f, "char should be ascii letter: {}", ch)
            }
            ChunkTypeParseError::FromStrInvalidNumberOfChars(number) => {
                write!(f, "string should contain 4 chars, got {} chars", number)
            }
        }
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

impl fmt::Display for ChunkParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkParseError::ChunkTypeParseError(e) => e.fmt(f),
            ChunkParseError::NotEnoughBytesToParseLength => {
                write!(f, "Not enough bytes to parse length")
            }
            ChunkParseError::NotEnoughBytesToParseType => {
                write!(f, "Not enough elements to parse type")
            }
            ChunkParseError::LengthDoesNotFitToU32 => write!(f, "Could not parse length of data"),
            ChunkParseError::EncodedLengthNotEqualToActual => {
                write!(f, "Length of data not equal to encoded length")
            }
            ChunkParseError::CouldNotParseCrc => write!(f, "Could not parse crc"),
            ChunkParseError::CrcMismatch => write!(f, "Decoded crc not equal to calculated crc"),
        }
    }
}

impl std::error::Error for ChunkParseError {}

#[derive(Debug)]
pub enum RemoveChunkError {
    NotFound(String),
}

impl fmt::Display for RemoveChunkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RemoveChunkError::NotFound(chunk_type) => {
                write!(f, "Chunk with type {} not found", chunk_type)
            }
        }
    }
}

impl std::error::Error for RemoveChunkError {}

#[derive(Debug)]
pub enum PngFromBytesError {
    InvalidHeader,
    LengthParse,
    ChunkParseError(ChunkParseError),
}

impl fmt::Display for PngFromBytesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidHeader => {
                write!(f, "Header does not equal to PNG file signature")
            }
            Self::LengthParse => write!(f, "Not enough bytes to parse length"),
            Self::ChunkParseError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for PngFromBytesError {}

impl From<ChunkParseError> for PngFromBytesError {
    fn from(err: ChunkParseError) -> Self {
        Self::ChunkParseError(err)
    }
}

#[derive(Debug)]
pub enum PngFromFileError {
    FileReadError(std::io::Error),
    PngFromBytesError(PngFromBytesError),
}

impl fmt::Display for PngFromFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PngFromFileError::FileReadError(e) => e.fmt(f),
            PngFromFileError::PngFromBytesError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for PngFromFileError {}

impl From<std::io::Error> for PngFromFileError {
    fn from(err: std::io::Error) -> Self {
        Self::FileReadError(err)
    }
}

impl From<PngFromBytesError> for PngFromFileError {
    fn from(err: PngFromBytesError) -> Self {
        Self::PngFromBytesError(err)
    }
}
