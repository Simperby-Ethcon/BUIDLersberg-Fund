use reqwest::header;
use serde_json::Value;
use std::time::Duration;
use simperby_core::{BlockHeader, BlockHeight, Transaction};
use simperby_core::merkle_tree::MerkleProof;
use tokio::time::sleep;
use simperby_evm_client::{ChainType, EvmCompatibleAddress};
use crate::lightclient::*;

// The relayer will manage a list of light clients.
struct Relayer {
    light_clients: Vec<MythereumTreasuryContract>,
    client: reqwest::Client,
    token: String,
    url: String,
}

impl Relayer {
    pub fn new(token: &str, url: &str) -> Self {
        Self {
            light_clients: Vec::new(),
            client: reqwest::Client::new(),
            token: token.to_string(),
            url: url.to_string(),
        }
    }

    pub async fn poll_github(&self) -> Result<Option<Value>, Box<dyn std::error::Error>> {
        let res = self.client
            .get(&self.url)
            .header(header::AUTHORIZATION, format!("token {}", &self.token))
            .header(header::USER_AGENT, "my-app")
            .send()
            .await?;

        println!("{:#?}", res.status());

        let commits: Vec<Value> = res.json().await?;
        if let Some(commit) = commits.first() {
            return Ok(Some(commit.clone()));
        }
        Ok(None)
    }

    pub async fn handle_commit(&self, commit: &Value) {
        if let Some(sha) = commit.get("sha") {
            let commit_url = format!("https://api.github.com/repos/OWNER/REPO/git/commits/{}", sha.as_str().unwrap());
            let commit_res = self.client
                .get(&commit_url)
                .header(header::AUTHORIZATION, format!("token {}", &self.token))
                .header(header::USER_AGENT, "my-app")
                .send()
                .await;

            match commit_res {
                Ok(response) => {
                    println!("{:#?}", response.status());

                    if let Ok(commit_data) = response.json().await {
                        if let Some(files) = commit_data.get("files") {
                            println!("Files changed in latest commit: {:#?}", files);
                        }
                    }
                },
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        }
    }

    pub fn initialize_light_client(&mut self, header: BlockHeader, chain: ChainType, address: Option<EvmCompatibleAddress>) -> Result<(), String> {
        let client = MythereumTreasuryContract::new(header, chain, address)?;
        self.light_clients.push(client);
        Ok(())
    }

    pub fn execute(&mut self, transaction: Transaction, height: BlockHeight, proof: MerkleProof) -> Result<(), String> {
        // For now, executing on the first client. Depending on your logic, you might want to iterate over all clients or choose a specific one.
        if let Some(client) = self.light_clients.first_mut() {
            client.execute(transaction, height, proof)
        } else {
            Err("No client initialized.".to_string())
        }
    }

    pub async fn run(&mut self) {
        loop {
            match self.poll_github().await {
                Ok(Some(commit)) => {
                    self.handle_commit(&commit).await;
                    // Add your logic here to determine when to call execute.
                    // If some condition, then:
                    // let transaction = Transaction {...};
                    // let height = BlockHeight {...};
                    // let proof = MerkleProof {...};
                    // let _ = self.execute(transaction, height, proof);
                },
                Ok(None) => {},
                Err(err) => {
                    println!("Error while polling Github: {}", err);
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "";
    let url = "https://api.github.com/repos/Simperby-Ethcon/dev-chain/commits";

    let mut relayer = Relayer::new(token, url);

    // Initialize a light client as an example. You should replace placeholders with actual values.
    let header = BlockHeader {
        author: (),
        prev_block_finalization_proof: FinalizationProof {},
        previous_hash: Hash256 {},
        height: 0,
        timestamp: 0,
        commit_merkle_root: Hash256 {},
        repository_merkle_root: Hash256 {},
        validator_set: vec![],
        version: "".to_string(),
    }; // Placeholder
    let chain = ChainType::YourVariantHere; // Placeholder
    let address = Some(EvmCompatibleAddress {}); // Placeholder
    relayer.initialize_light_client(header, chain, address)?;

    relayer.run().await;

    Ok(())
}
