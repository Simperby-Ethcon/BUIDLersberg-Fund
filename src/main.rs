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

    // Initialize a light client using actual values or replace with your custom values.
    // let header = BlockHeader {
    //     author: (),
    //     prev_block_finalization_proof: FinalizationProof {},
    //     previous_hash: Hash256 {},
    //     height: 0,
    //     timestamp: 0,
    //     commit_merkle_root: Hash256 {},
    //     repository_merkle_root: Hash256 {},
    //     validator_set: vec![],
    //     version: "".to_string(),
    // };
    //
    // let chain = ChainType::YourVariantHere; // Replace with your actual enum variant
    // let address = Some(EvmCompatibleAddress {}); // Use actual address or your custom value
    //
    // relayer.initialize_light_client(header, chain, address)?;
    //
    // relayer.run().await;

    Ok(())
}
