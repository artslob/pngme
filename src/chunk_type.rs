#[derive(std::fmt::Debug)]
struct ChunkType {
    ancillary_bit_char: char,
    private_bit_char: char,
    reserved_bit_char: char,
    safe_to_copy_bit_char: char,
    bytes: [u8; 4],
    chars: [char; 4],
    string: String,
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
    fn is_critical(&self) -> bool {
        self.ancillary_bit_char.is_ascii_uppercase()
    }
    fn is_public(&self) -> bool {
        self.private_bit_char.is_ascii_uppercase()
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.reserved_bit_char.is_ascii_uppercase()
    }
    fn is_safe_to_copy(&self) -> bool {
        self.safe_to_copy_bit_char.is_ascii_lowercase()
    }
}

// TODO checks for valid chars

impl std::convert::TryFrom<[u8; 4]> for ChunkType {
    type Error = String;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        let ancillary_bit_char = char::from(bytes[0]);
        let private_bit_char = char::from(bytes[1]);
        let reserved_bit_char = char::from(bytes[2]);
        let safe_to_copy_bit_char = char::from(bytes[3]);
        let chars = [
            ancillary_bit_char,
            private_bit_char,
            reserved_bit_char,
            safe_to_copy_bit_char,
        ];
        Ok(ChunkType {
            ancillary_bit_char,
            private_bit_char,
            reserved_bit_char,
            safe_to_copy_bit_char,
            bytes,
            chars,
            string: chars.iter().collect(),
        })
    }
}

impl std::str::FromStr for ChunkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<char>>();
        let chars = match chars[..] {
            [a, b, c, d] => [a, b, c, d],
            _ => return Err("string should contain 4 chars".into()),
        };
        Ok(ChunkType {
            ancillary_bit_char: chars[0],
            private_bit_char: chars[1],
            reserved_bit_char: chars[2],
            safe_to_copy_bit_char: chars[3],
            bytes: [
                u32::from(chars[0]) as u8,
                u32::from(chars[1]) as u8,
                u32::from(chars[2]) as u8,
                u32::from(chars[3]) as u8,
            ],
            chars,
            string: s.into(),
        })
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl std::cmp::PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl std::cmp::Eq for ChunkType {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        // TODO fix test
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
