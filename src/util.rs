use simperby_core::{HexSerializedVec, serde_spb};

pub fn string_to_hex(s: &str) -> HexSerializedVec {
    HexSerializedVec::from(s.as_bytes().to_vec())
}

pub async fn read_config<T: serde::de::DeserializeOwned>(path: &str) -> Option<T> {
    let content = tokio::fs::read_to_string(path).await.ok()?;
    serde_spb::from_str(&content).ok()
}