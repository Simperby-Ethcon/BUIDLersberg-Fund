mod lightclient;
mod relayer;

use serde_json::Value;
use relayer::Relayer;
use simperby_core::*;
use lightclient::BlockHeader;
use simperby_evm_client::ChainType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "";
    let url = "https://api.github.com/repos/Simperby-Ethcon/dev-chain/commits";

    let mut relayer = Relayer::new(token, url);

    // Get initial data from GitHub
    let commit = relayer.poll_github().await?;

    println!("{:#?}", commit);

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

    if let Some(commit_data) = commit {
        if let Value::String(author_hex) = &commit_data["author"] {
            let author_bytes = hex::decode(author_hex).expect("Failed to decode hex string");

            author = match author_bytes.len() {
                33 => {
                    let mut array = [0; 33];
                    array.copy_from_slice(&author_bytes);
                    PublicKey::from_array(array).expect("Failed to create PublicKey from bytes")
                },
                65 => {
                    let mut array = [0; 65];
                    array.copy_from_slice(&author_bytes);
                    PublicKey::from_array_uncompressed(array).expect("Failed to create uncompressed PublicKey from bytes")
                },
                _ => {
                    panic!("Unexpected length for PublicKey bytes");
                }
            };

            println!("{:?}", author);
        }

        if let Some(Value::Object(prev_block_finalization_proof)) = commit_data.get("prev_block_finalization_proof") {
            finalization_proof = serde_json::from_value(Value::Object(prev_block_finalization_proof.clone())).expect("Failed to parse finalization proof");
    
            println!("{:?}", finalization_proof);
        }
        
        if let Some(Value::String(previous_hash_hex)) = commit_data.get("previous_hash") {
            let previous_hash_bytes = hex::decode(previous_hash_hex).expect("Failed to decode hex string");
            if previous_hash_bytes.len() == 32 {
                let mut array = [0; 32];
                array.copy_from_slice(&previous_hash_bytes);
                previous_hash = Some(Hash256 { hash: HexSerializedBytes { data: array } });
                println!("{:?}", previous_hash);
            } else {
                panic!("Unexpected length for Hash256 bytes");
            }
        }

        if let Some(Value::Number(height)) = commit_data.get("height") {
            block_height = height.as_u64().expect("Failed to convert to u64");
            println!("{:?}", block_height);
        } else {
            panic!("Height is not a number");
        }
        
        if let Some(Value::Number(ts)) = commit_data.get("timestamp") {
            timestamp = ts.as_i64().expect("Failed to convert to i64");
            println!("{:?}", timestamp);
        } else {
            panic!("Timestamp is not a number");
        }

        if let Some(Value::String(commit_merkle_root_hex)) = commit_data.get("commit_merkle_root") {
            let commit_merkle_root_bytes = hex::decode(commit_merkle_root_hex).expect("Failed to decode hex string");
    
            if commit_merkle_root_bytes.len() == 32 {
                let mut array = [0; 32];
                array.copy_from_slice(&commit_merkle_root_bytes);
                commit_merkle_root = Some(Hash256 { hash: HexSerializedBytes { data: array } });

                println!("{:?}", commit_merkle_root);
            } else {
                panic!("Unexpected length for Hash256 bytes");
            }
        } else {
            panic!("commit_merkle_root is not a string");
        }

        if let Some(Value::String(repository_merkle_root_hex)) = commit_data.get("repository_merkle_root") {
            let repository_merkle_root_bytes = hex::decode(repository_merkle_root_hex).expect("Failed to decode hex string");
    
            if repository_merkle_root_bytes.len() == 32 {
                let mut array = [0; 32];
                array.copy_from_slice(&repository_merkle_root_bytes);
                repository_merkle_root = Some(Hash256 { hash: HexSerializedBytes { data: array } });
                println!("{:?}", repository_merkle_root);
            } else {
                panic!("Unexpected length for Hash256 bytes");
            }
        } else {
            panic!("repository_merkle_root is not a string");
        }

        if let Value::Array(validator_set_array) = &commit_data["validator_set"] {
        
            for item in validator_set_array.iter() {
                if let Value::Array(item_array) = item {
                    if item_array.len() != 2 {
                        panic!("Unexpected length for validator_set item");
                    }
        
                    let public_key_hex = item_array[0].as_str().expect("Failed to convert to string");
                    let public_key_bytes = hex::decode(public_key_hex).expect("Failed to decode hex string");
        
                    let public_key = match public_key_bytes.len() {
                        33 => {
                            let mut array = [0; 33];
                            array.copy_from_slice(&public_key_bytes);
                            PublicKey::from_array(array).expect("Failed to create PublicKey from bytes")
                        },
                        65 => {
                            let mut array = [0; 65];
                            array.copy_from_slice(&public_key_bytes);
                            PublicKey::from_array_uncompressed(array).expect("Failed to create uncompressed PublicKey from bytes")
                        },
                        _ => {
                            panic!("Unexpected length for PublicKey bytes");
                        }
                    };
        
                    let voting_power: VotingPower = item_array[1].as_u64().expect("Failed to convert to u64");
        
                    validator_set.push((public_key, voting_power));
                } else {
                    panic!("validator_set item is not an array");
                }
            }
        
            println!("{:?}", validator_set);
        } else {
            panic!("validator_set is not an array");
        }
        
        if let Value::String(ref version_str) = commit_data["version"] {
            version = version_str.clone();
            println!("{:?}", version);
        } else {
            panic!("version is not a string");
        }

        // println!("author: {:?}", author);
        // println!("prev_block_finalization_proof: {:?}", finalization_proof.expect("finalization_proof is None"));
        // println!("previous_hash: {:?}", previous_hash.expect("previous_hash is None"));
        // println!("height: {:?}", block_height);
        // println!("timestamp: {:?}", timestamp);
        // println!("commit_merkle_root: {:?}", commit_merkle_root.expect("commit_merkle_root is None"));
        // println!("repository_merkle_root: {:?}", repository_merkle_root.expect("repository_merkle_root is None"));
        // println!("validator_set: {:?}", validator_set);
        // println!("version: {:?}", version);


        let header = BlockHeader {
            author,
            prev_block_finalization_proof: finalization_proof.expect("finalization_proof is None"),
            previous_hash: previous_hash.expect("previous_hash is None"),
            height: block_height,
            timestamp,
            commit_merkle_root: commit_merkle_root.expect("commit_merkle_root is None"),
            repository_merkle_root: repository_merkle_root.expect("repository_merkle_root is None"),
            validator_set,
            version,
        };
        
        println!("{:?}", header);
        
        


        // Assuming you have valid ChainType and EvmCompatibleAddress values:
        // let chain = ChainType::Goerli(());
        // let address = Some(EvmCompatibleAddress {});
        // relayer.initialize_light_client(header, chain, address)?;

        // relayer.run().await?;
    }

    Ok(())
}
