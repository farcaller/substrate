// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
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

use crate::{cache::TrieCache, Error, NodeCodec, StorageProof, TrieDBBuilder};
use codec::Encode;
use hash_db::{HashDBRef, Hasher};
use parking_lot::Mutex;
use std::{collections::HashMap, mem, ops::DerefMut, sync::Arc};
use trie_db::{DBValue, KeyTrieAccessValue, TrieAccess, TrieLayout};

/// Combines information about an accessed key.
struct AccessedKey {
	/// Did we already recorded all the trie nodes for accessing this key?
	trie_nodes_recorded: bool,
	/// Exists the value for the key in the trie?
	exists: bool,
}

/// The internals of [`Recorder`].
struct RecorderInner<H> {
	/// If we are recording while a cache is enabled, we may don't access all nodes because some
	/// data is already cached. Thus, we will only be informed about the key that was accessed. We
	/// will use these keys when building the [`StorageProof`] to traverse the trie again and
	/// collecting the required trie nodes for accessing the data.
	accessed_keys: HashMap<H, HashMap<Vec<u8>, AccessedKey>>,
	/// The encoded nodes we accessed while recording.
	accessed_nodes: HashMap<H, Vec<u8>>,
	encoded_size_estimation: usize,
}

impl<H> Default for RecorderInner<H> {
	fn default() -> Self {
		Self {
			accessed_keys: HashMap::new(),
			accessed_nodes: HashMap::new(),
			encoded_size_estimation: 0,
		}
	}
}

pub struct Recorder<H: Hasher> {
	inner: Arc<Mutex<RecorderInner<H::Out>>>,
}

impl<H: Hasher> Default for Recorder<H> {
	fn default() -> Self {
		Self { inner: Default::default() }
	}
}

impl<H: Hasher> Clone for Recorder<H> {
	fn clone(&self) -> Self {
		Self { inner: self.inner.clone() }
	}
}

impl<H: Hasher> Recorder<H> {
	/// Returns the recorder as [`TrieRecorder`](trie_db::TrieRecorder) compatible type.
	///
	/// The given `storage_root` is the storage root of trie for that the access is being recorded.
	pub fn as_trie_recorder(
		&self,
		storage_root: H::Out,
	) -> impl trie_db::TrieRecorder<H::Out> + '_ {
		TrieRecorder::<H, _> { inner: self.inner.lock(), storage_root }
	}

	pub fn into_storage_proof<L: TrieLayout<Hash = H, Codec = NodeCodec<H>>>(
		self,
		root: &H::Out,
		hash_db: &dyn HashDBRef<H, DBValue>,
		mut cache: Option<&mut TrieCache<H>>,
	) -> Result<StorageProof, crate::Error<H::Out>> {
		let mut recorder = mem::take(&mut *self.inner.lock());
		let accessed_keys = mem::take(&mut recorder.accessed_keys);
		let mut trie_recorder = TrieRecorder::<H, _> { inner: &mut recorder, storage_root: *root };

		accessed_keys.into_iter().try_for_each(|(root, keys)| {
			let trie = TrieDBBuilder::<L>::new(hash_db, &root)
				.with_recorder(&mut trie_recorder)
				.with_optional_cache(cache.as_mut().map(|c| *c as _))
				.build();

			// For all keys we don't have recorded the trie nodes, we need to traverse
			// the trie to record all required nodes.
			keys.into_iter().filter(|(_, v)| !v.trie_nodes_recorded).try_for_each(|(k, v)| {
				if trie.traverse_to(&k)? != v.exists {
					Err(Error::InvalidRecording(k, v.exists))
				} else {
					Ok(())
				}
			})
		})?;

		Ok(StorageProof::new(recorder.accessed_nodes.drain().map(|(_, v)| v)))
	}

	/// Returns the estimated encoded size of the proof.
	///
	/// The estimation is based on all the nodes that were accessed until now while
	/// accessing the trie. When it comes to the [`TrieAccess::Key`] the estimation
	/// gets inaccurate because we may not have recorded all the required trie nodes
	/// yet.
	pub fn estimate_encoded_size(&self) -> usize {
		self.inner.lock().encoded_size_estimation
	}
}

struct TrieRecorder<H: Hasher, I> {
	inner: I,
	storage_root: H::Out,
}

