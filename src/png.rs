use crate::chunk::Chunk;
use byteorder::ByteOrder;

pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    const STATIC_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Self { chunks }
    }
    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks[..]
    }
    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk)
    }
    pub fn remove_chunk(&mut self, chunk_type: &str) -> crate::Result<Chunk> {
        // TODO try to use find_map()
        let index = self
            .chunks
            .iter()
            .enumerate()
            .find(|(i, chunk)| chunk.chunk_type().to_string() == chunk_type)
            .map(|(i, chunk)| i)
            .ok_or(String::from(format!(
                "Chunk with type {} not found",
                chunk_type
            )))?;
        Ok(self.chunks.remove(index))
    }
    pub fn header(&self) -> &[u8; 8] {
        &Self::STATIC_HEADER
    }
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks
            .iter()
            .find(|chunk| chunk.chunk_type().to_string() == chunk_type)
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        Self::STATIC_HEADER
            .iter()
            .copied()
            .chain(self.chunks.iter().flat_map(|chunk| chunk.as_bytes()))
            .collect()
    }
    // Display
}

impl std::convert::TryFrom<&[u8]> for Png {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Png, Self::Error> {
        let mut bytes_iter = bytes.iter();
        let header: Vec<u8> = bytes_iter
            .by_ref()
            .take(Self::STATIC_HEADER.len())
            .copied()
            .collect();
        if header != Self::STATIC_HEADER {
            return Err("Header does not equal to PNG file signature".to_owned());
        }
        loop {
            let length: Vec<u8> = bytes_iter.by_ref().take(4).copied().collect();
            if length.len() == 0 {
                break;
            }
            if length.len() != 4 {
                return Err("Not enough bytes to parse length".to_owned());
            }
            let length = byteorder::BigEndian::read_u32(&length[..]);
            todo!("read length + 8 bytes and decode chunk")
        }
        Err("".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::Chunk;
    use crate::chunk_type::ChunkType;
    use std::convert::TryFrom;
    use std::str::FromStr;

    fn testing_chunks() -> Vec<Chunk> {
        vec![
            chunk_from_strings("FrSt", "I am the first chunk").unwrap(),
            chunk_from_strings("miDl", "I am another chunk").unwrap(),
            chunk_from_strings("LASt", "I am the last chunk").unwrap(),
        ]
    }

    fn testing_png() -> Png {
        let chunks = testing_chunks();
        Png::from_chunks(chunks)
    }

    fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk, String> {
        let chunk_type = ChunkType::from_str(chunk_type)?;
        let data: Vec<u8> = data.bytes().collect();

        Ok(Chunk::new(chunk_type, &data[..]))
    }

    #[test]
    fn test_from_chunks() {
        let chunks = testing_chunks();
        let png = Png::from_chunks(chunks);

        assert_eq!(png.chunks().len(), 3);
    }
}
