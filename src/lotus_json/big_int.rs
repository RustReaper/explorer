// Copyright 2019-2024 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use super::*;

use fvm_shared::bigint::BigInt;

#[derive(Clone, Serialize, Deserialize)]
pub struct BigIntLotusJson(#[serde(with = "crate::lotus_json::stringify")] BigInt);

impl HasLotusJson for BigInt {
    type LotusJson = BigIntLotusJson;

    fn into_lotus_json(self) -> Self::LotusJson {
        BigIntLotusJson(self)
    }

    fn from_lotus_json(BigIntLotusJson(big_int): Self::LotusJson) -> Self {
        big_int
    }
}
