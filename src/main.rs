mod lightclient;
mod relayer;

use serde_json::Value;
use relayer::Relayer;
use simperby_core::*;
use lightclient::BlockHeader;
use simperby_evm_client::ChainType;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BlockHeaderError {
    DecodeError(hex::FromHexError),
    JsonError(serde_json::Error),
    UnexpectedPublicKeyLength,
    UnexpectedHash256Length,
    UnexpectedValidatorSetItemLength,
    ValueMismatch(&'static str),
}

impl From<hex::FromHexError> for BlockHeaderError {
    fn from(err: hex::FromHexError) -> Self {
        BlockHeaderError::DecodeError(err)
    }
}

impl From<serde_json::Error> for BlockHeaderError {
    fn from(err: serde_json::Error) -> Self {
        BlockHeaderError::JsonError(err)
    }
}

impl Error for BlockHeaderError {}

impl fmt::Display for BlockHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlockHeaderError::DecodeError(ref e) => write!(f, "DecodeError: {}", e),
            BlockHeaderError::JsonError(ref e) => write!(f, "JsonError: {}", e),
            BlockHeaderError::UnexpectedPublicKeyLength => write!(f, "Unexpected length for PublicKey bytes"),
            BlockHeaderError::UnexpectedHash256Length => write!(f, "Unexpected length for Hash256 bytes"),
            BlockHeaderError::UnexpectedValidatorSetItemLength => write!(f, "Unexpected length for validator_set item"),
            BlockHeaderError::ValueMismatch(ref s) => write!(f, "ValueMismatch: {}", s),
        }
    }
}


