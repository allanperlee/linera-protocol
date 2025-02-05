// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod mock_application;

pub use self::mock_application::{ExpectedCall, MockApplication};
use linera_base::{
    crypto::{BcsSignable, CryptoHash},
    data_types::BlockHeight,
    identifiers::{BytecodeId, ChainId, MessageId},
};
use linera_execution::{BytecodeLocation, UserApplicationDescription};
use serde::{Deserialize, Serialize};

pub fn create_dummy_user_application_description(index: u64) -> UserApplicationDescription {
    let chain_id = ChainId::root(1);
    let certificate_hash = CryptoHash::new(&FakeCertificate);
    UserApplicationDescription {
        bytecode_id: BytecodeId::new(MessageId {
            chain_id,
            height: BlockHeight(index),
            index: 0,
        }),
        bytecode_location: BytecodeLocation {
            certificate_hash,
            operation_index: 0,
        },
        creation: MessageId {
            chain_id,
            height: BlockHeight(index),
            index: 1,
        },
        required_application_ids: vec![],
        parameters: vec![],
    }
}

#[derive(Deserialize, Serialize)]
pub struct FakeCertificate;

impl BcsSignable for FakeCertificate {}