impl<H: Hasher, I: DerefMut<Target = RecorderInner<H::Out>>> trie_db::TrieRecorder<H::Out>
	for TrieRecorder<H, I>
{
	fn record<'b>(&mut self, access: TrieAccess<'b, H::Out>) {
		let mut encoded_size_update = 0;

		match access {
			TrieAccess::Key { key, value } => {
				self.inner
					.accessed_keys
					.entry(self.storage_root)
					.or_default()
					.entry(key.into())
					// If the value is served from the data cache, we need to ensure
					// that we traverse the trie when building the proof to record this
					// data.
					.or_insert_with(|| {
						match value {
							KeyTrieAccessValue::HashOnly => {
								// We don't know the number of nodes we need to reach this hash.
								// So, we only track the size of the hash..
								encoded_size_update += H::LENGTH;
							},
							KeyTrieAccessValue::NonExisting => {},
							KeyTrieAccessValue::Existing(ref value) => {
								// We don't know the number of nodes we need to reach this
								// value and we also don't know if we may already have recorded
								// some of these nodes. So, we only take into account the encoded
								// size of the value + length of a hash in the trie
								// (ignoring that the value may is inlined).
								encoded_size_update += value.encoded_size() + H::LENGTH;
							},
						}

						AccessedKey { trie_nodes_recorded: false, exists: value.exists() }
					});
			},
			TrieAccess::NodeOwned { hash, node_owned } => {
				self.inner.accessed_nodes.entry(hash).or_insert_with(|| {
					let node = node_owned.to_encoded::<NodeCodec<H>>();

					encoded_size_update += node.encoded_size();

					node
				});
			},
			TrieAccess::EncodedNode { hash, encoded_node } => {
				self.inner.accessed_nodes.entry(hash).or_insert_with(|| {
					let node = encoded_node.into_owned();

					encoded_size_update += node.encoded_size();

					node
				});
			},
			TrieAccess::Value { hash, value, full_key } => {
				self.inner.accessed_nodes.entry(hash).or_insert_with(|| {
					let value = value.into_owned();

					encoded_size_update += value.encoded_size();

					value
				});

				self.inner
					.accessed_keys
					.entry(self.storage_root)
					.or_default()
					.entry(full_key.into())
					// Insert the full key into the accessed keys map, but inform
					// us that we already have recorded all the trie nodes for this.
					// This prevents that we need to traverse the trie again when we
					// are building the proof for this key.
					.or_insert_with(|| AccessedKey { trie_nodes_recorded: true, exists: true });
			},
		};

		self.inner.encoded_size_estimation += encoded_size_update;
	}
}

#[cfg(test)]
mod tests {
	use trie_db::{Trie, TrieDBBuilder, TrieDBMutBuilder, TrieHash, TrieMut};

	type MemoryDB = crate::MemoryDB<sp_core::Blake2Hasher>;
	type Layout = crate::LayoutV1<sp_core::Blake2Hasher>;
	type Recorder = super::Recorder<sp_core::Blake2Hasher>;

	const TEST_DATA: &[(&[u8], &[u8])] =
		&[(b"key1", b"val1"), (b"key2", b"val2"), (b"key3", b"val3"), (b"key4", b"val4")];

	fn create_trie() -> (MemoryDB, TrieHash<Layout>) {
		let mut db = MemoryDB::default();
		let mut root = Default::default();

		{
			let mut trie = TrieDBMutBuilder::<Layout>::new(&mut db, &mut root).build();
			for (k, v) in TEST_DATA {
				trie.insert(k, v).expect("Inserts data");
			}
		}

		(db, root)
	}

	#[test]
	fn recorder_works() {
		let (db, root) = create_trie();

		let recorder = Recorder::default();

		{
			let mut trie_recorder = recorder.as_trie_recorder(root);
			let trie = TrieDBBuilder::<Layout>::new(&db, &root)
				.with_recorder(&mut trie_recorder)
				.build();
			assert_eq!(TEST_DATA[0].1.to_vec(), trie.get(TEST_DATA[0].0).unwrap().unwrap());
		}

		let storage_proof = recorder.into_storage_proof::<Layout>(&root, &db, None).unwrap();
		let memory_db: MemoryDB = storage_proof.into_memory_db();

		// Check that we recorded the required data
		let trie = TrieDBBuilder::<Layout>::new(&memory_db, &root).build();
		assert_eq!(TEST_DATA[0].1.to_vec(), trie.get(TEST_DATA[0].0).unwrap().unwrap());
	}
}
