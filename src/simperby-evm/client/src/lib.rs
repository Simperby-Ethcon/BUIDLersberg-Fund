use async_trait::async_trait;
use dotenvy_macro::{self, dotenv};
use ethers::signers::Signer;
use ethers::signers::coins_bip39::mnemonic;
use ethers::signers::{coins_bip39::English, LocalWallet, MnemonicBuilder};
use ethers::types::{H160, U256};
use ethers::{contract::abigen, middleware::SignerMiddleware, types::Address};
use ethers_core::k256::ecdsa::SigningKey;
use ethers_core::types::{BlockId, BlockNumber, Bytes};
use ethers_providers::{Http, Middleware, Provider};
use eyre::Error;
use merkle_tree::MerkleProof;
use rust_decimal::Decimal;
use simperby_core::*;
use simperby_settlement::execution::convert_transaction_to_execution;
use simperby_settlement::*;
use std::str::FromStr;
use std::sync::Arc;
use simperby_core::crypto::HexSerializedVec;

const EVM_COMPATIBLE_ADDRESS_BYTES: usize = 20;

abigen!(
    ITreasury,
    r#"[
        function name() external view returns (string memory)
        function chainName() external view returns (bytes memory)
        function contractSequence() external view returns (uint128)
        function lightClient() external view returns (uint64 heightOffset, bytes memory lastHeader)
        function viewCommitRoots() external view returns (bytes32[] memory commitRoots)
        function updateLightClient(bytes memory header, bytes memory proof) public
        function execute(bytes memory transaction,bytes memory executionHash, uint64 blockHeight, bytes memory merkleProof) public
    ]"#,
);

abigen!(
    IERC20,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function totalSupply() public view returns (uint256)
        function transfer(address _to, uint256 _value) public returns (bool success)
        function transferFrom(address _from, address _to, uint256 _value) public returns (bool success)
    ]"#,
);

abigen!(
    IERC721,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function tokenOfOwnerByIndex(address owner, uint256 index) external view returns (uint256)
    ]"#,
);

abigen!(
    UniswapV2,
    r#"[

    ]"#,
);

pub struct ChainConfigs {
    /// The RPC URL of the chain
    rpc_url: String,
    /// The name of the chain
    chain_name: Option<String>,
}

pub enum ChainType {
    Ethereum(ChainConfigs),
    Goerli(ChainConfigs),
    Other(ChainConfigs),
}

impl ChainType {
    fn get_rpc_url(&self) -> &str {
        match self {
            ChainType::Ethereum(chain) => chain.rpc_url.as_str(),
            ChainType::Goerli(chain) => chain.rpc_url.as_str(),
            ChainType::Other(chain) => chain.rpc_url.as_str(),
        }
    }

