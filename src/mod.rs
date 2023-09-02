use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Chain;
use std::rc::Rc;
use simperby_core::{BlockHeader, BlockHeight, FinalizationProof, HexSerializedVec, Transaction};
use simperby_core::merkle_tree::MerkleProof;
use simperby_evm_client::{ChainType, EvmCompatibleAddress};
use crate::lightclient::{MythereumTreasuryContract};
use crate::util::string_to_hex;

mod lightclient;
mod util;
mod relayer;
