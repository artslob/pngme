use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

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
    // pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {}
    pub fn header(&self) -> &[u8; 8] {
        &Self::STATIC_HEADER
    }
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks
            .iter()
            .find(|chunk| chunk.chunk_type().to_string() == chunk_type)
    }
    // pub fn as_bytes(&self) -> Vec<u8> {}
    // TryFrom<&[u8]>
    // Display
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
