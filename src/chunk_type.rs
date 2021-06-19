use std::convert::{TryFrom, TryInto};

enum AncillaryBit {
    Critical,  // UPPERCASE
    Ancillary, // lowercase
}

enum PrivateBit {
    Public,  // UPPERCASE
    Private, // lowercase
}

enum ReservedBit {
    Reserved, // UPPERCASE
}

enum SafeToCopyBit {
    Unsafe, // UPPERCASE
    Safe,   // lowercase
}

struct ChunkType {
    ancillary_bit: AncillaryBit,
    private_bit: PrivateBit,
    reserved_bit: ReservedBit,
    safe_to_copy_bit: SafeToCopyBit,
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
        self.chars[0].is_ascii_uppercase()
    }
    fn is_public(&self) -> bool {
        self.chars[1].is_ascii_uppercase()
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.chars[2].is_ascii_uppercase()
    }
    fn is_safe_to_copy(&self) -> bool {
        self.chars[3].is_ascii_lowercase()
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
            ancillary_bit: AncillaryBit::Critical,
            private_bit: PrivateBit::Public,
            reserved_bit: ReservedBit::Reserved,
            safe_to_copy_bit: SafeToCopyBit::Unsafe,
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
            ancillary_bit: AncillaryBit::Critical,
            private_bit: PrivateBit::Public,
            reserved_bit: ReservedBit::Reserved,
            safe_to_copy_bit: SafeToCopyBit::Unsafe,
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
}