fn extract_block_header(commit_data: Value) -> Result<BlockHeader, BlockHeaderError> {
    // Initialize author with a default value
    let mut author = PublicKey::zero();
    let mut finalization_proof: Option<FinalizationProof> = None;
    let mut previous_hash: Option<Hash256> = None;
    let mut block_height: BlockHeight = 0; // Assuming BlockHeight is u64 or similar
    let mut timestamp: Timestamp = 0; // Assuming Timestamp is i64 or similar
    let mut commit_merkle_root: Option<Hash256> = None;
    let mut repository_merkle_root: Option<Hash256> = None;
    let mut validator_set: Vec<(PublicKey, VotingPower)> = Vec::new();
    let mut version: String = String::new();

    if let Value::String(author_hex) = &commit_data["author"] {
        let author_bytes = hex::decode(author_hex)?;

        author = match author_bytes.len() {
            33 => {
                let mut array = [0; 33];
                array.copy_from_slice(&author_bytes);
                PublicKey::from_array(array).map_err(|_| BlockHeaderError::UnexpectedPublicKeyLength)?
            },
            65 => {
                let mut array = [0; 65];
                array.copy_from_slice(&author_bytes);
                PublicKey::from_array_uncompressed(array).map_err(|_| BlockHeaderError::UnexpectedPublicKeyLength)?
            },
            _ => return Err(BlockHeaderError::UnexpectedPublicKeyLength),
        };
    }

    if let Some(Value::Object(prev_block_finalization_proof)) = commit_data.get("prev_block_finalization_proof") {
        finalization_proof = Some(serde_json::from_value(Value::Object(prev_block_finalization_proof.clone()))?);
    }

    if let Some(Value::String(previous_hash_hex)) = commit_data.get("previous_hash") {
        let previous_hash_bytes = hex::decode(previous_hash_hex)?;

        if previous_hash_bytes.len() == 32 {
            let mut array = [0; 32];
            array.copy_from_slice(&previous_hash_bytes);
            previous_hash = Some(Hash256 { hash: HexSerializedBytes { data: array } });
        } else {
            return Err(BlockHeaderError::UnexpectedHash256Length);
        }
    }

    if let Some(Value::Number(height)) = commit_data.get("height") {
        block_height = height.as_u64().ok_or(BlockHeaderError::ValueMismatch("height"))?;
    } else {
        return Err(BlockHeaderError::ValueMismatch("height"));
    }

    if let Some(Value::Number(ts)) = commit_data.get("timestamp") {
        timestamp = ts.as_i64().ok_or(BlockHeaderError::ValueMismatch("timestamp"))?;
    } else {
        return Err(BlockHeaderError::ValueMismatch("timestamp"));
    }

    if let Some(Value::String(commit_merkle_root_hex)) = commit_data.get("commit_merkle_root") {
        let commit_merkle_root_bytes = hex::decode(commit_merkle_root_hex)?;

        if commit_merkle_root_bytes.len() == 32 {
            let mut array = [0; 32];
            array.copy_from_slice(&commit_merkle_root_bytes);
            commit_merkle_root = Some(Hash256 { hash: HexSerializedBytes { data: array } });
        } else {
            return Err(BlockHeaderError::UnexpectedHash256Length);
        }
    } else {
        return Err(BlockHeaderError::ValueMismatch("commit_merkle_root"));
    }

    if let Some(Value::String(repository_merkle_root_hex)) = commit_data.get("repository_merkle_root") {
        let repository_merkle_root_bytes = hex::decode(repository_merkle_root_hex)?;

        if repository_merkle_root_bytes.len() == 32 {
            let mut array = [0; 32];
            array.copy_from_slice(&repository_merkle_root_bytes);
            repository_merkle_root = Some(Hash256 { hash: HexSerializedBytes { data: array } });
        } else {
            return Err(BlockHeaderError::UnexpectedHash256Length);
        }
    } else {
        return Err(BlockHeaderError::ValueMismatch("repository_merkle_root"));
    }

    if let Value::Array(validator_set_array) = &commit_data["validator_set"] {

        for item in validator_set_array.iter() {
            if let Value::Array(item_array) = item {
                if item_array.len() != 2 {
                    return Err(BlockHeaderError::UnexpectedValidatorSetItemLength);
                }

                let public_key_hex = item_array[0].as_str().ok_or(BlockHeaderError::ValueMismatch("public_key"))?;
                let public_key_bytes = hex::decode(public_key_hex)?;

                let public_key = match public_key_bytes.len() {
                    33 => {
                        let mut array = [0; 33];
                        array.copy_from_slice(&public_key_bytes);
                        PublicKey::from_array(array).map_err(|_| BlockHeaderError::UnexpectedPublicKeyLength)?
                    },
                    65 => {
                        let mut array = [0; 65];
                        array.copy_from_slice(&public_key_bytes);
                        PublicKey::from_array_uncompressed(array).map_err(|_| BlockHeaderError::UnexpectedPublicKeyLength)?
                    },
                    _ => return Err(BlockHeaderError::UnexpectedPublicKeyLength),
                };

                let voting_power: VotingPower = item_array[1].as_u64().ok_or(BlockHeaderError::ValueMismatch("voting_power"))?;
                validator_set.push((public_key, voting_power));
            } else {
                return Err(BlockHeaderError::ValueMismatch("validator_set_item"));
            }
        }
    } else {
        return Err(BlockHeaderError::ValueMismatch("validator_set"));
    }

    if let Some(Value::String(ver)) = commit_data.get("version") {
        version = ver.clone();
    } else {
        return Err(BlockHeaderError::ValueMismatch("version"));
    }

    Ok(BlockHeader {
        author,
        prev_block_finalization_proof: finalization_proof.expect("finalization_proof is None"),
        previous_hash: previous_hash.expect("previous_hash is None"),
        height: block_height,
        timestamp,
        commit_merkle_root: commit_merkle_root.expect("commit_merkle_root is None"),
        repository_merkle_root: repository_merkle_root.expect("repository_merkle_root is None"),
        validator_set,
        version,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "";
    let url = "https://api.github.com/repos/Simperby-Ethcon/dev-chain/commits";

    let mut relayer = Relayer::new(token, url);

    // Get initial data from GitHub
    let commit = relayer.poll_github().await?;
    // println!("{:#?}", commit);

    if let Some(commit_data) = commit {
        let header = extract_block_header(commit_data)?;
        println!("{:?}", header);

        /*
        let chain = ChainType::YourChainVariantHere; // Replace with your actual enum variant
        let address = Some(EvmCompatibleAddress {}); // Use an actual address or your custom value

        relayer.initialize_light_client(header, chain, address)?;

        relayer.run().await?;
        */
    }

    Ok(())
}
