use simperby_core::HexSerializedVec;

pub fn string_to_hex(s: &str) -> HexSerializedVec {
    HexSerializedVec::from(s.as_bytes().to_vec())
}
