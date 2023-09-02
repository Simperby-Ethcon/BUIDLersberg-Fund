use reqwest::header;
use serde_json::Value;
use std::time::Duration;
use simperby_core::{BlockHeader, BlockHeight, Diff, Transaction};
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

pub enum CommitMessageType {
    Block(Value),
    Transaction(Value),
    Unknown,
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

    pub async fn handle_commit_body(&self, commit: &Value) -> Result<CommitMessageType, Box<dyn std::error::Error>> {
        if let Some(message) = commit["commit"]["message"].as_str() {
            if message.starts_with(">block:") {
                let content = &message[">block:".len()..].trim_start_matches('\n');
                let mut parts = content.splitn(2, '\n');
                if let (Some(_number_str), Some(json_str)) = (parts.next(), parts.next()) {
                    if let Ok(parsed_message) = serde_json::from_str::<Value>(json_str) {
                        return Ok(CommitMessageType::Block(parsed_message));
                    }
                }
            } else if let Some(tx_type) = message.split(':').next() {
                if tx_type.starts_with("ex-") {
                    let json_str = message[tx_type.len()+1..].trim_start_matches('\n');
                    if let Ok(parsed_message) = serde_json::from_str::<Value>(json_str) {
                        return Ok(CommitMessageType::Transaction(parsed_message));
                    }
                }
            }
        }
        Ok(CommitMessageType::Unknown)
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
                            Ok(CommitMessageType::Block(files_changed)) => {
                                println!("The latest block commit: {:#?}", files_changed);
                            },
                            Ok(CommitMessageType::Transaction(transaction)) => {
                                println!("The latest transaction commit: {:#?}", transaction);
                                self.execute(
                                    Transaction{
                                        author: "".to_string(),
                                        timestamp: 0,
                                        head: "".to_string(),
                                        body: transaction.to_string(),
                                        diff: Diff::None,
                                    },
                                    transaction["height"].as_u64().unwrap() as BlockHeight,
                                    // TODO: get_total_commits implementation
                                    MerkleProof{
                                        proof: vec![],
                                    }
                                ).await.unwrap()
                            },
                            Err(err) => {
                                println!("Error while handling commit: {}", err);
                            }
                            _ => {} // e.g. CommitMessageType::Unknown => no recognizable commit prefix
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
