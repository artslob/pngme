use crate::chunk_type::ChunkType;
use byteorder::ByteOrder;
use std::convert::TryInto;

#[derive(::derive_more::Display)]
#[display(fmt = "Chunk \"{}\" len:{}", chunk_type, length)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: &[u8]) -> Self {
        let data_for_crc: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let crc = ::crc::crc32::checksum_ieee(&data_for_crc[..]);
        Self {
            length: data.len() as u32,
            chunk_type,
            data: data.iter().copied().collect(),
            crc,
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> crate::Result<String> {
        String::from_utf8(self.data.clone())
            .or_else(|x| Err(Box::new(x) as Box<dyn std::error::Error>))
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl std::convert::TryFrom<&[u8]> for Chunk {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut bytes_iter = bytes.iter();
        let length: Vec<u8> = bytes_iter.by_ref().take(4).copied().collect();
        if length.len() != 4 {
            return Err("Not enough bytes to parse length".to_owned());
        }
        let length = byteorder::BigEndian::read_u32(&length[..]);
        let type_vector: Vec<u8> = bytes_iter.by_ref().take(4).copied().collect();
        let type_error = Err("Not enough elements to parse type".to_owned());
        let type_bytes: [u8; 4] = <[u8; 4]>::try_from(type_vector).or(type_error)?;
        let chunk_type = ChunkType::try_from(type_bytes)?;
        let data_take_count = bytes.len().checked_sub(4 * 3).unwrap_or(0);
        let data: Vec<u8> = bytes_iter.by_ref().take(data_take_count).copied().collect();
        let data_parse_error = Err("Could not parse length of data".to_owned());
        let data_length: u32 = data.len().try_into().or(data_parse_error)?;
        if length != data_length {
            return Err("Length of data not equal to encoded length".to_owned());
        }
        let crc: Vec<u8> = bytes_iter.by_ref().copied().collect();
        if crc.len() != 4 {
            return Err("Could not parse crc".to_owned());
        }
        let crc = byteorder::BigEndian::read_u32(&crc[..]);
        let data_for_crc: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        if crc != ::crc::crc32::checksum_ieee(&data_for_crc[..]) {
            return Err("Decoded crc not equal to calculated crc".to_owned());
        }
        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::convert::TryFrom;
    use std::str::FromStr;

    fn testing_bytes() -> Vec<u8> {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect()
    }

    fn testing_chunk() -> Chunk {
        let chunk_data = testing_bytes();
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_new() {
        let left = testing_chunk();
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let msg = "This is where your secret message will be!".as_bytes();
        let right = Chunk::new(chunk_type, msg);
        assert_eq!(left.as_bytes(), right.as_bytes());
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_data() {
        let chunk = testing_chunk();
        let msg = "This is where your secret message will be!".as_bytes();
        assert_eq!(chunk.data(), msg);
    }

    #[test]
    fn test_chunk_as_bytes() {
        let chunk = testing_chunk();
        assert_eq!(chunk.as_bytes(), testing_bytes());
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333; // invalid crc

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    fn test_invalid_chunk_by_length() {
        let data_length: u32 = 41;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let chunk = testing_chunk();
        assert_eq!(format!("{}", chunk), "Chunk \"RuSt\" len:42")
    }
}
