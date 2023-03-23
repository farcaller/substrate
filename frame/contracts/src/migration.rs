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

use crate::{CodeHash, Config, Error, GasMeter, MigrationInProgress, Pallet, Weight, LOG_TARGET};
use codec::{Codec, Decode, Encode};
use frame_support::{
	codec,
	pallet_prelude::*,
	storage_alias,
	traits::{ConstU32, Get, OnRuntimeUpgrade},
	Identity,
};
use sp_std::{marker::PhantomData, mem, prelude::*};

type Migrations<T> = (V9<T>, V10<T>);
type State = BoundedVec<u8, ConstU32<1024>>;

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Progress {
	InProgress { state: State },
	Failed { state_before: State },
	FailedOnInit,
}

struct WeightedResult<T> {
	weight: Weight,
	result: Result<T, &'static str>,
}

enum IsFinished {
	Yes,
	No,
}

trait Migrate<T: Config>: Codec + MaxEncodedLen {
	const VERSION: u16;

	fn init() -> WeightedResult<Self>;

	// TODO: replace &mut Weight with a weight meter
	fn step(&mut self, remaining_weight: &mut GasMeter<T>) -> Result<IsFinished, &'static str>;
}

trait MigrateSequence<T: Config> {
	const VERSION_RANGE: Option<(u16, u16)>;

	fn init(version: StorageVersion) -> WeightedResult<Progress>;

	fn step(
		version: StorageVersion,
		remaining_weight: &mut GasMeter<T>,
		state: &[u8],
	) -> Result<Option<State>, &'static str>;

	fn integrity_test();

	fn is_upgrade_supported(in_storage: StorageVersion, target: StorageVersion) -> bool {
		if in_storage == target {
			return true
		}
		if in_storage > target {
			return false
		}
		let Some((low, high)) = Self::VERSION_RANGE else {
			return false
		};
		let Some(first_supported) = low.checked_sub(1) else {
			return false
		};
		in_storage == first_supported && target == high
	}
}

/// Performs all necessary migrations based on `StorageVersion`.
pub struct Migration<T: Config>(PhantomData<T>);

impl<T: Config> OnRuntimeUpgrade for Migration<T> {
	fn on_runtime_upgrade() -> Weight {
		// TODO: If `try-runtime` we just run all migrations inside a single block. Otherwise
		// testing a stack of migrations would entail running the chain for a while.

		let latest_version = <Pallet<T>>::current_storage_version();
		let storage_version = <Pallet<T>>::on_chain_storage_version();
		let mut weight = T::DbWeight::get().reads(1);

		if storage_version == latest_version {
			return weight
		}

		// In case a migration is already in progress we initialize the next migration
		// (if any) right when the current one finishes.
		weight.saturating_accrue(T::DbWeight::get().reads(1));
		if in_progress::<T>() {
			return weight
		}

		let outcome = Migrations::<T>::init(storage_version + 1);
		weight.saturating_accrue(outcome.weight);
		let progress = match outcome.result {
			// TODO: emit event that migration has started
			Ok(migration) => migration,
			Err(msg) => {
				log::error!(target: LOG_TARGET, "Failed to init migration: {}", msg);
				Progress::FailedOnInit
			},
		};
		MigrationInProgress::<T>::set(Some(progress));
		weight.saturating_accrue(T::DbWeight::get().writes(1));

		weight
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		// We can't really do much here as our migrations do not happen during the runtime upgrade.
		// Instead, we call the migrations `pre_upgrade` and `post_upgrade` hooks when we iterate
		// over our migrations.
		let storage_version = <Pallet<T>>::on_chain_storage_version();
		let target_version = <Pallet<T>>::current_storage_version();
		if Migrations::<T>::is_upgrade_supported(storage_version, target_version) {
			Ok(Vec::new())
		} else {
			Err("New runtime does not contain the required migrations to perform this upgrade.")
		}
	}
}

pub fn integrity_test<T: Config>() {
	Migrations::<T>::integrity_test()
}

pub fn migrate<T: Config>(remaining_weight: &mut GasMeter<T>) -> Result<(), DispatchError> {
	// TODO: emit event when migration is done
	// TODO: call pre and post hooks if migration when try_runtime
	// TODO: update in storage version and trigger next migration if the current one is done

	MigrationInProgress::<T>::try_mutate_exists(|progress| {
		let mut state_before = progress
			.as_mut()
			.and_then(|progress| match progress {
				Progress::InProgress { state } => Some(state),
				_ => None,
			})
			.ok_or_else(|| Error::<T>::NoMigrationInProgress)?;

		// if a migration is running it is always upgrading to the next version
		let storage_version = <Pallet<T>>::on_chain_storage_version();
		let in_progress_version = storage_version + 1;

		*progress = match Migrations::<T>::step(
			in_progress_version,
			remaining_weight,
			state_before.as_ref(),
		) {
			Ok(Some(state)) => Some(Progress::InProgress { state }),
			Ok(None) => None,
			Err(err) => {
				log::error!(
					target: "LOG_TARGET", "Migration failed while migrating to {:?}: {}",
					in_progress_version, err,
				);
				Some(Progress::Failed { state_before: mem::take(&mut state_before) })
			},
		};
		Ok(())
	})
}

