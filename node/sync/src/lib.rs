// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![forbid(unsafe_code)]

#[macro_use]
extern crate tracing;

pub use snarkos_node_sync_communication_service as communication_service;
pub use snarkos_node_sync_ledger_service as ledger_service;
pub use snarkos_node_sync_locators as locators;

mod block_sync;
pub use block_sync::*;

mod helpers;
pub use helpers::*;

// use snarkos_node_sync_communication_service::CommunicationService;
// use snarkvm::console::network::Network;

// /// The synchronization module for the node.
// pub struct Sync<N: Network> {
//     /// The block sync module.
//     block_sync: BlockSync<N>,
// }
//
// impl<N: Network> Sync<N> {
//     /// Initializes a new instance of the sync module.
//     pub fn new(block_sync: BlockSync<N>) -> Self {
//         Self { block_sync }
//     }
// }
