// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_transaction_storage
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_transaction_storage
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/transaction-storage/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_transaction_storage.
pub trait WeightInfo {
	fn store(l: u32, ) -> Weight;
	fn renew() -> Weight;
	fn check_proof_max() -> Weight;
}

/// Weights for pallet_transaction_storage using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: TransactionStorage ByteFee (r:1 w:0)
	/// Proof: TransactionStorage ByteFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: TransactionStorage EntryFee (r:1 w:0)
	/// Proof: TransactionStorage EntryFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: TransactionStorage BlockTransactions (r:1 w:1)
	/// Proof: TransactionStorage BlockTransactions (max_values: Some(1), max_size: Some(36866), added: 37361, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 8388608]`.
	fn store(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `176`
		//  Estimated: `41353`
		// Minimum execution time: 33_286_000 picoseconds.
		Weight::from_parts(33_596_000, 41353)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(4_954, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: TransactionStorage Transactions (r:1 w:0)
	/// Proof: TransactionStorage Transactions (max_values: None, max_size: Some(36886), added: 39361, mode: MaxEncodedLen)
	/// Storage: TransactionStorage ByteFee (r:1 w:0)
	/// Proof: TransactionStorage ByteFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: TransactionStorage EntryFee (r:1 w:0)
	/// Proof: TransactionStorage EntryFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: TransactionStorage BlockTransactions (r:1 w:1)
	/// Proof: TransactionStorage BlockTransactions (max_values: Some(1), max_size: Some(36866), added: 37361, mode: MaxEncodedLen)
	fn renew() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `326`
		//  Estimated: `81704`
		// Minimum execution time: 41_892_000 picoseconds.
		Weight::from_parts(42_802_000, 81704)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: TransactionStorage ProofChecked (r:1 w:1)
	/// Proof: TransactionStorage ProofChecked (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: TransactionStorage StoragePeriod (r:1 w:0)
	/// Proof: TransactionStorage StoragePeriod (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: TransactionStorage ChunkCount (r:1 w:0)
	/// Proof: TransactionStorage ChunkCount (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: System ParentHash (r:1 w:0)
	/// Proof: System ParentHash (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TransactionStorage Transactions (r:1 w:0)
	/// Proof: TransactionStorage Transactions (max_values: None, max_size: Some(36886), added: 39361, mode: MaxEncodedLen)
	fn check_proof_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37145`
		//  Estimated: `48332`
		// Minimum execution time: 63_799_000 picoseconds.
		Weight::from_parts(66_145_000, 48332)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: TransactionStorage ByteFee (r:1 w:0)
	/// Proof: TransactionStorage ByteFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: TransactionStorage EntryFee (r:1 w:0)
	/// Proof: TransactionStorage EntryFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: TransactionStorage BlockTransactions (r:1 w:1)
	/// Proof: TransactionStorage BlockTransactions (max_values: Some(1), max_size: Some(36866), added: 37361, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 8388608]`.
	fn store(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `176`
		//  Estimated: `41353`
		// Minimum execution time: 33_286_000 picoseconds.
		Weight::from_parts(33_596_000, 41353)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(4_954, 0).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: TransactionStorage Transactions (r:1 w:0)
	/// Proof: TransactionStorage Transactions (max_values: None, max_size: Some(36886), added: 39361, mode: MaxEncodedLen)
	/// Storage: TransactionStorage ByteFee (r:1 w:0)
	/// Proof: TransactionStorage ByteFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: TransactionStorage EntryFee (r:1 w:0)
	/// Proof: TransactionStorage EntryFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: TransactionStorage BlockTransactions (r:1 w:1)
	/// Proof: TransactionStorage BlockTransactions (max_values: Some(1), max_size: Some(36866), added: 37361, mode: MaxEncodedLen)
	fn renew() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `326`
		//  Estimated: `81704`
		// Minimum execution time: 41_892_000 picoseconds.
		Weight::from_parts(42_802_000, 81704)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: TransactionStorage ProofChecked (r:1 w:1)
	/// Proof: TransactionStorage ProofChecked (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: TransactionStorage StoragePeriod (r:1 w:0)
	/// Proof: TransactionStorage StoragePeriod (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: TransactionStorage ChunkCount (r:1 w:0)
	/// Proof: TransactionStorage ChunkCount (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: System ParentHash (r:1 w:0)
	/// Proof: System ParentHash (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: TransactionStorage Transactions (r:1 w:0)
	/// Proof: TransactionStorage Transactions (max_values: None, max_size: Some(36886), added: 39361, mode: MaxEncodedLen)
	fn check_proof_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37145`
		//  Estimated: `48332`
		// Minimum execution time: 63_799_000 picoseconds.
		Weight::from_parts(66_145_000, 48332)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
