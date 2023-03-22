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

//! Autogenerated weights for pallet_multisig
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-ul9xcbg-project-145-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/multisig/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_multisig.
pub trait WeightInfo {
	fn as_multi_threshold_1(z: u32, ) -> Weight;
	fn as_multi_create(s: u32, z: u32, ) -> Weight;
	fn as_multi_approve(s: u32, z: u32, ) -> Weight;
	fn as_multi_complete(s: u32, z: u32, ) -> Weight;
	fn approve_as_multi_create(s: u32, ) -> Weight;
	fn approve_as_multi_approve(s: u32, ) -> Weight;
	fn cancel_as_multi(s: u32, ) -> Weight;
}

/// Weights for pallet_multisig using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_640_000 picoseconds.
		Weight::from_parts(16_707_648, 0)
			// Standard Error: 16
			.saturating_add(Weight::from_parts(695, 0).saturating_mul(z.into()))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `301 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 50_474_000 picoseconds.
		Weight::from_parts(41_695_789, 6811)
			// Standard Error: 2_105
			.saturating_add(Weight::from_parts(119_771, 0).saturating_mul(s.into()))
			// Standard Error: 20
			.saturating_add(Weight::from_parts(1_525, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `320`
		//  Estimated: `6811`
		// Minimum execution time: 32_628_000 picoseconds.
		Weight::from_parts(22_872_311, 6811)
			// Standard Error: 1_936
			.saturating_add(Weight::from_parts(122_676, 0).saturating_mul(s.into()))
			// Standard Error: 18
			.saturating_add(Weight::from_parts(1_579, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `426 + s * (33 ±0)`
		//  Estimated: `10404`
		// Minimum execution time: 56_203_000 picoseconds.
		Weight::from_parts(43_488_752, 10404)
			// Standard Error: 3_432
			.saturating_add(Weight::from_parts(169_985, 0).saturating_mul(s.into()))
			// Standard Error: 33
			.saturating_add(Weight::from_parts(1_704, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `301 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 36_671_000 picoseconds.
		Weight::from_parts(38_445_953, 6811)
			// Standard Error: 2_208
			.saturating_add(Weight::from_parts(133_828, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `320`
		//  Estimated: `6811`
		// Minimum execution time: 20_284_000 picoseconds.
		Weight::from_parts(21_523_740, 6811)
			// Standard Error: 1_327
			.saturating_add(Weight::from_parts(113_915, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `492 + s * (1 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 37_810_000 picoseconds.
		Weight::from_parts(40_005_118, 6811)
			// Standard Error: 1_778
			.saturating_add(Weight::from_parts(128_519, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_640_000 picoseconds.
		Weight::from_parts(16_707_648, 0)
			// Standard Error: 16
			.saturating_add(Weight::from_parts(695, 0).saturating_mul(z.into()))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `301 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 50_474_000 picoseconds.
		Weight::from_parts(41_695_789, 6811)
			// Standard Error: 2_105
			.saturating_add(Weight::from_parts(119_771, 0).saturating_mul(s.into()))
			// Standard Error: 20
			.saturating_add(Weight::from_parts(1_525, 0).saturating_mul(z.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `320`
		//  Estimated: `6811`
		// Minimum execution time: 32_628_000 picoseconds.
		Weight::from_parts(22_872_311, 6811)
			// Standard Error: 1_936
			.saturating_add(Weight::from_parts(122_676, 0).saturating_mul(s.into()))
			// Standard Error: 18
			.saturating_add(Weight::from_parts(1_579, 0).saturating_mul(z.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `426 + s * (33 ±0)`
		//  Estimated: `10404`
		// Minimum execution time: 56_203_000 picoseconds.
		Weight::from_parts(43_488_752, 10404)
			// Standard Error: 3_432
			.saturating_add(Weight::from_parts(169_985, 0).saturating_mul(s.into()))
			// Standard Error: 33
			.saturating_add(Weight::from_parts(1_704, 0).saturating_mul(z.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `301 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 36_671_000 picoseconds.
		Weight::from_parts(38_445_953, 6811)
			// Standard Error: 2_208
			.saturating_add(Weight::from_parts(133_828, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `320`
		//  Estimated: `6811`
		// Minimum execution time: 20_284_000 picoseconds.
		Weight::from_parts(21_523_740, 6811)
			// Standard Error: 1_327
			.saturating_add(Weight::from_parts(113_915, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `492 + s * (1 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 37_810_000 picoseconds.
		Weight::from_parts(40_005_118, 6811)
			// Standard Error: 1_778
			.saturating_add(Weight::from_parts(128_519, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
