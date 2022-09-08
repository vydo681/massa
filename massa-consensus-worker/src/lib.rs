// Copyright (c) 2022 MASSA LABS <info@massa.net>

#![doc = include_str!("../endorsements.md")]
#![feature(async_closure)]
#![feature(hash_drain_filter)]
#![feature(map_first_last)]
#![feature(int_roundings)]
#![feature(deadline_api)]
#![warn(missing_docs)]
#![warn(unused_crate_dependencies)]
#[macro_use]
extern crate massa_logging;

mod consensus_worker;
mod tools;
use std::sync::mpsc::SyncSender;

use massa_consensus_exports::ConsensusError;
use massa_graph::BootstrapableGraph;
pub use tools::start_consensus_controller;

#[cfg(test)]
mod tests;

pub(crate) enum Command {
    GetBootstrapState {
        response_tx: SyncSender<Result<Vec<u64>, ConsensusError>>,
    },
}
