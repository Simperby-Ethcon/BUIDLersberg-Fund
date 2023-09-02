mod lightclient;
mod relayer;

use serde_json::Value;
use relayer::Relayer;
use simperby_core::*;
use lightclient::BlockHeader;

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

        // Assuming you want to proceed with using the author in a BlockHeader and initializing a light client:
        // let header = BlockHeader {
        //     author,  // and other necessary fields...
        // };

        // println!("{:?}", header);

        // Assuming you have valid ChainType and EvmCompatibleAddress values:
        // let chain = ChainType::YourVariantHere;
        // let address = Some(EvmCompatibleAddress {});
        // relayer.initialize_light_client(header, chain, address)?;

        // relayer.run().await?;
    }

    Ok(())
}
