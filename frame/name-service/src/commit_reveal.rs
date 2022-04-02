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

//! Handles the commit and reveal part of the registration process.

use crate::{types::*, *};
use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement, OnUnbalanced, ReservableCurrency, WithdrawReasons},
};
use sp_runtime::traits::{Saturating, Zero};
use sp_std::prelude::*;

impl<T: Config> Pallet<T> {
	/// Get the commitment hash from the raw name and secret.
	pub fn commitment_hash(name: &[u8], secret: u64) -> CommitmentHash {
		sp_io::hashing::blake2_256(&(name, secret).encode())
	}

	/// Get the name hash from raw bytes.
	pub fn name_hash(name: &[u8]) -> NameHash {
		sp_io::hashing::blake2_256(name)
	}

	/// Returns a commitment by hash if it exists.
	pub fn get_commitment(
		commitment_hash: CommitmentHash,
	) -> Result<CommitmentOf<T>, DispatchError> {
		Commitments::<T>::get(commitment_hash).ok_or(Error::<T>::CommitmentNotFound.into())
	}

	/// Checks whether a commitment has passed the minimum commitment period.
	pub fn is_commitment_valid(
		commitment: &CommitmentOf<T>,
		block_number: &T::BlockNumber,
	) -> bool {
		&commitment.when.saturating_add(T::MinCommitmentAge::get()) < block_number
	}

	/// Checks whether a commitment has passed the commitment expiry time.
	pub fn is_commitment_expired(
		commitment: &CommitmentOf<T>,
		block_number: &T::BlockNumber,
	) -> bool {
		&commitment.when.saturating_add(T::MaxCommitmentAge::get()) < block_number
	}

	pub fn do_commit(
		depositor: T::AccountId,
		owner: T::AccountId,
		commitment_hash: CommitmentHash,
	) -> DispatchResult {
		ensure!(!Commitments::<T>::contains_key(commitment_hash), Error::<T>::CommitmentExists);

		let block_number = frame_system::Pallet::<T>::block_number();
		let deposit = T::CommitmentDeposit::get();

		T::Currency::reserve(&depositor, deposit)?;

		let commitment = Commitment {
			owner: owner.clone(),
			when: block_number,
			depositor: depositor.clone(),
			deposit,
		};

		Commitments::<T>::insert(commitment_hash, commitment);
		Self::deposit_event(Event::<T>::Committed { depositor, owner, hash: commitment_hash });
		Ok(())
	}

	pub fn do_reveal(
		fee_payer: T::AccountId,
		name: Vec<u8>,
		secret: u64,
		length: T::BlockNumber,
	) -> DispatchResult {
		ensure!(name.len() <= T::MaxNameLength::get() as usize, Error::<T>::NameTooLong);

		let commitment_hash = Self::commitment_hash(&name, secret);
		let commitment = Self::get_commitment(commitment_hash)?;

		let block_number = frame_system::Pallet::<T>::block_number();

		ensure!(
			Self::is_commitment_valid(&commitment, &block_number),
			Error::<T>::TooEarlyToReveal
		);

		let name_hash = sp_io::hashing::blake2_256(&name);

		ensure!(Self::get_registration(name_hash).is_err(), Error::<T>::RegistrationExists);

		let fee = Self::registration_fee(name.clone(), length);

		let imbalance = T::Currency::withdraw(
			&fee_payer,
			fee,
			WithdrawReasons::FEE,
			ExistenceRequirement::KeepAlive,
		)?;

		T::RegistrationFeeHandler::on_unbalanced(imbalance);

		let expiry = block_number.saturating_add(length);

		Self::do_register(
			name_hash,
			commitment.owner.clone(),
			commitment.owner.clone(),
			Some(expiry),
			None,
		)?;

		Self::do_remove_commitment(&commitment_hash, &commitment);
		Ok(())
	}

	pub fn do_renew(
		fee_payer: T::AccountId,
		name_hash: NameHash,
		expiry: T::BlockNumber,
	) -> DispatchResult {
		Registrations::<T>::try_mutate(name_hash, |maybe_registration| {
			let r = maybe_registration.as_mut().ok_or(Error::<T>::RegistrationNotFound)?;

			// cannot renew a domain that has no expiry
			let current_expiry = r.expiry.ok_or(Error::<T>::RegistrationHasNoExpiry)?;

			let block_number = frame_system::Pallet::<T>::block_number();

			// `expiry` must be at least 1 block in the future
			ensure!(expiry > block_number, Error::<T>::ExpiryInvalid);

			// `expiry` must be larger than `current_expiry`
			ensure!(expiry > current_expiry, Error::<T>::ExpiryInvalid);

			// calculate additional length to determine fee to be paid
			let length = if block_number <= current_expiry {
				// if we are renewing withn or at the current expiry. Calculate difference from
				// current expiry. Equivalent to extending the current expiry.

				expiry.saturating_sub(current_expiry)
			} else {
				// if we are renewing after current expiry (registration has expired), calculate
				// difference from current block. Equivalent to extending from the current block to
				// `expiry`.

				expiry.saturating_sub(block_number)
			};

			// determine renew fee
			let fee = Self::length_fee(length);

			// withdraw fees from account
			let imbalance = T::Currency::withdraw(
				&fee_payer,
				fee,
				WithdrawReasons::FEE,
				ExistenceRequirement::KeepAlive,
			)?;

			r.expiry = Some(expiry);

			T::RegistrationFeeHandler::on_unbalanced(imbalance);
			Self::deposit_event(Event::<T>::NameRenewed { name_hash, expires: expiry });
			Ok(())
		})
	}

	/// Remove an existing commitment without any checks.
	///
	/// Unreserves any deposit held for the commitment.
	pub fn do_remove_commitment(commitment_hash: &CommitmentHash, commitment: &CommitmentOf<T>) {
		let res = T::Currency::unreserve(&commitment.depositor, commitment.deposit);
		debug_assert!(res.is_zero());
		Commitments::<T>::remove(commitment_hash);
	}
}
