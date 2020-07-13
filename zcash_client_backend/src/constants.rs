//! Zcash global and per-network constants.

pub mod mainnet;
pub mod regtest;
pub mod testnet;

pub const SPROUT_CONSENSUS_BRANCH_ID: u32 = 0;
pub const OVERWINTER_CONSENSUS_BRANCH_ID: u32 = 0x6f76_727a;
pub const SAPLING_CONSENSUS_BRANCH_ID: u32 = 0x7361_707a;
