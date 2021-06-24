use crate::chunk::Chunk;
use byteorder::ByteOrder;

pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

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
        &Self::STANDARD_HEADER
    }
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks
            .iter()
            .find(|chunk| chunk.chunk_type().to_string() == chunk_type)
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        Self::STANDARD_HEADER
            .iter()
            .copied()
            .chain(self.chunks.iter().flat_map(|chunk| chunk.as_bytes()))
            .collect()
    }
}

impl std::fmt::Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Png with {} chunks", self.chunks.len())
    }
}

impl std::convert::TryFrom<&[u8]> for Png {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Png, Self::Error> {
        let mut bytes_iter = bytes.iter();
        let header: Vec<u8> = bytes_iter
            .by_ref()
            .take(Self::STANDARD_HEADER.len())
            .copied()
            .collect();
        if header != Self::STANDARD_HEADER {
            return Err("Header does not equal to PNG file signature".to_owned());
        }
        let mut chunks: Vec<Chunk> = vec![];
        loop {
            let length: Vec<u8> = bytes_iter.by_ref().take(4).copied().collect();
            if length.len() == 0 {
                break;
            }
            if length.len() != 4 {
                return Err("Not enough bytes to parse length".to_owned());
            }
            let length = byteorder::BigEndian::read_u32(&length[..]);
            let data: Vec<u8> = length
                .to_be_bytes()
                .iter()
                .copied()
                .chain(bytes_iter.by_ref().take((length + 8) as usize).copied())
                .collect();
            chunks.push(Chunk::try_from(&data[..])?)
        }
        Ok(Png::from_chunks(chunks))
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

    fn chunk_from_strings(chunk_type: &str, data: &str) -> crate::Result<Chunk> {
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

    #[test]
    fn test_valid_from_bytes() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = Png::STANDARD_HEADER
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());

        assert!(png.is_ok());
    }

