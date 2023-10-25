// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_xcm_benchmarks::generic`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// target/production/polkadot
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=pallet_xcm_benchmarks::generic
// --chain=westend-dev
// --header=./file_header.txt
// --template=./xcm/pallet-xcm-benchmarks/template.hbs
// --output=./runtime/westend/src/weights/xcm/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_xcm_benchmarks::generic`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	/// Storage: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	/// Proof Skipped: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	/// Storage: Dmp DeliveryFeeFactor (r:1 w:0)
	/// Proof Skipped: Dmp DeliveryFeeFactor (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet SupportedVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub(crate) fn report_holding() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `3634`
		// Minimum execution time: 30_790_000 picoseconds.
		Weight::from_parts(31_265_000, 3634)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	pub(crate) fn buy_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_741_000 picoseconds.
		Weight::from_parts(2_823_000, 0)
	}
	/// Storage: XcmPallet Queries (r:1 w:0)
	/// Proof Skipped: XcmPallet Queries (max_values: None, max_size: None, mode: Measured)
	pub(crate) fn query_response() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `3634`
		// Minimum execution time: 10_848_000 picoseconds.
		Weight::from_parts(11_183_000, 3634)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	pub(crate) fn transact() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_145_000 picoseconds.
		Weight::from_parts(12_366_000, 0)
	}
	pub(crate) fn refund_surplus() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_837_000 picoseconds.
		Weight::from_parts(2_939_000, 0)
	}
	pub(crate) fn set_error_handler() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_526_000 picoseconds.
		Weight::from_parts(2_622_000, 0)
	}
	pub(crate) fn set_appendix() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_603_000 picoseconds.
		Weight::from_parts(2_642_000, 0)
	}
	pub(crate) fn clear_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_500_000 picoseconds.
		Weight::from_parts(2_573_000, 0)
	}
	pub(crate) fn descend_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_323_000 picoseconds.
		Weight::from_parts(3_401_000, 0)
	}
	pub(crate) fn clear_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_557_000 picoseconds.
		Weight::from_parts(2_620_000, 0)
	}
	/// Storage: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	/// Proof Skipped: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	/// Storage: Dmp DeliveryFeeFactor (r:1 w:0)
	/// Proof Skipped: Dmp DeliveryFeeFactor (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet SupportedVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub(crate) fn report_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `3634`
		// Minimum execution time: 25_828_000 picoseconds.
		Weight::from_parts(26_318_000, 3634)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: XcmPallet AssetTraps (r:1 w:1)
	/// Proof Skipped: XcmPallet AssetTraps (max_values: None, max_size: None, mode: Measured)
	pub(crate) fn claim_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226`
		//  Estimated: `3691`
		// Minimum execution time: 14_794_000 picoseconds.
		Weight::from_parts(15_306_000, 3691)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	pub(crate) fn trap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_534_000 picoseconds.
		Weight::from_parts(2_574_000, 0)
	}
	/// Storage: XcmPallet VersionNotifyTargets (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	/// Storage: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	/// Proof Skipped: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	/// Storage: Dmp DeliveryFeeFactor (r:1 w:0)
	/// Proof Skipped: Dmp DeliveryFeeFactor (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet SupportedVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub(crate) fn subscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `3634`
		// Minimum execution time: 32_218_000 picoseconds.
		Weight::from_parts(32_945_000, 3634)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: XcmPallet VersionNotifyTargets (r:0 w:1)
	/// Proof Skipped: XcmPallet VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	pub(crate) fn unsubscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_983_000 picoseconds.
		Weight::from_parts(5_132_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	pub(crate) fn burn_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_101_000 picoseconds.
		Weight::from_parts(4_228_000, 0)
	}
	pub(crate) fn expect_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_740_000 picoseconds.
		Weight::from_parts(2_814_000, 0)
	}
	pub(crate) fn expect_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_716_000 picoseconds.
		Weight::from_parts(2_795_000, 0)
	}
	pub(crate) fn expect_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_550_000 picoseconds.
		Weight::from_parts(2_601_000, 0)
	}
	pub(crate) fn expect_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_762_000 picoseconds.
		Weight::from_parts(2_849_000, 0)
	}
	/// Storage: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	/// Proof Skipped: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	/// Storage: Dmp DeliveryFeeFactor (r:1 w:0)
	/// Proof Skipped: Dmp DeliveryFeeFactor (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet SupportedVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub(crate) fn query_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `3634`
		// Minimum execution time: 31_709_000 picoseconds.
		Weight::from_parts(32_288_000, 3634)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	pub(crate) fn expect_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_209_000 picoseconds.
		Weight::from_parts(7_332_000, 0)
	}
	/// Storage: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	/// Proof Skipped: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	/// Storage: Dmp DeliveryFeeFactor (r:1 w:0)
	/// Proof Skipped: Dmp DeliveryFeeFactor (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet SupportedVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: XcmPallet VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: XcmPallet SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	pub(crate) fn report_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `3634`
		// Minimum execution time: 26_161_000 picoseconds.
		Weight::from_parts(26_605_000, 3634)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	pub(crate) fn clear_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_539_000 picoseconds.
		Weight::from_parts(2_647_000, 0)
	}
	pub(crate) fn set_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_494_000 picoseconds.
		Weight::from_parts(2_588_000, 0)
	}
	pub(crate) fn clear_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_510_000 picoseconds.
		Weight::from_parts(2_590_000, 0)
	}
	pub(crate) fn set_fees_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_491_000 picoseconds.
		Weight::from_parts(2_546_000, 0)
	}
	pub(crate) fn unpaid_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_696_000 picoseconds.
		Weight::from_parts(2_816_000, 0)
	}
}