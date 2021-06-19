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
        let mut i = value.iter();
        let length: Vec<u8> = i.by_ref().take(4).copied().collect();
        let chunk_type: Vec<u8> = i.by_ref().take(4).copied().collect();
        let data: Vec<u8> = i
            .by_ref()
            .take(value.len().checked_sub(4 * 3).unwrap_or(0))
            .copied()
            .collect();
        let crc: Vec<u8> = i.by_ref().copied().collect();
        let mut data: Vec<u8> = value.iter().copied().collect();
        let length: Vec<u8> = data.drain(..4.min(data.len())).collect();
        let type_vector = value.iter().take(4).copied().collect::<Vec<u8>>();
        let type_bytes: [u8; 4] = <[u8; 4]>::try_from(type_vector)
            .or(Err("Not enough elements to parse type".to_owned()))?;
        let chunk_type = ChunkType::try_from(type_bytes)?;
        let data = value.iter().skip(4).copied().collect::<Vec<u8>>();
        let length = data.len() as u32;
        let crc = ::crc::crc32::checksum_ieee(&data);
        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}
