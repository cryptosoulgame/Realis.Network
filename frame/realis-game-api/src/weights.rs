// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for realis_game_api
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-29, STEPS: `[20, ]`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/realis
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// realis-game-api
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --raw
// --output=./frame/realis-game-api/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for realis_game_api.
pub trait WeightInfo {
    fn mint_basic_nft(b: u32, ) -> Weight;
}

/// Weights for realis_game_api using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
    impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
            fn mint_basic_nft(_b: u32, ) -> Weight {
            (76_453_000 as Weight)
                .saturating_add(T::DbWeight::get().reads(4 as Weight))
                .saturating_add(T::DbWeight::get().writes(3 as Weight))
            }
        }

        // For backwards compatibility and tests
        impl WeightInfo for () {
            fn mint_basic_nft(_b: u32, ) -> Weight {
            (76_453_000 as Weight)
                .saturating_add(RocksDbWeight::get().reads(4 as Weight))
                .saturating_add(RocksDbWeight::get().writes(3 as Weight))
            }
        }