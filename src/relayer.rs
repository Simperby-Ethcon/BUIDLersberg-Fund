use crate::extract_block_header;
use crate::lightclient::*;
use reqwest::header;
use serde_json::Value;
use simperby::Client;
use simperby_core::merkle_tree::MerkleProof;
use simperby_core::{BlockHeader, BlockHeight, Commit, CommitHash, Diff, Transaction};
use simperby_evm_client::{ChainType, EvmCompatibleAddress};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

// The relayer will manage a list of light clients.
pub struct Relayer {
    light_clients: Vec<TreasuryContract>,
    reqwest_client: reqwest::Client,
    token: String,
    github_url: String,
    simperby_client: Client,
    block_height: BlockHeight,
}

pub enum CommitMessageType {
    Block(Value),
    Transaction(Value),
    Unknown,
}

impl Relayer {
    pub fn new(token: &str, github_url: &str, simperby_client: Client) -> Self {
        Self {
            light_clients: Vec::new(),
            reqwest_client: reqwest::Client::new(),
            token: token.to_string(),
            github_url: github_url.to_string(),
            simperby_client,
            // if u64::MAX, then it means that the block_height has not been set up
            block_height: u64::MAX,
        }
    }

    pub fn set_block_height(&mut self, block_height: BlockHeight) {
        self.block_height = block_height;
        println!("Relayer block height set to {}", block_height);
    }

    pub async fn poll_github(
        &self,
        only_latest: bool,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let res = self
            .reqwest_client
            .get(&self.github_url)
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

    pub async fn handle_commit_body(
        &self,
        commit: &Value,
    ) -> Result<CommitMessageType, Box<dyn std::error::Error>> {
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
                    let json_str = message[tx_type.len() + 1..].trim_start_matches('\n');
                    if let Ok(parsed_message) = serde_json::from_str::<Value>(json_str) {
                        return Ok(CommitMessageType::Transaction(parsed_message));
                    }
                }
            }
        }
        Ok(CommitMessageType::Unknown)
    }

    pub fn initialize_light_client(
        &mut self,
        header: BlockHeader,
        chain: ChainType,
        address: Option<EvmCompatibleAddress>,
        block_height: BlockHeight,
    ) -> Result<(), String> {
        let client = TreasuryContract::new(header, chain, address, block_height.clone())?;
        self.light_clients.push(client);
        Ok(())
    }

    pub async fn execute(
        &mut self,
        transaction: Transaction,
        height: BlockHeight,
        proof: MerkleProof,
    ) -> Result<(), String> {
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
                                match extract_block_header(&files_changed) {
                                    Ok(header) => {
                                        println!("{:?}", header);
                                        self.set_block_height(header.height);
                                    }
                                    Err(e) => {
                                        // Handle the error from extract_block_header
                                        println!("Failed to extract block header: {}", e);
                                    }
                                }
                            }
                            // Ok(CommitMessageType::Block(files_changed)) => {
                            Ok(CommitMessageType::Transaction(transaction)) => {
                                println!("The latest transaction commit: {:#?}", transaction);
                                // TODO: extract commit_hash from transaction
                                let commit_hash =
                                    "7e2c04181d63b75c1060a016c16d53fea7eb3301".to_string();
                                let hash = hex::decode(&commit_hash).unwrap().try_into().unwrap();
                                let data = self
                                    .simperby_client
                                    .repository()
                                    .read_commit(CommitHash { hash })
                                    .await
                                    .unwrap();
                                dbg!("data: {:?}", data.clone());
                                match data {
                                    Commit::Transaction(data) => {
                                        self.execute(
                                            Transaction {
                                                author: data.author.clone(),
                                                timestamp: data.timestamp,
                                                head: data.head.clone(),
                                                body: data.body.clone(),
                                                diff: data.diff.clone(), // assuming diff is Option<Diff>
                                            },
                                            self.block_height,
                                            MerkleProof { proof: vec![] },
                                        )
                                        .await
                                        .unwrap();
                                    }
                                    _ => {
                                        println!("The commit data is not of transaction type");
                                    }
                                }
                            }
                            _ => {
                                println!("not supported commit message type at the moment");
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("Error while polling Github: {}", err);
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
}
