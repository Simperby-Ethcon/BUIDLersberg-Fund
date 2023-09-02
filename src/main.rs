// main.rs
mod lightclient;
mod util;
mod relayer;

use relayer::Relayer;
use simperby_core::*;
use lightclient::{BlockHeader, ChainType, EvmCompatibleAddress, FinalizationProof, Hash256};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "";
    let url = "https://api.github.com/repos/Simperby-Ethcon/dev-chain/commits";

    let mut relayer = Relayer::new(token, url);

    // Get initial data from GitHub
    let commit = relayer.poll_github().await?;

    if let Some(commit_data) = commit {
        println!("{:?}", commit_data.get("author"));
        let author = commit_data.get("author")
            .and_then(|v| v.as_str())
            .and_then(|s| hex::decode(s).ok())
            .and_then(|bytes| {
                match bytes.len() {
                    33 => {
                        let mut array = [0; 33];
                        array.copy_from_slice(&bytes);
                        PublicKey::from_array(array).ok()
                    },
                    65 => {
                        let mut array = [0; 65];
                        array.copy_from_slice(&bytes);
                        PublicKey::from_array_uncompressed(array).ok()
                    },
                    _ => None,
                }
            })
            .unwrap_or(PublicKey::zero());
        println!("{}", author);
    }

    // if let Some(commit_data) = commit {
    //     // Extract block header data from commit_data
    //     let author = commit_data.get("author")
    //         .and_then(|v| v.as_str())
    //         .and_then(|s| hex::decode(s).ok())
    //         .and_then(|bytes| {
    //             match bytes.len() {
    //                 33 => {
    //                     let mut array = [0; 33];
    //                     array.copy_from_slice(&bytes);
    //                     PublicKey::from_array(array).ok()
    //                 },
    //                 65 => {
    //                     let mut array = [0; 65];
    //                     array.copy_from_slice(&bytes);
    //                     PublicKey::from_array_uncompressed(array).ok()
    //                 },
    //                 _ => None,
    //             }
    //         })
    //         .unwrap_or(PublicKey::zero());
    //     let header = BlockHeader {
    //         author,
    //         prev_block_finalization_proof: FinalizationProof {
    //             round: commit_data.prev_block_finalization_proof.round,
    //             signatures: commit_data.prev_block_finalization_proof.signatures.clone(),
    //         },
    //         previous_hash: Hash256 { hash: commit_data.previous_hash.clone() },
    //         height: commit_data.height,
    //         timestamp: commit_data.timestamp,
    //         commit_merkle_root: Hash256 { hash: commit_data.commit_merkle_root.clone() },
    //         repository_merkle_root: Hash256 { hash: commit_data.repository_merkle_root.clone() },
    //         validator_set: commit_data.validator_set.clone(),
    //         version: commit_data.version.clone(),
    //     };
    //
    //     println!("{:?}", header);
    //
    //     // Assuming you have valid ChainType and EvmCompatibleAddress values, uncomment the below lines:
    //     // let chain = ChainType::YourVariantHere;
    //     // let address = Some(EvmCompatibleAddress {});
    //     // relayer.initialize_light_client(header, chain, address)?;
    //
    //     relayer.run().await;
    // }

    Ok(())
}