fn in_progress<T: Config>() -> bool {
	MigrationInProgress::<T>::exists()
}

pub fn ensure_migrated<T: Config>() -> DispatchResult {
	if in_progress::<T>() {
		Err(Error::<T>::MigrationInProgress.into())
	} else {
		Ok(())
	}
}

#[impl_trait_for_tuples::impl_for_tuples(10)]
#[tuple_types_custom_trait_bound(Migrate<T>)]
impl<T: Config> MigrateSequence<T> for Tuple {
	const VERSION_RANGE: Option<(u16, u16)> = {
		let mut versions: Option<(u16, u16)> = None;
		for_tuples!(
			#(
				match versions {
					None => {
						versions = Some((Tuple::VERSION, Tuple::VERSION));
					},
					Some((min_version, last_version)) if Tuple::VERSION == last_version + 1 => {
						versions = Some((min_version, Tuple::VERSION));
					},
					_ => panic!("Migrations must be ordered by their versions with no gaps.")
				}
			)*
		);
		versions
	};

	fn init(version: StorageVersion) -> WeightedResult<Progress> {
		for_tuples!(
			#(
				if version == Tuple::VERSION {
					let outcome = Tuple::init();
					return WeightedResult {
						weight: outcome.weight,
						result: outcome.result.and_then(|state| {
							Ok(Progress::InProgress {
								state: state.encode().try_into().map_err(|_| "Migration state too big.")?,
							})
						})
					}
				}
			)*
		);
		WeightedResult {
			weight: Weight::zero(),
			result: Err("Migration not supported by this runtime."),
		}
	}

	fn step(
		version: StorageVersion,
		remaining_weight: &mut GasMeter<T>,
		mut state: &[u8],
	) -> Result<Option<State>, &'static str> {
		for_tuples!(
			#(
				if version == Tuple::VERSION {
					let mut migration = <Tuple as Decode>::decode(&mut state)
						.map_err(|_| "Can't decode migration state")?;
					let is_finished = matches!(migration.step(remaining_weight)?, IsFinished::Yes);
					return Ok(if is_finished  {
						None
					} else {
						Some(
							migration
							.encode()
							.try_into()
							.map_err(|_| "Migration state too big.")?
						)
					})
				}
			)*
		);
		Err("Migration not supported by this runtime.")
	}

	fn integrity_test() {
		for_tuples!(
			#(
				let len = <Tuple as MaxEncodedLen>::max_encoded_len();
				let max = State::bound();
				if len > max {
					let version = Tuple::VERSION;
					panic!(
						"Migration {} has size {} which is bigger than the maximum of {}",
						version, len, max,
					);
				}
			)*
		);
	}
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct V9<T: Config>(PhantomData<T>);

impl<T: Config> Migrate<T> for V9<T> {
	const VERSION: u16 = 9;

	fn init() -> WeightedResult<Self> {
		unimplemented!()
	}

	fn step(&mut self, _remaining_weight: &mut GasMeter<T>) -> Result<IsFinished, &'static str> {
		unimplemented!()
	}
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct V10<T: Config>(PhantomData<T>);

impl<T: Config> Migrate<T> for V10<T> {
	const VERSION: u16 = 10;

	fn init() -> WeightedResult<Self> {
		unimplemented!()
	}

	fn step(&mut self, _remaining_weight: &mut GasMeter<T>) -> Result<IsFinished, &'static str> {
		unimplemented!()
	}
}

/// Update `CodeStorage` with the new `determinism` field.
mod v9 {
	use super::*;
	use crate::Determinism;

	#[derive(Encode, Decode)]
	struct OldPrefabWasmModule {
		#[codec(compact)]
		instruction_weights_version: u32,
		#[codec(compact)]
		initial: u32,
		#[codec(compact)]
		maximum: u32,
		#[codec(compact)]
		refcount: u64,
		_reserved: Option<()>,
		code: Vec<u8>,
		original_code_len: u32,
	}

	#[derive(Encode, Decode)]
	pub struct PrefabWasmModule {
		#[codec(compact)]
		pub instruction_weights_version: u32,
		#[codec(compact)]
		pub initial: u32,
		#[codec(compact)]
		pub maximum: u32,
		pub code: Vec<u8>,
		pub determinism: Determinism,
	}

	#[storage_alias]
	type CodeStorage<T: Config> = StorageMap<Pallet<T>, Identity, CodeHash<T>, PrefabWasmModule>;

	#[allow(dead_code)]
	pub fn migrate<T: Config>(weight: &mut Weight) {
		<CodeStorage<T>>::translate_values(|old: OldPrefabWasmModule| {
			weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 1));
			Some(PrefabWasmModule {
				instruction_weights_version: old.instruction_weights_version,
				initial: old.initial,
				maximum: old.maximum,
				code: old.code,
				determinism: Determinism::Enforced,
			})
		});
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn check_versions() {
		// this fails the compilation when running local tests
		// otherwise it will only be evaluated when the whole runtime is build
		let _ = Migrations::VERSION_RANGE;
	}
}
