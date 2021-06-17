use std::convert::TryInto;

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
}

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
