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
}
