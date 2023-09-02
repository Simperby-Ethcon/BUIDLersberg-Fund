use reqwest::header;
use serde_json::Value;
use std::time::Duration;
use simperby_core::{BlockHeader, BlockHeight, Transaction};
use simperby_core::merkle_tree::MerkleProof;
use tokio::time::sleep;
use simperby_evm_client::{ChainType, EvmCompatibleAddress};
use crate::lightclient::*;

// The relayer will manage a list of light clients.
pub struct Relayer {
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

    pub async fn poll_github(&self, only_latest: bool) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let res = self.client
            .get(&self.url)
            .header(header::AUTHORIZATION, format!("token {}", &self.token))
            .header(header::USER_AGENT, "my-app")
            .send()
            .await?;

        let commits: Vec<Value> = res.json().await?;
        if only_latest {
            Ok(commits.into_iter().take(1).collect())
        } else {
            Ok(commits)
        }
    }

    pub async fn handle_commit_body(&self, commit: &Value) -> Result<Option<Value>, Box<dyn std::error::Error>> {
        if let Some(message) = commit["commit"]["message"].as_str() {
            if message.starts_with(">block:") {
                let content = &message[">block:".len()..].trim_start_matches('\n');

                // Split the content into two parts: number and JSON
                let mut parts = content.splitn(2, '\n');
                if let (Some(number_str), Some(json_str)) = (parts.next(), parts.next()) {
                    // Parse the number
                    if let Ok(number) = number_str.trim().parse::<i32>() {
                        // Parse the JSON content
                        if let Ok(parsed_message) = serde_json::from_str::<Value>(json_str) {
                            return Ok(Some(parsed_message));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    pub fn initialize_light_client(&mut self, header: BlockHeader, chain: ChainType, address: Option<EvmCompatibleAddress>) -> Result<(), String> {
        let client = MythereumTreasuryContract::new(header, chain, address)?;
        self.light_clients.push(client);
        Ok(())
    }

    pub async fn execute(&mut self, transaction: Transaction, height: BlockHeight, proof: MerkleProof) -> Result<(), String> {
        // For now, executing on the first client. Depending on your logic, you might want to iterate over all clients or choose a specific one.
        if let Some(client) = self.light_clients.first_mut() {
            client.execute(transaction, height, proof).await
        } else {
            Err("No client initialized.".to_string())
        }
    }

    pub async fn run(&mut self) {
        loop {
            match self.poll_github(true).await {
                Ok(commits) => {
                    for commit in commits.iter() {
                        println!("{}", commit);
                        match self.handle_commit_body(&commit).await {
                            Ok(Some(files_changed)) => {
                                println!("Files changed in latest commit: {:#?}", files_changed);

                                // Add your logic here to determine when to call execute.
                                // If some condition, then:
                                // let transaction = Transaction {...};
                                // let height = BlockHeight {...};
                                // let proof = MerkleProof {...};
                                // let _ = self.execute(transaction, height, proof);
                            },
                            Err(err) => {
                                println!("Error while handling commit: {}", err);
                            }
                            _ => {} // e.g. Ok(None) => no block found in commit
                        }
                    }
                },
                Err(err) => {
                    println!("Error while polling Github: {}", err);
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
}
