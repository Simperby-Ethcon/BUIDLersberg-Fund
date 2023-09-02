//! An example of a settlement chain (Mythereum) abstracted in plain Rust code.
//!
//! This lightclient.rs is designed to use the settlement chain in the context of simperby-evm's code.

// Imports
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use simperby_core::{BlockHeader, BlockHeight, FinalizationProof, Hash256, HexSerializedVec, light_client, serde_spb, Transaction};
use rust_decimal::Decimal;
use simperby_core::merkle_tree::MerkleProof;
use serde::{Deserialize, Serialize};
use crate::util::string_to_hex;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Execution {
    /// The target settlement chain which this message will be delivered to.
    pub target_chain: String,
    /// An increasing sequence for the target contract to prevent replay attack.
    pub contract_sequence: u128,
    /// The actual content to deliver.
    pub message: ExecutionMessage,
}

/// Reads an execution transaction and tries to extract an execution message.
pub fn convert_transaction_to_execution(transaction: &Transaction) -> Result<Execution, String> {
    let segments = transaction.body.split("\n---\n").collect::<Vec<_>>();
    if segments.len() != 2 {
        return Err(format!(
            "Invalid body: expected 2 segments, got {}",
            segments.len()
        ));
    }
    let execution: Execution = serde_spb::from_str(segments[0]).map_err(|e| e.to_string())?;
    let hash = Hash256::hash(serde_spb::to_vec(&execution).unwrap());
    if format!("{hash}") != segments[1] {
        return Err(format!(
            "Invalid body: expected hash {hash}, got {}",
            segments[1]
        ));
    }

    if !transaction.head.starts_with("ex-") {
        return Err("Invalid head".to_string());
    }
    let execution_message =
        transaction.head.split(": ").next().ok_or("Invalid head")?[3..].to_owned();
    let target_chain = transaction.head.split(": ").nth(1).ok_or("Invalid head")?;
    if execution.target_chain != target_chain {
        return Err("Invalid target chain".to_string());
    }
    match execution_message.as_str() {
        "dummy" => {
            if !matches!(execution.message, ExecutionMessage::Dummy { .. }) {
                return Err("Invalid message".to_string());
            }
        }
        "transfer-ft" => {
            if !matches!(
                execution.message,
                ExecutionMessage::TransferFungibleToken { .. }
            ) {
                return Err("Invalid message".to_string());
            }
        }
        "transfer-nft" => {
            if !matches!(
                execution.message,
                ExecutionMessage::TransferNonFungibleToken { .. }
            ) {
                return Err("Invalid message".to_string());
            }
        }
        _ => return Err("Invalid message".to_string()),
    }
    Ok(execution)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ExecutionMessage {
    /// Does nothing but make the treasury contract verify the commitment anyway.
    Dummy { msg: String },
    /// Transfers a fungible token from the treasury contract.
    TransferFungibleToken(TransferFungibleToken),
    /// Transfers an NFT from the treasury contract.
    TransferNonFungibleToken(TransferNonFungibleToken),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TransferFungibleToken {
    pub token_address: HexSerializedVec,
    pub amount: Decimal,
    pub receiver_address: HexSerializedVec,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TransferNonFungibleToken {
    pub collection_address: HexSerializedVec,
    pub token_index: HexSerializedVec,
    pub receiver_address: HexSerializedVec,
}

pub struct TetherContract {
    pub(crate) balances: HashMap<HexSerializedVec, Decimal>,
}

pub trait MRC20 {
    fn get_balance(&self, address: &HexSerializedVec) -> Decimal;
    fn transfer(
        &mut self,
        context: &mut GlobalContext,
        to: &HexSerializedVec,
        amount: Decimal,
    ) -> bool;
}

impl MRC20 for TetherContract {
    fn get_balance(&self, address: &HexSerializedVec) -> Decimal {
        *self.balances.get(address).unwrap_or(&Decimal::ZERO)
    }

    fn transfer(
        &mut self,
        context: &mut GlobalContext,
        to: &HexSerializedVec,
        amount: Decimal,
    ) -> bool {
        let from_balance = self.get_balance(&context.caller);
        if from_balance < amount {
            return false;
        }
        let to_balance = self.get_balance(to);
        self.balances
            .insert(context.caller.clone(), from_balance - amount);
        self.balances.insert(to.clone(), to_balance + amount);
        true
    }
}

pub struct GlobalContext {
    pub tether: Rc<RefCell<TetherContract>>,
    pub caller: HexSerializedVec,
}

pub struct MythereumTreasuryContract {
    light_client: light_client::LightClient,
    sequence: u128,
}

impl MythereumTreasuryContract {
    pub fn new(header: BlockHeader) -> Result<Self, String> {
        let light_client = light_client::LightClient::new(header);
        Ok(Self {
            light_client,
            sequence: 0,
        })
    }

    pub fn update_light_client(
        &mut self,
        _context: &mut GlobalContext,
        header: BlockHeader,
        proof: FinalizationProof,
    ) -> Result<(), String> {
        self.light_client.update(header, proof)
    }

    pub fn execute(
        &mut self,
        context: &mut GlobalContext,
        execution_transaction: Transaction,
        simperby_height: BlockHeight,
        proof: MerkleProof,
    ) -> Result<(), String> {
        let execution = convert_transaction_to_execution(&execution_transaction)?;
        if execution.contract_sequence != self.sequence {
            return Err("Invalid sequence".to_string());
        }
        if execution.target_chain != "mythereum" {
            return Err("Invalid target chain".to_string());
        }

        if !self.light_client.verify_transaction_commitment(
            &execution_transaction,
            simperby_height,
            proof,
        ) {
            return Err("Invalid proof".to_string());
        }

        match execution.message {
            ExecutionMessage::Dummy { msg } => {
                unimplemented!("Should emit an event with the message ({})", msg)
            }
            ExecutionMessage::TransferFungibleToken(TransferFungibleToken {
                                                        token_address,
                                                        amount,
                                                        receiver_address,
                                                    }) => {
                if token_address != string_to_hex("tether-address") {
                    unimplemented!()
                }
                let tether_rc = Rc::clone(&context.tether);
                let mut tether = tether_rc.borrow_mut();
                context.caller = string_to_hex("treasury-address");
                if !tether.transfer(context, &receiver_address, amount) {
                    return Err("Insufficient balance".to_string());
                }
            }
            ExecutionMessage::TransferNonFungibleToken(_) => todo!(),
        }

        self.sequence += 1;
        Ok(())
    }
}