    fn get_chain_name(&self) -> &str {
        match self {
            ChainType::Ethereum(_) => "Ethereum",
            ChainType::Goerli(_) => "Goerli",
            ChainType::Other(configs) => {
                if configs.chain_name.is_some() {
                    configs.chain_name.as_ref().unwrap().as_str()
                } else {
                    "Unknown"
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvmCompatibleAddress {
    pub address: Address,
}

impl EvmCompatibleAddress {
    pub fn to_hex_str(&self) -> String {
        format!("0x{}", hex::encode(self.address.as_fixed_bytes()))
    }

    pub fn to_hex_str_without_prefix(&self) -> String {
        hex::encode(self.address.as_fixed_bytes())
    }

    pub fn to_hex_serialized_vec(&self) -> HexSerializedVec {
        HexSerializedVec {
            data: self.address.as_bytes().to_vec(),
        }
    }

    pub fn from_hex_str(address: &str) -> Result<EvmCompatibleAddress, Error> {
        let address = if address.len() == 2 * EVM_COMPATIBLE_ADDRESS_BYTES + 2 {
            if !address.starts_with("0x") {
                return Err(eyre::eyre!(
                    "Invalid address format: missing 0x prefix or invalid length({})",
                    address.len()
                ));
            }
            address[2..].to_string()
        } else if address.len() == 2 * EVM_COMPATIBLE_ADDRESS_BYTES {
            address.to_string()
        } else {
            return Err(eyre::eyre!(
                "Invalid address format: invalid length({})",
                address.len()
            ));
        };
        address
            .parse::<Address>()
            .map_err(|e| eyre::eyre!("Invalid address format: {}", e))
            .map(|address| EvmCompatibleAddress { address })
    }

    pub fn from_hex_serialized_vec(
        address: &HexSerializedVec,
    ) -> Result<EvmCompatibleAddress, Error> {
        let address = if address.data.len() == EVM_COMPATIBLE_ADDRESS_BYTES {
            hex::encode(&address.data)
        } else {
            return Err(eyre::eyre!("Invalid address format: invalid length"));
        };
        address
            .parse::<Address>()
            .map_err(|e| eyre::eyre!("Invalid address format : {}", e))
            .map(|address| EvmCompatibleAddress { address })
    }
}

pub struct EvmCompatibleChain {
    pub chain: ChainType,
    pub treasury_address: Option<EvmCompatibleAddress>,
}

#[async_trait]
impl SettlementChain for EvmCompatibleChain {
    async fn get_chain_name(&self) -> String {
        self.chain.get_chain_name().to_string()
    }

    async fn check_connection(&self) -> Result<(), Error> {
        let provider = Provider::<Http>::try_from(self.chain.get_rpc_url())?;
        let block_number = provider.get_block_number().await;
        if block_number.is_err() {
            return Err(eyre::eyre!(format!(
                "Failed to connect to chain {}",
                self.chain.get_chain_name()
            )));
        }
        Ok(())
    }

    async fn get_last_block(&self) -> Result<SettlementChainBlock, Error> {
        let provider = Provider::<Http>::try_from(self.chain.get_rpc_url())?;
        let block = provider
            .get_block_with_txs(BlockId::Number(BlockNumber::Latest))
            .await?;
        if let Some(block) = block {
            let height = block.number.unwrap().as_u64();
            let timestamp = block.timestamp.as_u64();
            return Ok(SettlementChainBlock { height, timestamp });
        } else {
            return Err(eyre::eyre!(format!(
                "Failed to get last block from chain {}",
                self.chain.get_chain_name()
            )));
        }
    }

    async fn get_contract_sequence(&self) -> Result<u128, Error> {
        let treasury = if let Some(address) = &self.treasury_address {
            address
        } else {
            return Err(eyre::eyre!("Treasury address is not set"));
        };
        let provider = Provider::<Http>::try_from(self.chain.get_rpc_url())?;
        let contract = ITreasury::new(treasury.address, Arc::new(provider));
        let contract_sequence = contract.contract_sequence().call().await?;
        Ok(contract_sequence)
    }

    async fn get_relayer_account_info(&self) -> Result<(HexSerializedVec, Decimal), Error> {
        let provider = Provider::<Http>::try_from(self.chain.get_rpc_url())?;
        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        let test_mnemonic: &str = "candy maple cake sugar pudding cream honey rich smooth crumble sweet treat";
        // let mnemonic = dotenv!("RELAYER_MNEMONIC").to_string();
        let wallet: LocalWallet = MnemonicBuilder::<English>::default()
            .phrase(test_mnemonic)
            .build()
            .unwrap()
            .with_chain_id(chain_id);
        let relayer_address: H160 = wallet.address();
        let provider = Provider::<Http>::try_from(self.chain.get_rpc_url())?;
        let balance = provider
            .get_balance(relayer_address, None)
            .await?
            .to_string();
        let address = HexSerializedVec::from(relayer_address.as_bytes().to_vec());
        Ok((
            address,
            Decimal::from_str(balance.as_str()).map_err(|_| {
                eyre::eyre!(format!("Failed to parse balance {} to decimal", balance))
            })?,
        ))
    }

    async fn get_light_client_header(&self) -> Result<BlockHeader, Error> {
        let treasury = if let Some(address) = &self.treasury_address {
            address
        } else {
            return Err(eyre::eyre!("Treasury address is not set"));
        };
        let provider = Provider::<Http>::try_from(self.chain.get_rpc_url())?;
        let contract = ITreasury::new(treasury.address, Arc::new(&provider));
        let (_, last_header) = contract.light_client().call().await.unwrap();
        let light_client_header: BlockHeader = serde_spb::from_slice(&last_header).unwrap();
        Ok(light_client_header)
    }

    async fn get_treasury_fungible_token_balance(
        &self,
        address: HexSerializedVec,
    ) -> Result<Decimal, Error> {
        let treasury = if let Some(address) = &self.treasury_address {
            address
        } else {
            return Err(eyre::eyre!("Treasury address is not set"));
        };
        let contract_address = EvmCompatibleAddress::from_hex_serialized_vec(&address)?.address;
        let provider = Provider::<Http>::try_from(self.chain.get_rpc_url())?;
        let contract = IERC20::new(contract_address, Arc::new(provider));
        let balance = contract.balance_of(treasury.address).call().await.unwrap();
        Ok(Decimal::from(balance.as_u128()))
    }

    async fn get_treasury_non_fungible_token_balance(
        &self,
        address: HexSerializedVec,
    ) -> Result<Vec<HexSerializedVec>, Error> {
        todo!()
    }

    async fn update_treasury_light_client(
        &self,
        header: BlockHeader,
        proof: FinalizationProof,
    ) -> Result<(), Error> {
        let treasury = if let Some(address) = &self.treasury_address {
            address
        } else {
            return Err(eyre::eyre!("Treasury address is not set"));
        };
        let provider = Provider::<Http>::try_from(self.chain.get_rpc_url())?;
        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        // let mnemonic = dotenv!("RELAYER_MNEMONIC");
        let test_mnemonic: &str = "candy maple cake sugar pudding cream honey rich smooth crumble sweet treat";
        let wallet: LocalWallet = MnemonicBuilder::<English>::default()
            .phrase(test_mnemonic)
            .build()
            .unwrap()
            .with_chain_id(chain_id);
        let client = SignerMiddleware::new(&provider, wallet);
        let contract = ITreasury::new(treasury.address, Arc::new(client));
        let header = Bytes::from(
            serde_spb::to_vec(&header)
                .map_err(|_| eyre::eyre!("Failed to serialize block header"))?,
        );
        let proof = Bytes::from(
            serde_spb::to_vec(&proof)
                .map_err(|_| eyre::eyre!("Failed to serialize finalization proof"))?,
        );
        contract
            .update_light_client(header, proof)
            .gas_price(U256::from(10000000000u64))
            .send()
            .await
            .map_err(|err| eyre::eyre!("Failed to update light client: {}", err))?;
        Ok(())
    }

    async fn execute(
        &self,
        transaction: Transaction,
        block_height: u64,
        proof: MerkleProof,
    ) -> Result<(), Error> {
        let treasury = if let Some(address) = &self.treasury_address {
            address
        } else {
            return Err(eyre::eyre!("Treasury address is not set"));
        };
        let provider = Provider::<Http>::try_from(self.chain.get_rpc_url())?;
        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        // let mnemonic = dotenv!("RELAYER_MNEMONIC");
        let test_mnemonic: &str = "candy maple cake sugar pudding cream honey rich smooth crumble sweet treat";
        let wallet: LocalWallet = MnemonicBuilder::<English>::default()
            .phrase(test_mnemonic)
            .build()
            .unwrap()
            .with_chain_id(chain_id);
        let client = SignerMiddleware::new(&provider, wallet);
        let contract = ITreasury::new(treasury.address, Arc::new(client));
        let execution = convert_transaction_to_execution(&transaction).map_err(|_| {
            eyre::eyre!(format!(
                "Failed to convert transaction to execution: {:?}",
                transaction
            ))
        })?;
        let transaction = Bytes::from(
            serde_spb::to_vec(&transaction)
                .map_err(|_| eyre::eyre!("Failed to serialize transaction"))?,
        );

        let execution = Bytes::from(
            serde_spb::to_vec(&execution)
                .map_err(|_| eyre::eyre!("Failed to serialize execution"))?,
        );
        let proof = Bytes::from(
            serde_spb::to_vec(&proof)
                .map_err(|_| eyre::eyre!("Failed to serialize merkle proof"))?,
        );
        contract
            .execute(transaction, execution, block_height, proof)
            .send()
            .await
            .map_err(|err| eyre::eyre!(format!("Failed to execute: {:?}", err)))?;
        Ok(())
    }

    async fn eoa_get_sequence(&self, address: HexSerializedVec) -> Result<u128, Error> {
        let eoa = EvmCompatibleAddress::from_hex_serialized_vec(&address)?.address;
        let provider = Provider::<Http>::try_from(self.chain.get_rpc_url())?;
        let sequence = provider
            .get_transaction_count(eoa, None)
            .await
            .map_err(|_| eyre::eyre!(format!("Failed to get sequence for address: {:?}", eoa)))?
            .as_u128();
        Ok(sequence)
    }

    async fn eoa_get_fungible_token_balance(
        &self,
        address: HexSerializedVec,
        token_address: HexSerializedVec,
    ) -> Result<Decimal, Error> {
        let eoa = EvmCompatibleAddress::from_hex_serialized_vec(&address)?.address;
        let contract_address =
            EvmCompatibleAddress::from_hex_serialized_vec(&token_address)?.address;
        let provider = Provider::<Http>::try_from(self.chain.get_rpc_url())?;
        let contract = IERC20::new(contract_address, Arc::new(provider));
        let balance = contract.balance_of(eoa).call().await.unwrap();
        Ok(Decimal::from(balance.as_u128()))
    }

    async fn eoa_transfer_fungible_token(
        &self,
        address: HexSerializedVec,
        sender_private_key: HexSerializedVec,
        token_address: HexSerializedVec,
        receiver_address: HexSerializedVec,
        amount: Decimal,
    ) -> Result<(), Error> {
        let provider = Provider::<Http>::try_from(self.chain.get_rpc_url())?;
        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        let eoa = EvmCompatibleAddress::from_hex_serialized_vec(&address)?.address;
        let signer = SigningKey::from_slice(sender_private_key.data.as_slice())?;
        let wallet = LocalWallet::new_with_signer(signer, eoa, chain_id);
        let client = SignerMiddleware::new(&provider, wallet);
        let contract_address =
            EvmCompatibleAddress::from_hex_serialized_vec(&token_address)?.address;
        let contract = IERC20::new(contract_address, Arc::new(client));
        let receiver_address =
            EvmCompatibleAddress::from_hex_serialized_vec(&receiver_address)?.address;
        let amount = U256::from_dec_str(amount.to_string().as_str()).unwrap();
        contract
            .transfer(receiver_address, amount)
            .send()
            .await
            .map_err(|_| eyre::eyre!("Failed to transfer fungible token"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rust_decimal::prelude::FromPrimitive;
    use simperby_core::merkle_tree::OneshotMerkleTree;
    use simperby_core::{verify::CommitSequenceVerifier, FinalizationProof};
    use simperby_settlement::execution::TransferFungibleToken;
    use simperby_settlement::execution::{Execution, ExecutionMessage};
    use std::thread::sleep;
    use std::time::Duration;

    /// Constants for testing
    // TODO: fill in test constants
    // WARNING: DO NOT USE REAL FUNDS FOR TESTS
    const TEST_RPC_URL: &str = "http://localhost:8545";
    const TEST_CHAIN_NAME: &str = "localhost";
    const TEST_LIGHT_CLIENT_HEADER: &str = "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040000000000000004b31b74ad078b082cad69775717016d7fbfae7b9f7dde8d1d988e0ff2e2b30e9413090e436c7c2a2c06e7ddf69484aeaaadc7ecbf1dd92459769ba96043a07564010000000000000004a688f0a4f9c863b6aa927e0df198307e058999c3ea8a012e47e1c598a70b67b383c8a3f7b2a392904e71689595147334e821985b1175b10fbc47d1d9ffd4ec6b010000000000000004c1b5a31db87d102ac45efe81288a1ea380abca214a37b3b9bc9ad1da984f08c4d40e948e6548df924ee7f2513324136f40fe20ebe77a1ee019e526ea6e3b974c01000000000000000420e4b9d289f068377a1ec0c37fd89661a60351914cacaca2f116c95d0ec0e8a7f48f22a495f6922c8b48790975d4a639f320135e89c98c30cf0da2201fc5145501000000000000000500000000000000302e312e30";
    const TEST_EOA_ADDRESS: &str = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    const TEST_EOA_PRIV_KEY: &str =
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    const TEST_TREASURY_ADDRESS: &str = "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0";
    const TEST_ERC20_ADDRESS: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
    // const TEST_ERC721_ADDRESS: &str = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512";
    const DURATION_IN_SECONDS: u64 = 10;

    #[ignore]
    #[tokio::test]
    async fn test_chain_basics() {
        let test_chain = EvmCompatibleChain {
            chain: ChainType::Other(ChainConfigs {
                rpc_url: TEST_RPC_URL.to_owned(),
                chain_name: Some(TEST_CHAIN_NAME.to_owned()),
            }),
            treasury_address: Some(
                EvmCompatibleAddress::from_hex_str(TEST_TREASURY_ADDRESS).unwrap(),
            ),
        };
        assert_eq!(test_chain.get_chain_name().await, "localhost");
        assert_eq!(test_chain.chain.get_rpc_url(), TEST_RPC_URL);
        assert_eq!(
            test_chain
                .treasury_address
                .unwrap()
                .to_hex_str()
                .to_uppercase(),
            TEST_TREASURY_ADDRESS.to_uppercase()
        );
    }

    #[ignore]
    #[tokio::test]
    async fn check_connection() {
        let test_chain = EvmCompatibleChain {
            chain: ChainType::Other(ChainConfigs {
                rpc_url: TEST_RPC_URL.to_owned(),
                chain_name: Some(TEST_CHAIN_NAME.to_owned()),
            }),
            treasury_address: Some(
                EvmCompatibleAddress::from_hex_str(TEST_TREASURY_ADDRESS).unwrap(),
            ),
        };
        test_chain.check_connection().await.unwrap();
    }

    #[ignore]
    #[tokio::test]
    async fn get_last_block() {
        let test_chain = EvmCompatibleChain {
            chain: ChainType::Other(ChainConfigs {
                rpc_url: TEST_RPC_URL.to_owned(),
                chain_name: Some(TEST_CHAIN_NAME.to_owned()),
            }),
            treasury_address: Some(
                EvmCompatibleAddress::from_hex_str(TEST_TREASURY_ADDRESS).unwrap(),
            ),
        };
        let block = test_chain.get_last_block().await.unwrap();
        assert!(block.height > 0 || block.height == 0);
        assert!(block.timestamp > 0);
    }

    #[ignore]
    #[tokio::test]
    async fn get_relayer_account_info() {
        let provider = Provider::<Http>::try_from(TEST_RPC_URL).unwrap();
        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        let test_mnemonic: &str = "candy maple cake sugar pudding cream honey rich smooth crumble sweet treat";
        let wallet: LocalWallet = MnemonicBuilder::<English>::default()
            .phrase(test_mnemonic)
            .build()
            .unwrap()
            .with_chain_id(chain_id);
        let relayer_address = EvmCompatibleAddress {
            address: wallet.address(),
        };
        let test_chain = EvmCompatibleChain {
            chain: ChainType::Other(ChainConfigs {
                rpc_url: TEST_RPC_URL.to_owned(),
                chain_name: Some(TEST_CHAIN_NAME.to_owned()),
            }),
            treasury_address: Some(EvmCompatibleAddress {
                address: relayer_address.address.clone(),
            }),
        };
        let (address, balance) = test_chain.get_relayer_account_info().await.unwrap();
        let address = EvmCompatibleAddress::from_hex_serialized_vec(&address)
            .unwrap()
            .to_hex_str();
        assert_eq!(address, relayer_address.to_hex_str());
        assert!(balance >= Decimal::from_usize(0).unwrap());
    }

    #[ignore]
    #[tokio::test]
    async fn get_contract_sequence() {
        let test_chain = EvmCompatibleChain {
            chain: ChainType::Other(ChainConfigs {
                rpc_url: TEST_RPC_URL.to_owned(),
                chain_name: Some(TEST_CHAIN_NAME.to_owned()),
            }),
            treasury_address: Some(
                EvmCompatibleAddress::from_hex_str(TEST_TREASURY_ADDRESS).unwrap(),
            ),
        };
        let sequence = test_chain.get_contract_sequence().await.unwrap();
        println!("sequence: {:?}", sequence);
    }

    #[ignore]
    #[tokio::test]
    async fn get_current_light_client_header() {
        let test_chain = EvmCompatibleChain {
            chain: ChainType::Other(ChainConfigs {
                rpc_url: TEST_RPC_URL.to_owned(),
                chain_name: Some(TEST_CHAIN_NAME.to_owned()),
            }),
            treasury_address: Some(
                EvmCompatibleAddress::from_hex_str(TEST_TREASURY_ADDRESS).unwrap(),
            ),
        };
        let header = test_chain.get_light_client_header().await.unwrap();
        let header = hex::encode(serde_spb::to_vec(&header).unwrap());
        assert_eq!(format!("0x{}", header), TEST_LIGHT_CLIENT_HEADER);
    }

    pub struct Chain {
        pub chain_name: String,
        pub last_finalized_header: BlockHeader,
        pub last_finalization_proof: FinalizationProof,
        pub reserved_state: ReservedState,
        pub validators: Vec<PrivateKey>,
    }

    impl Chain {
        pub fn standard_genesis(chain_name: String) -> Self {
            let (reserved_state, validators) = test_utils::generate_standard_genesis(4);
            Self {
                chain_name,
                last_finalized_header: reserved_state.genesis_info.header.clone(),
                last_finalization_proof: reserved_state.genesis_info.genesis_proof.clone(),
                reserved_state,
                validators: validators
                    .into_iter()
                    .map(|(_, private_key)| private_key)
                    .collect(),
            }
        }
    }

    #[ignore]
    #[tokio::test]
    async fn update_light_client_and_execute_right_after_genesis() {
        // Set up the on-chain state
        let sc = EvmCompatibleChain {
            chain: ChainType::Other(ChainConfigs {
                rpc_url: TEST_RPC_URL.to_owned(),
                chain_name: Some("Local".to_owned()),
            }),
            treasury_address: Some(
                EvmCompatibleAddress::from_hex_str(TEST_TREASURY_ADDRESS).unwrap(),
            ),
        };
        let chain = Chain::standard_genesis("mythereum".to_owned());
        let mut csv = CommitSequenceVerifier::new(
            chain.last_finalized_header.clone(),
            chain.reserved_state.clone(),
        )
        .unwrap();
        // Query the initial status
        let initial_balance = sc
            .get_treasury_fungible_token_balance(
                EvmCompatibleAddress::from_hex_str(TEST_ERC20_ADDRESS)
                    .unwrap()
                    .to_hex_serialized_vec(),
            )
            .await
            .unwrap();
        let initial_contract_sequence = sc.get_contract_sequence().await.unwrap();
        let initial_temporary_receiver_balance = sc
            .eoa_get_fungible_token_balance(
                EvmCompatibleAddress::from_hex_str(TEST_EOA_ADDRESS)
                    .unwrap()
                    .to_hex_serialized_vec(),
                EvmCompatibleAddress::from_hex_str(TEST_ERC20_ADDRESS)
                    .unwrap()
                    .to_hex_serialized_vec(),
            )
            .await
            .unwrap();
        // Apply transactions
        let mut transactions = Vec::new();
        let erc20_address = HexSerializedVec {
            data: EvmCompatibleAddress::from_hex_str(TEST_ERC20_ADDRESS)
                .unwrap()
                .to_hex_serialized_vec()
                .data,
        };
        let temporary_receiver_address = HexSerializedVec {
            data: EvmCompatibleAddress::from_hex_str(TEST_EOA_ADDRESS)
                .unwrap()
                .to_hex_serialized_vec()
                .data,
        };
        let execute_tx = execution::create_execution_transaction(
            &Execution {
                target_chain: chain.chain_name,
                contract_sequence: initial_contract_sequence,
                message: ExecutionMessage::TransferFungibleToken(TransferFungibleToken {
                    token_address: erc20_address.clone(),
                    amount: initial_balance,
                    receiver_address: temporary_receiver_address.clone(),
                }),
            },
            "jinwoo".to_owned(),
            0,
        )
        .unwrap();
        csv.apply_commit(&Commit::Transaction(execute_tx.clone()))
            .unwrap();
        transactions.push(execute_tx.clone());
        // Complete the block
        let agenda = Agenda {
            height: 1,
            author: chain.reserved_state.consensus_leader_order[0].clone(),
            timestamp: 1,
            transactions_hash: Agenda::calculate_transactions_hash(&transactions),
            previous_block_hash: chain.last_finalized_header.to_hash256(),
        };
        csv.apply_commit(&Commit::Agenda(agenda.clone())).unwrap();
        csv.apply_commit(&Commit::AgendaProof(AgendaProof {
            height: 1,
            agenda_hash: agenda.to_hash256(),
            proof: chain
                .validators
                .iter()
                .map(|private_key| TypedSignature::sign(&agenda, private_key).unwrap())
                .collect::<Vec<_>>(),
            timestamp: 0,
        }))
        .unwrap();
        let block_header = BlockHeader {
            author: chain.validators[0].public_key(),
            prev_block_finalization_proof: chain.last_finalization_proof,
            previous_hash: chain.last_finalized_header.to_hash256(),
            height: 1,
            timestamp: 0,
            commit_merkle_root: BlockHeader::calculate_commit_merkle_root(
                &csv.get_total_commits()[1..],
            ),
            repository_merkle_root: Hash256::zero(),
            validator_set: chain.last_finalized_header.validator_set.clone(),
            version: chain.last_finalized_header.version,
        };
        csv.apply_commit(&Commit::Block(block_header.clone()))
            .unwrap();
        let signatures = chain
            .validators
            .iter()
            .map(|private_key| {
                TypedSignature::sign(
                    &FinalizationSignTarget {
                        round: 0,
                        block_hash: block_header.to_hash256(),
                    },
                    private_key,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let fp = FinalizationProof {
            round: 0,
            signatures,
        };
        csv.verify_last_header_finalization(&fp).unwrap();
        // Update light client
        sc.update_treasury_light_client(block_header.clone(), fp)
            .await
            .unwrap();
        // Check time duration to wait for transaction to be confirmed on mainnet or testnet
        sleep(Duration::from_secs(DURATION_IN_SECONDS));
        assert_eq!(sc.get_light_client_header().await.unwrap(), block_header);
        // Execute transfer
        let commits = csv.get_total_commits();
        let merkle_tree = OneshotMerkleTree::create(
            commits[1..=(commits.len() - 2)]
                .iter()
                .map(|c| c.to_hash256())
                .collect(),
        );
        let merkle_proof = merkle_tree
            .create_merkle_proof(execute_tx.to_hash256())
            .unwrap();
        sc.execute(execute_tx, 1, merkle_proof).await.unwrap();
        // Check time duration to wait for transaction to be confirmed on mainnet or testnet
        sleep(Duration::from_secs(DURATION_IN_SECONDS));
        let balance_after_tx = sc
            .eoa_get_fungible_token_balance(
                EvmCompatibleAddress::from_hex_str(TEST_EOA_ADDRESS)
                    .unwrap()
                    .to_hex_serialized_vec(),
                EvmCompatibleAddress::from_hex_str(TEST_ERC20_ADDRESS)
                    .unwrap()
                    .to_hex_serialized_vec(),
            )
            .await
            .unwrap();
        let contract_sequence_after_execution = sc.get_contract_sequence().await.unwrap();
        assert_eq!(
            balance_after_tx,
            initial_temporary_receiver_balance + initial_balance
        );
        assert_eq!(
            contract_sequence_after_execution,
            initial_contract_sequence + 1
        );
    }

    #[ignore]
    #[tokio::test]
    async fn transfer_ft_from_eoa_to_treasury() {
        let eoa = EvmCompatibleAddress::from_hex_str(TEST_EOA_ADDRESS)
            .unwrap()
            .to_hex_serialized_vec();
        let test_chain = EvmCompatibleChain {
            chain: ChainType::Other(ChainConfigs {
                rpc_url: TEST_RPC_URL.to_owned(),
                chain_name: Some(TEST_CHAIN_NAME.to_owned()),
            }),
            treasury_address: Some(
                EvmCompatibleAddress::from_hex_str(TEST_TREASURY_ADDRESS).unwrap(),
            ),
        };
        let ft_address = EvmCompatibleAddress::from_hex_str(TEST_ERC20_ADDRESS)
            .unwrap()
            .to_hex_serialized_vec();
        let sequence_before = test_chain.eoa_get_sequence(eoa.clone()).await.unwrap();
        let eoa_balance_before = test_chain
            .eoa_get_fungible_token_balance(eoa.clone(), ft_address.clone())
            .await
            .unwrap();
        let treasury_balance_before = test_chain
            .get_treasury_fungible_token_balance(ft_address.clone())
            .await
            .unwrap();
        let treasury_address = EvmCompatibleAddress::from_hex_str(TEST_TREASURY_ADDRESS)
            .unwrap()
            .to_hex_serialized_vec();
        let amount = eoa_balance_before / Decimal::from(2usize);
        let eoa_priv_key = HexSerializedVec {
            data: hex::decode(&TEST_EOA_PRIV_KEY[2..]).unwrap(),
        };
        test_chain
            .eoa_transfer_fungible_token(
                eoa.clone(),
                eoa_priv_key,
                ft_address.clone(),
                treasury_address.clone(),
                amount,
            )
            .await
            .unwrap();
        // Check time duration to wait for transaction to be confirmed on mainnet or testnet
        sleep(Duration::from_secs(DURATION_IN_SECONDS));
        let sequence_after = test_chain.eoa_get_sequence(eoa.clone()).await.unwrap();
        let eoa_balance_after = test_chain
            .eoa_get_fungible_token_balance(eoa.clone(), ft_address.clone())
            .await
            .unwrap();
        let treasury_balance_after = test_chain
            .get_treasury_fungible_token_balance(ft_address.clone())
            .await
            .unwrap();
        assert_eq!(eoa_balance_after + amount, eoa_balance_before);
        assert_eq!(treasury_balance_after, treasury_balance_before + amount);
        assert_eq!(sequence_after, sequence_before + 1);
    }
}
