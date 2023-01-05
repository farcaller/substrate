// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_state_trie_migration
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm2`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_state_trie_migration
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/state-trie-migration/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_state_trie_migration.
pub trait WeightInfo {
	fn continue_migrate() -> Weight;
	fn continue_migrate_wrong_witness() -> Weight;
	fn migrate_custom_top_success() -> Weight;
	fn migrate_custom_top_fail() -> Weight;
	fn migrate_custom_child_success() -> Weight;
	fn migrate_custom_child_fail() -> Weight;
	fn process_top_key(v: u32, ) -> Weight;
}

/// Weights for pallet_state_trie_migration using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: StateTrieMigration SignedMigrationMaxLimits (r:1 w:0)
	/// Proof: StateTrieMigration SignedMigrationMaxLimits (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: StateTrieMigration MigrationProcess (r:1 w:1)
	/// Proof: StateTrieMigration MigrationProcess (max_values: Some(1), max_size: Some(1054), added: 1549, mode: MaxEncodedLen)
	fn continue_migrate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `108`
		//  Estimated: `2052`
		// Minimum execution time: 15_245 nanoseconds.
		Weight::from_parts(15_786_000, 2052)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: StateTrieMigration SignedMigrationMaxLimits (r:1 w:0)
	/// Proof: StateTrieMigration SignedMigrationMaxLimits (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn continue_migrate_wrong_witness() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `503`
		// Minimum execution time: 4_240 nanoseconds.
		Weight::from_parts(4_590_000, 503)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	fn migrate_custom_top_success() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_632 nanoseconds.
		Weight::from_ref_time(8_817_000)
	}
	/// Storage: unknown `0x666f6f` (r:1 w:1)
	/// Proof Skipped: unknown `0x666f6f` (r:1 w:1)
	fn migrate_custom_top_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `2619`
		// Minimum execution time: 23_983 nanoseconds.
		Weight::from_parts(24_470_000, 2619)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn migrate_custom_child_success() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_839 nanoseconds.
		Weight::from_ref_time(9_142_000)
	}
	/// Storage: unknown `0x666f6f` (r:1 w:1)
	/// Proof Skipped: unknown `0x666f6f` (r:1 w:1)
	fn migrate_custom_child_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `2611`
		// Minimum execution time: 24_260 nanoseconds.
		Weight::from_parts(25_245_000, 2611)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: unknown `0x6b6579` (r:1 w:1)
	/// Proof Skipped: unknown `0x6b6579` (r:1 w:1)
	/// The range of component `v` is `[1, 4194304]`.
	fn process_top_key(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `228 + v * (1 ±0)`
		//  Estimated: `2700 + v * (1 ±0)`
		// Minimum execution time: 5_225 nanoseconds.
		Weight::from_parts(5_369_000, 2700)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(1_124).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_proof_size(1).saturating_mul(v.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: StateTrieMigration SignedMigrationMaxLimits (r:1 w:0)
	/// Proof: StateTrieMigration SignedMigrationMaxLimits (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: StateTrieMigration MigrationProcess (r:1 w:1)
	/// Proof: StateTrieMigration MigrationProcess (max_values: Some(1), max_size: Some(1054), added: 1549, mode: MaxEncodedLen)
	fn continue_migrate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `108`
		//  Estimated: `2052`
		// Minimum execution time: 15_245 nanoseconds.
		Weight::from_parts(15_786_000, 2052)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: StateTrieMigration SignedMigrationMaxLimits (r:1 w:0)
	/// Proof: StateTrieMigration SignedMigrationMaxLimits (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn continue_migrate_wrong_witness() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `503`
		// Minimum execution time: 4_240 nanoseconds.
		Weight::from_parts(4_590_000, 503)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	fn migrate_custom_top_success() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_632 nanoseconds.
		Weight::from_ref_time(8_817_000)
	}
	/// Storage: unknown `0x666f6f` (r:1 w:1)
	/// Proof Skipped: unknown `0x666f6f` (r:1 w:1)
	fn migrate_custom_top_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `2619`
		// Minimum execution time: 23_983 nanoseconds.
		Weight::from_parts(24_470_000, 2619)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn migrate_custom_child_success() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_839 nanoseconds.
		Weight::from_ref_time(9_142_000)
	}
	/// Storage: unknown `0x666f6f` (r:1 w:1)
	/// Proof Skipped: unknown `0x666f6f` (r:1 w:1)
	fn migrate_custom_child_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `2611`
		// Minimum execution time: 24_260 nanoseconds.
		Weight::from_parts(25_245_000, 2611)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: unknown `0x6b6579` (r:1 w:1)
	/// Proof Skipped: unknown `0x6b6579` (r:1 w:1)
	/// The range of component `v` is `[1, 4194304]`.
	fn process_top_key(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `228 + v * (1 ±0)`
		//  Estimated: `2700 + v * (1 ±0)`
		// Minimum execution time: 5_225 nanoseconds.
		Weight::from_parts(5_369_000, 2700)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(1_124).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_proof_size(1).saturating_mul(v.into()))
	}
}
