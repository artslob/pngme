use crate::chunk_type::ChunkType;

struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl std::convert::TryFrom<&[u8]> for Chunk {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let type_vector = value.iter().take(4).copied().collect::<Vec<u8>>();
        let type_bytes: [u8; 4] = <[u8; 4]>::try_from(type_vector).or(Err("".to_owned()))?;
        let chunk_type = ChunkType::try_from(type_bytes)?;
        let data = value.iter().skip(4).copied().collect::<Vec<u8>>();
        let length = data.len() as u32;
        // TODO calc crc
        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc: 0,
        })
    }
}
