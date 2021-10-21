// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Proof utilities
use crate::StorageProof;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_storage::ChildInfo;

/// Interface for providing block proving utilities.
pub trait ProofProvider<Block: BlockT> {
	/// Reads storage value at a given block + key, returning read proof.
	fn read_proof(
		&self,
		id: &BlockId<Block>,
		keys: &mut dyn Iterator<Item = &[u8]>,
	) -> sp_blockchain::Result<StorageProof>;

	/// Reads child storage value at a given block + storage_key + key, returning
	/// read proof.
	fn read_child_proof(
		&self,
		id: &BlockId<Block>,
		child_info: &ChildInfo,
		keys: &mut dyn Iterator<Item = &[u8]>,
	) -> sp_blockchain::Result<StorageProof>;

	/// Execute a call to a contract on top of state in a block of given hash
	/// AND returning execution proof.
	///
	/// No changes are made.
	fn execution_proof(
		&self,
		id: &BlockId<Block>,
		method: &str,
		call_data: &[u8],
	) -> sp_blockchain::Result<(Vec<u8>, StorageProof)>;

	/// Given a `BlockId` iterate over all storage values starting at `start_key` exclusively,
	/// building proofs until size limit is reached. Returns combined proof and the number of
	/// collected keys.
	fn read_proof_collection(
		&self,
		id: &BlockId<Block>,
		start_key: &[u8],
		size_limit: usize,
	) -> sp_blockchain::Result<(StorageProof, u32)>;

	/// Given a `BlockId` iterate over all storage values starting at `start_key`.
	/// Returns collected keys and values.
	fn storage_collection(
		&self,
		id: &BlockId<Block>,
		start_key: &[u8],
		size_limit: usize,
	) -> sp_blockchain::Result<Vec<(Vec<u8>, Vec<u8>)>>;

	/// Verify read storage proof for a set of keys.
	/// Returns collected key-value pairs and a flag indicating if iteration is complete.
	fn verify_range_proof(
		&self,
		root: Block::Hash,
		proof: StorageProof,
		start_key: &[u8],
	) -> sp_blockchain::Result<(Vec<(Vec<u8>, Vec<u8>)>, bool)>;
}
