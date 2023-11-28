// This file is part of OAK-blockchain.

// Copyright (C) OAK Network Ltd.
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

//! Autogenerated weights for pallet_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `actions-runner-1`, CPU: `Intel(R) Xeon(R) E-2388G CPU @ 3.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("turing-dev"), DB CACHE: 1024

// Executed Command:
// ./oak-collator
// benchmark
// pallet
// --header
// ./.maintain/HEADER-GPL3
// --chain
// turing-dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_vesting
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// ./vesting-raw-weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_vesting.
pub trait WeightInfo {
	fn vest(v: u32, ) -> Weight;
}

/// Weights for pallet_vesting using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Vesting VestingSchedule (r:1 w:1)
	/// Proof Skipped: Vesting VestingSchedule (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:20 w:20)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Vesting TotalUnvestedAllocation (r:1 w:1)
	/// Proof Skipped: Vesting TotalUnvestedAllocation (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `v` is `[0, 20]`.
	fn vest(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `286 + v * (55 ±0)`
		//  Estimated: `3735 + v * (2603 ±0)`
		// Minimum execution time: 8_566_000 picoseconds.
		Weight::from_parts(15_159_205, 3735)
			// Standard Error: 16_870
			.saturating_add(Weight::from_parts(24_811_986, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(v.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(v.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Vesting VestingSchedule (r:1 w:1)
	/// Proof Skipped: Vesting VestingSchedule (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:20 w:20)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Vesting TotalUnvestedAllocation (r:1 w:1)
	/// Proof Skipped: Vesting TotalUnvestedAllocation (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `v` is `[0, 20]`.
	fn vest(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `286 + v * (55 ±0)`
		//  Estimated: `3735 + v * (2603 ±0)`
		// Minimum execution time: 8_566_000 picoseconds.
		Weight::from_parts(15_159_205, 3735)
			// Standard Error: 16_870
			.saturating_add(Weight::from_parts(24_811_986, 0).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(v.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(v.into()))
	}
}