    #[test]
    fn test_invalid_header() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = [13, 80, 78, 71, 13, 10, 26, 10]
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());

        assert!(png.is_err());
    }

    #[test]
    fn test_invalid_chunk() {
        let mut chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        #[rustfmt::skip]
        let mut bad_chunk = vec![
            0, 0, 0, 5,         // length
            32, 117, 83, 116,   // Chunk Type (bad)
            65, 64, 65, 66, 67, // Data
            1, 2, 3, 4, 5       // CRC (bad)
        ];

        chunk_bytes.append(&mut bad_chunk);

        let png = Png::try_from(chunk_bytes.as_ref());

        assert!(png.is_err());
    }

    #[test]
    fn test_list_chunks() {
        let png = testing_png();
        let chunks = png.chunks();
        assert_eq!(chunks.len(), 3);
    }

    #[test]
    fn test_chunk_by_type() {
        let png = testing_png();
        let chunk = png.chunk_by_type("FrSt").unwrap();
        assert_eq!(&chunk.chunk_type().to_string(), "FrSt");
        assert_eq!(&chunk.data_as_string().unwrap(), "I am the first chunk");
    }

    #[test]
    fn test_append_chunk() {
        let mut png = testing_png();
        png.append_chunk(chunk_from_strings("TeSt", "Message").unwrap());
        let chunk = png.chunk_by_type("TeSt").unwrap();
        assert_eq!(&chunk.chunk_type().to_string(), "TeSt");
        assert_eq!(&chunk.data_as_string().unwrap(), "Message");
    }

    #[test]
    fn test_remove_chunk() {
        let mut png = testing_png();
        png.append_chunk(chunk_from_strings("TeSt", "Message").unwrap());
        png.remove_chunk("TeSt").unwrap();
        let chunk = png.chunk_by_type("TeSt");
        assert!(chunk.is_none());
    }

    #[test]
    fn test_png_from_image_file() {
        let png = Png::try_from(&PNG_FILE[..]);
        assert!(png.is_ok());
    }

    #[test]
    fn test_as_bytes() {
        let png = Png::try_from(&PNG_FILE[..]).unwrap();
        let actual = png.as_bytes();
        let expected: Vec<u8> = PNG_FILE.iter().copied().collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_png_trait_impls() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = Png::STANDARD_HEADER
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png: Png = TryFrom::try_from(bytes.as_ref()).unwrap();

        let _png_string = format!("{}", png);
    }

    // This is the raw bytes for a shrunken version of the `dice.png` image on Wikipedia
    const PNG_FILE: [u8; 4803] = [
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 50, 0, 0, 0, 50, 8,
        6, 0, 0, 0, 30, 63, 136, 177, 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28, 233, 0, 0, 0,
        4, 103, 65, 77, 65, 0, 0, 177, 143, 11, 252, 97, 5, 0, 0, 0, 9, 112, 72, 89, 115, 0, 0, 14,
        194, 0, 0, 14, 194, 1, 21, 40, 74, 128, 0, 0, 18, 73, 73, 68, 65, 84, 104, 67, 237, 153,
        121, 112, 28, 213, 157, 199, 95, 223, 215, 76, 207, 61, 26, 29, 163, 251, 180, 101, 27,
        217, 194, 14, 24, 18, 192, 78, 124, 4, 19, 108, 47, 46, 28, 2, 235, 64, 128, 164, 216, 10,
        144, 132, 10, 187, 97, 23, 66, 150, 224, 101, 33, 241, 134, 56, 164, 8, 149, 133, 80, 132,
        212, 134, 36, 101, 112, 184, 109, 108, 3, 190, 176, 177, 124, 201, 146, 172, 123, 164, 153,
        209, 104, 206, 158, 190, 207, 125, 35, 15, 174, 0, 78, 226, 107, 107, 255, 225, 83, 154,
        105, 169, 231, 245, 175, 223, 183, 127, 239, 119, 188, 17, 248, 140, 207, 248, 191, 1, 41,
        31, 207, 138, 95, 253, 202, 64, 28, 199, 193, 186, 186, 28, 36, 22, 67, 204, 53, 107, 72,
        167, 252, 209, 255, 59, 127, 87, 200, 252, 249, 25, 140, 36, 211, 141, 149, 149, 204, 21,
        126, 63, 223, 205, 113, 118, 101, 32, 96, 187, 250, 250, 204, 62, 219, 86, 79, 204, 154,
        229, 236, 227, 121, 170, 231, 222, 123, 171, 236, 242, 37, 23, 196, 236, 253, 179, 177, 97,
        97, 152, 253, 142, 239, 59, 124, 130, 73, 20, 155, 66, 77, 226, 15, 66, 63, 248, 187, 182,
        255, 170, 144, 133, 11, 139, 136, 219, 93, 104, 232, 234, 178, 239, 88, 186, 20, 95, 223,
        211, 35, 241, 83, 83, 110, 98, 122, 90, 22, 178, 89, 32, 48, 140, 68, 121, 60, 56, 151,
        201, 232, 169, 66, 193, 217, 124, 240, 32, 247, 235, 98, 177, 69, 43, 95, 126, 206, 224,
        219, 113, 242, 6, 249, 134, 197, 152, 138, 221, 128, 34, 232, 60, 138, 164, 124, 125, 118,
        223, 177, 154, 234, 154, 15, 163, 222, 232, 214, 42, 79, 85, 223, 61, 161, 123, 204, 242,
        240, 79, 113, 70, 33, 87, 95, 157, 32, 114, 57, 97, 89, 103, 167, 252, 111, 179, 102, 113,
        205, 24, 70, 40, 111, 188, 17, 183, 17, 4, 101, 56, 206, 139, 88, 22, 158, 33, 8, 17, 165,
        105, 18, 27, 30, 182, 210, 46, 23, 194, 20, 139, 214, 179, 137, 132, 231, 191, 38, 38, 26,
        140, 178, 153, 179, 98, 233, 177, 165, 96, 215, 248, 174, 106, 79, 214, 115, 243, 114, 124,
        249, 183, 166, 205, 105, 29, 135, 180, 122, 90, 107, 194, 174, 176, 132, 122, 80, 185, 183,
        216, 59, 21, 166, 195, 127, 226, 66, 220, 230, 7, 107, 31, 204, 148, 47, 253, 24, 88, 249,
        120, 154, 245, 235, 227, 30, 65, 16, 190, 169, 105, 234, 109, 166, 233, 240, 3, 3, 154, 60,
        50, 34, 186, 38, 38, 178, 240, 40, 244, 231, 114, 138, 104, 24, 42, 238, 114, 49, 142, 36,
        97, 208, 3, 168, 77, 146, 24, 176, 44, 164, 197, 231, 179, 147, 0, 60, 218, 95, 40, 60, 90,
        182, 246, 183, 193, 63, 192, 81, 103, 220, 89, 184, 88, 94, 188, 9, 232, 96, 225, 170, 186,
        85, 254, 110, 119, 55, 63, 150, 31, 3, 83, 106, 82, 200, 129, 124, 225, 150, 198, 91, 170,
        71, 147, 163, 129, 162, 93, 92, 16, 147, 70, 67, 95, 186, 247, 11, 7, 247, 252, 226, 128,
        84, 54, 113, 26, 180, 124, 156, 97, 221, 186, 137, 202, 88, 172, 240, 160, 32, 104, 55,
        240, 60, 234, 13, 6, 17, 111, 91, 27, 168, 96, 89, 196, 193, 48, 102, 202, 239, 199, 41,
        154, 54, 213, 234, 106, 194, 148, 36, 85, 86, 20, 197, 90, 177, 130, 10, 111, 218, 20, 108,
        111, 106, 34, 188, 8, 226, 220, 118, 233, 165, 133, 202, 178, 185, 191, 9, 249, 62, 73, 16,
        99, 196, 114, 46, 199, 253, 136, 113, 152, 121, 56, 134, 219, 203, 91, 151, 251, 90, 131,
        173, 204, 135, 242, 193, 209, 183, 210, 111, 247, 158, 72, 157, 16, 239, 223, 121, 255,
        228, 174, 248, 174, 244, 120, 113, 220, 144, 29, 109, 137, 37, 17, 255, 186, 117, 116, 171,
        167, 108, 230, 52, 167, 61, 114, 215, 93, 147, 1, 77, 19, 126, 154, 201, 152, 11, 108, 27,
        1, 245, 245, 118, 133, 166, 233, 70, 99, 35, 111, 173, 92, 201, 176, 138, 146, 113, 119,
        117, 5, 107, 91, 224, 115, 207, 102, 109, 69, 85, 29, 195, 182, 109, 103, 245, 234, 112,
        120, 209, 162, 144, 107, 104, 40, 79, 136, 34, 234, 5, 192, 18, 106, 107, 31, 218, 63, 56,
        248, 88, 217, 242, 167, 121, 226, 196, 19, 88, 229, 116, 229, 173, 205, 106, 243, 35, 189,
        74, 175, 116, 68, 59, 50, 129, 88, 208, 181, 162, 197, 110, 137, 189, 60, 190, 39, 183, 39,
        105, 58, 166, 217, 68, 54, 249, 18, 118, 34, 155, 52, 146, 169, 38, 186, 137, 169, 97, 107,
        60, 134, 104, 206, 41, 18, 197, 248, 45, 27, 111, 233, 217, 178, 113, 75, 217, 226, 95,
        120, 36, 16, 156, 254, 6, 207, 203, 43, 1, 112, 128, 3, 127, 250, 251, 145, 201, 84, 10,
        164, 187, 186, 48, 230, 248, 241, 156, 134, 97, 1, 184, 246, 253, 83, 71, 143, 106, 147,
        197, 162, 173, 148, 198, 192, 203, 236, 23, 95, 156, 74, 253, 252, 231, 137, 137, 201, 73,
        21, 116, 116, 144, 117, 12, 163, 173, 103, 24, 33, 116, 202, 234, 153, 145, 37, 249, 82,
        66, 35, 30, 32, 0, 193, 123, 113, 47, 179, 130, 88, 209, 101, 91, 14, 249, 31, 67, 143,
        245, 252, 41, 241, 167, 49, 152, 226, 103, 178, 84, 5, 30, 246, 47, 171, 88, 214, 54, 219,
        53, 187, 50, 202, 69, 137, 13, 13, 27, 60, 130, 89, 52, 70, 114, 35, 95, 59, 54, 117, 172,
        102, 198, 88, 153, 25, 143, 60, 188, 113, 130, 220, 181, 99, 228, 251, 147, 19, 182, 203,
        231, 179, 88, 77, 51, 16, 28, 215, 185, 138, 10, 221, 115, 252, 184, 204, 13, 12, 152, 124,
        91, 91, 88, 234, 235, 43, 10, 146, 100, 153, 80, 196, 105, 178, 89, 221, 60, 118, 76, 87,
        37, 137, 144, 88, 214, 164, 198, 198, 12, 85, 85, 237, 241, 145, 145, 95, 28, 47, 15, 249,
        24, 179, 142, 205, 66, 134, 226, 67, 255, 176, 44, 188, 108, 201, 170, 200, 42, 223, 72,
        124, 196, 55, 169, 78, 40, 115, 61, 243, 176, 16, 8, 97, 35, 218, 72, 177, 60, 20, 100,
        141, 76, 113, 65, 100, 1, 231, 35, 125, 252, 246, 233, 237, 147, 136, 131, 16, 41, 109, 74,
        62, 152, 59, 168, 166, 244, 84, 182, 227, 225, 142, 35, 195, 155, 134, 103, 198, 206, 120,
        132, 197, 113, 250, 115, 139, 162, 161, 75, 46, 9, 90, 93, 93, 94, 230, 202, 43, 189, 158,
        75, 46, 97, 104, 93, 71, 249, 247, 223, 23, 92, 146, 100, 218, 153, 140, 162, 228, 243, 58,
        20, 113, 202, 99, 37, 49, 56, 14, 208, 150, 22, 156, 102, 89, 133, 133, 158, 196, 62, 248,
        192, 56, 89, 40, 96, 138, 105, 162, 95, 250, 198, 55, 82, 212, 204, 29, 62, 1, 237, 208,
        148, 102, 104, 45, 181, 238, 90, 130, 161, 24, 100, 84, 25, 29, 242, 123, 2, 169, 31, 95,
        253, 227, 134, 111, 54, 127, 179, 141, 4, 36, 122, 185, 239, 242, 192, 179, 93, 207, 94,
        222, 66, 183, 114, 56, 77, 104, 221, 77, 221, 64, 210, 37, 117, 75, 124, 75, 220, 141, 185,
        253, 77, 92, 83, 0, 215, 240, 107, 169, 28, 85, 81, 54, 123, 74, 72, 119, 55, 237, 84, 87,
        147, 228, 146, 37, 84, 21, 77, 227, 106, 125, 61, 137, 85, 85, 185, 157, 171, 174, 138, 20,
        87, 172, 8, 76, 235, 186, 101, 196, 98, 34, 92, 78, 167, 124, 81, 122, 143, 70, 201, 224,
        205, 55, 7, 219, 159, 120, 162, 181, 125, 225, 66, 134, 96, 89, 45, 140, 162, 122, 21, 142,
        155, 181, 209, 40, 118, 249, 229, 151, 131, 51, 6, 189, 199, 246, 32, 173, 104, 107, 213,
        51, 71, 127, 133, 110, 60, 254, 168, 166, 80, 114, 41, 221, 98, 9, 33, 161, 31, 200, 30,
        72, 233, 166, 110, 109, 104, 219, 80, 183, 124, 246, 114, 255, 92, 207, 92, 31, 131, 49,
        152, 155, 113, 179, 87, 133, 174, 234, 40, 168, 5, 13, 198, 212, 248, 101, 254, 203, 248,
        251, 219, 238, 191, 154, 23, 249, 165, 101, 179, 167, 132, 56, 14, 33, 230, 114, 224, 173,
        98, 81, 48, 231, 207, 87, 67, 161, 144, 200, 45, 92, 168, 133, 154, 154, 21, 239, 250, 245,
        46, 238, 139, 95, 12, 73, 193, 32, 30, 44, 9, 40, 121, 3, 94, 130, 132, 195, 152, 63, 16,
        32, 89, 158, 119, 3, 4, 193, 205, 169, 41, 113, 2, 86, 252, 10, 152, 178, 9, 89, 118, 201,
        125, 125, 31, 207, 136, 31, 209, 70, 183, 233, 181, 116, 237, 20, 137, 81, 102, 17, 17,
        180, 90, 190, 174, 106, 76, 25, 147, 87, 190, 186, 242, 221, 199, 7, 31, 239, 45, 153, 127,
        101, 228, 149, 196, 75, 189, 47, 197, 119, 102, 119, 78, 201, 178, 140, 13, 77, 13, 17,
        240, 188, 63, 140, 134, 91, 10, 122, 193, 126, 35, 253, 70, 2, 122, 21, 33, 77, 242, 154,
        89, 189, 179, 102, 60, 63, 19, 35, 207, 61, 247, 99, 176, 118, 237, 247, 196, 190, 62, 227,
        202, 124, 30, 115, 226, 113, 90, 178, 109, 29, 141, 197, 76, 224, 114, 153, 152, 101, 249,
        68, 130, 64, 184, 120, 92, 23, 76, 19, 192, 100, 101, 57, 178, 44, 106, 195, 195, 138, 180,
        99, 71, 62, 181, 111, 95, 49, 79, 16, 56, 21, 141, 210, 65, 89, 230, 166, 124, 62, 178,
        208, 217, 137, 191, 240, 234, 171, 143, 137, 37, 251, 127, 201, 129, 159, 31, 112, 208, 91,
        208, 236, 120, 110, 188, 19, 78, 78, 171, 34, 170, 194, 162, 38, 138, 49, 37, 86, 48, 17,
        211, 65, 80, 4, 25, 144, 6, 196, 215, 18, 175, 37, 60, 136, 135, 242, 96, 158, 144, 37, 91,
        212, 137, 68, 239, 73, 65, 42, 228, 97, 125, 35, 34, 120, 36, 244, 86, 250, 173, 84, 76,
        143, 9, 13, 190, 134, 215, 7, 158, 26, 80, 79, 63, 181, 198, 70, 247, 135, 110, 55, 183,
        47, 159, 39, 52, 195, 192, 236, 201, 184, 91, 178, 29, 132, 237, 239, 231, 243, 166, 137,
        219, 30, 143, 75, 10, 4, 48, 174, 180, 186, 74, 66, 146, 73, 61, 55, 48, 160, 79, 29, 57,
        162, 64, 111, 81, 21, 62, 31, 211, 112, 224, 128, 0, 45, 57, 136, 207, 231, 156, 240, 251,
        209, 233, 196, 162, 69, 204, 208, 220, 185, 243, 7, 111, 188, 241, 142, 125, 115, 230, 60,
        56, 186, 120, 241, 166, 158, 175, 126, 245, 215, 191, 124, 54, 188, 250, 250, 163, 150,
        136, 37, 198, 243, 58, 208, 38, 31, 91, 240, 88, 199, 134, 138, 13, 45, 51, 19, 41, 247,
        26, 176, 182, 16, 152, 134, 225, 74, 70, 70, 119, 158, 220, 121, 98, 215, 244, 187, 67,
        131, 250, 80, 50, 138, 214, 18, 180, 66, 219, 240, 60, 112, 4, 7, 232, 150, 62, 211, 22,
        157, 22, 178, 98, 133, 79, 139, 70, 185, 231, 224, 122, 85, 112, 28, 65, 49, 20, 5, 193,
        128, 27, 5, 136, 201, 194, 95, 1, 73, 226, 118, 125, 61, 55, 227, 70, 184, 148, 16, 151,
        139, 38, 151, 45, 243, 84, 52, 53, 241, 145, 198, 70, 46, 136, 162, 238, 100, 83, 19, 151,
        89, 48, 91, 97, 174, 236, 221, 124, 162, 237, 95, 230, 63, 82, 48, 140, 63, 7, 127, 246,
        179, 237, 174, 123, 239, 221, 236, 106, 107, 187, 193, 242, 251, 215, 194, 181, 184, 150,
        116, 216, 245, 183, 29, 140, 214, 60, 243, 155, 8, 251, 237, 183, 24, 180, 133, 173, 4, 94,
        198, 75, 148, 108, 99, 0, 67, 230, 146, 115, 35, 215, 179, 95, 89, 232, 118, 220, 236, 31,
        166, 255, 248, 193, 33, 227, 80, 220, 102, 109, 228, 10, 247, 226, 150, 102, 190, 169, 122,
        64, 29, 136, 31, 87, 122, 135, 3, 172, 255, 85, 193, 45, 40, 51, 115, 42, 189, 125, 196,
        59, 239, 8, 196, 193, 131, 194, 227, 4, 97, 111, 48, 12, 32, 153, 150, 198, 1, 135, 70, 41,
        10, 203, 160, 168, 83, 106, 67, 192, 107, 175, 141, 103, 68, 209, 177, 174, 189, 214, 91,
        113, 247, 221, 117, 209, 151, 94, 18, 114, 219, 183, 43, 146, 63, 27, 47, 94, 129, 188,
        131, 205, 51, 123, 76, 76, 203, 26, 66, 60, 46, 185, 59, 59, 67, 26, 207, 51, 234, 225,
        195, 150, 146, 201, 20, 36, 93, 71, 58, 159, 124, 178, 5, 239, 232, 160, 83, 47, 190, 56,
        170, 109, 219, 38, 90, 233, 180, 40, 96, 182, 246, 208, 237, 128, 58, 142, 102, 138, 152,
        137, 33, 141, 68, 99, 144, 7, 110, 238, 164, 62, 152, 172, 164, 43, 185, 90, 62, 202, 142,
        43, 49, 51, 99, 166, 69, 5, 168, 70, 220, 152, 44, 216, 156, 115, 4, 107, 196, 158, 176,
        150, 90, 185, 210, 220, 63, 214, 107, 61, 247, 220, 163, 246, 157, 119, 62, 56, 172, 170,
        214, 210, 174, 46, 169, 209, 182, 89, 171, 169, 201, 34, 109, 27, 55, 3, 1, 189, 228, 13,
        12, 118, 37, 142, 32, 96, 72, 52, 26, 96, 230, 207, 119, 113, 233, 93, 187, 148, 249, 67,
        207, 103, 175, 95, 148, 229, 42, 208, 56, 110, 226, 14, 7, 221, 197, 186, 87, 173, 170,
        246, 223, 116, 147, 207, 80, 20, 144, 233, 233, 81, 115, 177, 216, 168, 86, 200, 231, 82,
        91, 255, 60, 37, 31, 63, 78, 163, 165, 204, 82, 81, 17, 69, 57, 46, 196, 202, 154, 26, 237,
        47, 72, 137, 203, 58, 42, 84, 83, 3, 9, 37, 145, 181, 41, 219, 232, 244, 117, 70, 191, 59,
        247, 187, 205, 181, 108, 173, 111, 66, 140, 57, 7, 181, 15, 199, 178, 78, 86, 118, 56, 112,
        8, 68, 193, 211, 206, 50, 39, 117, 106, 230, 103, 232, 126, 71, 70, 28, 176, 119, 175, 184,
        82, 81, 82, 155, 220, 110, 7, 182, 41, 150, 19, 10, 217, 156, 162, 216, 142, 199, 131, 57,
        7, 14, 184, 166, 222, 124, 51, 158, 231, 216, 48, 254, 45, 250, 5, 208, 97, 28, 180, 189,
        119, 220, 81, 169, 140, 140, 168, 169, 67, 135, 80, 28, 118, 149, 40, 92, 119, 48, 242,
        129, 146, 136, 235, 83, 3, 125, 100, 198, 40, 80, 168, 155, 38, 93, 40, 153, 246, 113, 145,
        162, 37, 74, 150, 36, 203, 22, 86, 93, 221, 66, 187, 92, 6, 217, 223, 63, 86, 116, 185,
        220, 255, 61, 123, 56, 111, 93, 122, 105, 123, 66, 77, 100, 100, 216, 5, 57, 186, 19, 244,
        177, 190, 196, 152, 52, 38, 103, 204, 140, 2, 219, 149, 105, 224, 1, 111, 130, 106, 240,
        71, 176, 4, 100, 203, 83, 158, 225, 140, 109, 252, 209, 163, 6, 218, 223, 47, 173, 74, 36,
        236, 239, 195, 70, 176, 209, 229, 178, 105, 69, 193, 149, 96, 80, 128, 85, 159, 115, 158,
        121, 166, 247, 100, 211, 228, 54, 254, 22, 240, 91, 87, 77, 125, 125, 1, 46, 110, 67, 76,
        165, 100, 181, 80, 80, 249, 89, 237, 97, 250, 158, 187, 90, 167, 38, 134, 241, 212, 182,
        157, 90, 5, 23, 84, 113, 158, 23, 148, 209, 209, 156, 157, 207, 91, 208, 252, 76, 57, 178,
        44, 203, 193, 88, 214, 75, 93, 119, 93, 21, 182, 99, 199, 176, 232, 247, 71, 199, 228, 254,
        201, 231, 175, 39, 8, 169, 88, 116, 137, 166, 104, 176, 4, 103, 87, 195, 14, 139, 211, 57,
        124, 212, 28, 125, 239, 29, 250, 157, 159, 128, 26, 176, 27, 92, 6, 123, 229, 79, 112, 70,
        33, 37, 38, 39, 29, 112, 242, 164, 94, 151, 203, 233, 107, 17, 196, 252, 146, 162, 80, 17,
        69, 49, 208, 92, 174, 128, 236, 222, 61, 17, 91, 51, 182, 209, 95, 137, 38, 81, 234, 11,
        95, 152, 175, 239, 216, 17, 71, 114, 57, 219, 211, 209, 206, 140, 30, 222, 231, 35, 121,
        207, 9, 174, 162, 198, 9, 119, 206, 99, 1, 172, 166, 217, 147, 39, 149, 210, 26, 46, 41,
        112, 16, 4, 129, 217, 2, 232, 208, 213, 85, 183, 222, 90, 23, 186, 238, 186, 80, 241, 216,
        49, 49, 253, 200, 35, 99, 150, 105, 88, 155, 190, 138, 40, 158, 96, 107, 235, 81, 249, 104,
        82, 176, 133, 124, 3, 219, 160, 232, 168, 254, 186, 43, 236, 250, 195, 239, 189, 191, 31,
        1, 29, 51, 117, 236, 83, 252, 85, 33, 127, 73, 44, 166, 226, 197, 34, 230, 73, 165, 116,
        118, 207, 30, 65, 175, 30, 216, 106, 71, 14, 63, 245, 71, 28, 238, 184, 136, 186, 186, 32,
        16, 69, 197, 128, 79, 152, 20, 197, 230, 220, 225, 195, 73, 34, 18, 25, 131, 129, 238, 227,
        124, 62, 66, 153, 156, 212, 74, 117, 116, 166, 29, 128, 197, 40, 242, 208, 67, 237, 54,
        203, 18, 177, 7, 30, 56, 214, 118, 251, 237, 85, 145, 47, 127, 57, 156, 221, 177, 35, 59,
        254, 192, 3, 195, 165, 49, 133, 182, 138, 223, 30, 184, 115, 241, 208, 239, 10, 191, 83,
        116, 68, 47, 176, 62, 118, 124, 159, 119, 95, 10, 180, 130, 191, 185, 221, 61, 43, 33, 159,
        164, 103, 205, 26, 119, 58, 22, 123, 29, 166, 104, 152, 45, 49, 184, 113, 68, 17, 19, 69,
        81, 21, 69, 73, 251, 208, 161, 2, 225, 243, 25, 92, 107, 171, 7, 135, 207, 31, 175, 169,
        161, 201, 207, 127, 62, 168, 188, 253, 118, 138, 241, 249, 176, 89, 63, 249, 73, 187, 9,
        171, 234, 209, 251, 238, 59, 225, 140, 143, 155, 149, 87, 92, 225, 206, 111, 219, 150, 179,
        224, 22, 179, 228, 50, 184, 228, 222, 186, 244, 131, 15, 126, 80, 190, 213, 89, 115, 198,
        54, 226, 239, 225, 11, 6, 97, 173, 193, 101, 56, 121, 80, 18, 81, 122, 97, 60, 143, 243,
        107, 214, 132, 176, 182, 54, 212, 82, 20, 19, 198, 139, 94, 138, 133, 218, 187, 239, 174,
        175, 95, 189, 58, 20, 90, 183, 174, 82, 30, 24, 80, 196, 158, 158, 188, 184, 123, 119, 218,
        56, 118, 76, 54, 114, 57, 45, 243, 230, 155, 226, 71, 34, 74, 227, 161, 223, 230, 140, 215,
        213, 157, 243, 3, 62, 47, 33, 117, 79, 63, 109, 194, 62, 127, 212, 129, 2, 240, 138, 10,
        30, 161, 105, 162, 116, 103, 189, 189, 189, 2, 95, 176, 32, 232, 192, 138, 138, 219, 54,
        128, 159, 121, 89, 184, 39, 246, 120, 60, 0, 73, 38, 101, 27, 246, 247, 189, 223, 251, 222,
        208, 192, 15, 127, 56, 10, 163, 189, 180, 220, 28, 19, 195, 78, 149, 0, 40, 194, 134, 127,
        67, 59, 225, 180, 215, 27, 158, 57, 119, 14, 156, 151, 144, 18, 168, 207, 215, 79, 194,
        229, 4, 120, 158, 67, 171, 171, 253, 40, 12, 106, 244, 248, 241, 52, 26, 10, 209, 48, 173,
        178, 37, 97, 38, 12, 232, 195, 207, 63, 15, 246, 111, 220, 40, 76, 190, 240, 66, 178, 52,
        241, 211, 79, 190, 140, 1, 253, 169, 116, 118, 182, 216, 110, 55, 13, 133, 0, 139, 166, 41,
        27, 65, 218, 78, 221, 229, 236, 57, 111, 33, 222, 197, 139, 79, 98, 46, 23, 101, 166, 211,
        130, 204, 48, 52, 213, 210, 226, 194, 182, 109, 75, 128, 222, 222, 60, 30, 137, 48, 182,
        105, 218, 102, 42, 149, 23, 223, 123, 239, 112, 126, 235, 214, 126, 56, 115, 184, 155, 181,
        49, 3, 216, 164, 1, 44, 194, 116, 108, 24, 66, 14, 162, 100, 179, 2, 74, 146, 46, 35, 16,
        240, 193, 212, 129, 27, 11, 22, 116, 56, 24, 86, 87, 190, 205, 89, 115, 222, 66, 248, 134,
        134, 221, 184, 203, 133, 160, 176, 29, 197, 85, 181, 32, 93, 123, 109, 19, 198, 113, 56,
        76, 173, 8, 60, 79, 58, 112, 99, 3, 39, 78, 26, 154, 200, 42, 184, 17, 77, 161, 74, 115,
        18, 85, 218, 83, 152, 210, 62, 137, 200, 29, 73, 120, 204, 32, 106, 141, 5, 35, 95, 155,
        152, 72, 83, 215, 92, 227, 37, 130, 65, 26, 182, 214, 184, 229, 241, 4, 202, 183, 57, 107,
        206, 91, 72, 221, 186, 117, 211, 182, 36, 237, 135, 171, 11, 33, 211, 105, 25, 8, 130, 233,
        192, 174, 18, 134, 62, 1, 60, 30, 94, 175, 143, 70, 5, 218, 110, 53, 57, 164, 217, 32, 156,
        8, 32, 17, 15, 20, 198, 184, 56, 63, 25, 12, 68, 73, 148, 162, 25, 7, 163, 176, 36, 198,
        215, 215, 125, 251, 219, 100, 195, 186, 117, 46, 242, 43, 95, 153, 5, 14, 28, 0, 48, 89,
        204, 52, 130, 231, 194, 121, 11, 41, 129, 209, 244, 78, 88, 27, 48, 204, 237, 198, 168, 87,
        94, 25, 66, 39, 39, 101, 130, 32, 200, 172, 92, 240, 209, 193, 48, 237, 225, 195, 148, 3,
        23, 60, 77, 226, 128, 192, 79, 197, 52, 67, 177, 0, 22, 76, 48, 255, 63, 159, 2, 237, 143,
        63, 25, 114, 224, 78, 217, 3, 27, 203, 82, 66, 112, 98, 49, 219, 171, 105, 195, 112, 82,
        123, 103, 6, 159, 3, 23, 36, 132, 175, 173, 125, 155, 173, 175, 247, 25, 55, 222, 216, 140,
        193, 218, 128, 195, 45, 57, 92, 239, 56, 229, 128, 124, 78, 17, 92, 150, 172, 20, 77, 203,
        4, 162, 172, 2, 203, 23, 6, 145, 53, 55, 129, 180, 37, 1, 39, 18, 114, 234, 102, 207, 6,
        245, 157, 179, 144, 182, 16, 109, 143, 61, 252, 48, 114, 244, 206, 59, 13, 233, 229, 151,
        199, 12, 138, 59, 238, 13, 133, 142, 150, 111, 113, 214, 124, 172, 251, 61, 87, 54, 239,
        221, 91, 188, 167, 165, 165, 70, 108, 107, 91, 196, 91, 150, 11, 238, 113, 125, 208, 35,
        130, 110, 131, 162, 141, 57, 180, 56, 62, 84, 176, 128, 201, 192, 64, 183, 23, 253, 251,
        79, 209, 186, 207, 95, 131, 96, 48, 206, 193, 142, 189, 2, 223, 217, 129, 107, 71, 142, 56,
        202, 190, 189, 114, 118, 100, 132, 177, 68, 145, 160, 28, 116, 178, 224, 10, 220, 183, 240,
        245, 151, 39, 202, 183, 56, 107, 46, 200, 35, 37, 40, 175, 119, 179, 127, 251, 246, 65,
        216, 166, 136, 136, 105, 230, 73, 216, 182, 48, 56, 206, 185, 1, 149, 38, 109, 34, 69, 0,
        68, 37, 112, 151, 201, 51, 46, 196, 235, 245, 130, 72, 62, 27, 175, 116, 180, 216, 200, 93,
        119, 29, 46, 110, 222, 124, 130, 211, 245, 73, 158, 162, 122, 85, 69, 235, 21, 60, 193,
        187, 174, 217, 246, 242, 254, 178, 233, 115, 226, 188, 90, 148, 79, 210, 251, 207, 255,
        116, 159, 62, 81, 184, 19, 70, 11, 234, 80, 20, 111, 134, 195, 60, 206, 48, 104, 236, 181,
        215, 142, 42, 154, 233, 241, 132, 106, 27, 96, 102, 85, 26, 106, 3, 147, 198, 192, 128, 12,
        247, 202, 32, 151, 203, 169, 176, 88, 18, 104, 117, 0, 55, 42, 232, 151, 212, 193, 226, 99,
        221, 91, 182, 168, 101, 147, 231, 204, 5, 123, 164, 68, 98, 137, 217, 151, 239, 50, 77,
        186, 38, 226, 118, 7, 131, 22, 157, 205, 198, 64, 42, 53, 138, 226, 184, 26, 207, 142, 169,
        177, 236, 168, 234, 214, 11, 83, 114, 54, 75, 88, 176, 63, 43, 125, 93, 79, 44, 158, 231,
        29, 92, 133, 251, 226, 183, 226, 152, 121, 179, 123, 248, 66, 68, 148, 184, 160, 24, 249,
        136, 213, 171, 167, 252, 73, 100, 250, 50, 33, 156, 54, 29, 148, 32, 152, 34, 106, 2, 10,
        134, 12, 124, 80, 84, 182, 152, 213, 13, 89, 142, 155, 34, 161, 133, 48, 163, 48, 143, 245,
        102, 190, 232, 15, 89, 221, 115, 48, 166, 186, 129, 19, 197, 119, 51, 28, 71, 29, 124, 250,
        233, 222, 67, 101, 115, 231, 197, 69, 241, 136, 203, 53, 87, 164, 233, 38, 212, 100, 220,
        214, 160, 255, 68, 234, 253, 217, 251, 133, 253, 149, 239, 201, 241, 89, 19, 118, 246, 203,
        8, 170, 174, 197, 11, 206, 215, 235, 133, 220, 210, 8, 22, 139, 136, 182, 78, 34, 100, 42,
        245, 190, 20, 10, 205, 65, 116, 29, 135, 29, 175, 81, 218, 112, 93, 16, 23, 69, 72, 101,
        229, 170, 66, 125, 253, 98, 148, 97, 106, 57, 199, 169, 114, 83, 244, 124, 70, 161, 128,
        144, 119, 103, 5, 169, 210, 180, 141, 0, 131, 54, 182, 222, 84, 215, 214, 182, 161, 154,
        97, 170, 243, 83, 83, 253, 125, 178, 60, 97, 37, 147, 251, 164, 202, 202, 175, 85, 200,
        114, 162, 244, 61, 210, 5, 113, 81, 132, 68, 34, 221, 73, 191, 191, 182, 199, 182, 9, 119,
        125, 253, 218, 234, 182, 182, 127, 108, 174, 170, 186, 50, 72, 146, 36, 12, 19, 6, 71, 81,
        2, 27, 30, 126, 110, 228, 228, 201, 95, 14, 10, 194, 158, 130, 101, 77, 72, 154, 150, 22,
        51, 153, 253, 162, 219, 29, 133, 159, 115, 159, 250, 199, 205, 185, 114, 81, 132, 4, 131,
        221, 48, 126, 145, 223, 217, 118, 226, 164, 40, 190, 145, 79, 38, 255, 71, 50, 140, 62, 53,
        16, 168, 12, 180, 180, 220, 218, 54, 111, 222, 191, 206, 225, 249, 74, 70, 146, 222, 205,
        97, 88, 222, 36, 8, 3, 54, 185, 53, 180, 170, 90, 158, 193, 193, 173, 25, 28, 15, 164, 203,
        166, 206, 155, 139, 34, 164, 4, 236, 243, 246, 5, 2, 85, 239, 40, 202, 184, 38, 73, 199,
        28, 73, 74, 85, 21, 139, 66, 64, 215, 227, 176, 37, 59, 146, 55, 140, 140, 205, 243, 159,
        139, 70, 34, 183, 55, 55, 52, 60, 216, 78, 211, 181, 20, 220, 146, 104, 150, 149, 217, 227,
        241, 116, 15, 150, 205, 156, 55, 23, 77, 72, 36, 178, 82, 15, 133, 22, 252, 134, 101, 185,
        247, 29, 39, 51, 230, 118, 47, 176, 162, 209, 175, 211, 154, 150, 74, 143, 142, 62, 57, 40,
        203, 163, 170, 101, 105, 6, 77, 243, 48, 57, 84, 243, 44, 91, 231, 166, 40, 125, 75, 32,
        176, 251, 135, 179, 103, 127, 255, 130, 99, 228, 162, 20, 196, 143, 16, 197, 30, 48, 52,
        244, 84, 71, 60, 222, 119, 27, 65, 44, 90, 226, 114, 117, 213, 38, 147, 59, 50, 211, 211,
        175, 36, 16, 196, 134, 37, 4, 40, 240, 149, 231, 184, 250, 233, 64, 160, 117, 31, 207, 43,
        111, 116, 119, 255, 254, 99, 223, 79, 157, 47, 23, 85, 200, 71, 12, 15, 255, 136, 207, 100,
        62, 104, 86, 20, 171, 73, 81, 116, 151, 97, 12, 11, 4, 225, 100, 9, 194, 158, 166, 105, 39,
        227, 118, 219, 185, 57, 115, 98, 23, 84, 0, 63, 227, 51, 62, 227, 92, 0, 224, 127, 1, 208,
        202, 28, 31, 66, 176, 235, 16, 0, 0, 0, 3, 82, 117, 83, 116, 104, 101, 121, 158, 176, 245,
        160, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
    ];
}
