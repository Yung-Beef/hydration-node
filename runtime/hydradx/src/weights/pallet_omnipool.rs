// This file is part of HydraDX.

// Copyright (C) 2020-2024  Intergalactic, Limited (GIB).
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


//! Autogenerated weights for `pallet_omnipool`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-04-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// ./bin/hydradx
// benchmark
// pallet
// --wasm-execution=compiled
// --pallet
// pallet_omnipool
// --extrinsic
// *
// --heap-pages
// 4096
// --steps
// 50
// --repeat
// 20
// --template=scripts/pallet-weight-template.hbs
// --output
// runtime/hydradx/src/weights/pallet_omnipool.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_omnipool`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `pallet_omnipool` using the HydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_omnipool::WeightInfo for HydraWeight<T> {
	/// Storage: `Omnipool::Assets` (r:1 w:1)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:1 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:1)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::NextPositionId` (r:1 w:1)
	/// Proof: `Omnipool::NextPositionId` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:1)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::CollectionMaxSupply` (r:1 w:0)
	/// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `DynamicFees::AssetFee` (r:1 w:0)
	/// Proof: `DynamicFees::AssetFee` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:1 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:1)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Positions` (r:0 w:1)
	/// Proof: `Omnipool::Positions` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	fn add_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3253`
		//  Estimated: `7406`
		// Minimum execution time: 164_981_000 picoseconds.
		Weight::from_parts(165_740_000, 7406)
			.saturating_add(T::DbWeight::get().reads(14_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:3 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Assets` (r:1 w:1)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `DynamicFees::AssetFee` (r:1 w:1)
	/// Proof: `DynamicFees::AssetFee` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::NextPositionId` (r:1 w:1)
	/// Proof: `Omnipool::NextPositionId` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:1)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:1 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::CollectionMaxSupply` (r:1 w:0)
	/// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::LiquidityAddLimitPerAsset` (r:1 w:0)
	/// Proof: `CircuitBreaker::LiquidityAddLimitPerAsset` (`max_values`: None, `max_size`: Some(29), added: 2504, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::AllowedAddLiquidityAmountPerAsset` (r:1 w:1)
	/// Proof: `CircuitBreaker::AllowedAddLiquidityAmountPerAsset` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::LiquidityRemoveLimitPerAsset` (r:1 w:0)
	/// Proof: `CircuitBreaker::LiquidityRemoveLimitPerAsset` (`max_values`: None, `max_size`: Some(29), added: 2504, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::AllowedRemoveLiquidityAmountPerAsset` (r:1 w:1)
	/// Proof: `CircuitBreaker::AllowedRemoveLiquidityAmountPerAsset` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:1)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Positions` (r:0 w:1)
	/// Proof: `Omnipool::Positions` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5245`
		//  Estimated: `8739`
		// Minimum execution time: 289_108_000 picoseconds.
		Weight::from_parts(290_794_000, 8739)
			.saturating_add(T::DbWeight::get().reads(23_u64))
			.saturating_add(T::DbWeight::get().writes(14_u64))
	}
	/// Storage: `Uniques::Asset` (r:1 w:1)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Positions` (r:1 w:1)
	/// Proof: `Omnipool::Positions` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Assets` (r:1 w:1)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:4 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `DynamicFees::AssetFee` (r:1 w:0)
	/// Proof: `DynamicFees::AssetFee` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:1 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::LiquidityAddLimitPerAsset` (r:1 w:0)
	/// Proof: `CircuitBreaker::LiquidityAddLimitPerAsset` (`max_values`: None, `max_size`: Some(29), added: 2504, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::AllowedAddLiquidityAmountPerAsset` (r:1 w:0)
	/// Proof: `CircuitBreaker::AllowedAddLiquidityAmountPerAsset` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::LiquidityRemoveLimitPerAsset` (r:1 w:0)
	/// Proof: `CircuitBreaker::LiquidityRemoveLimitPerAsset` (`max_values`: None, `max_size`: Some(29), added: 2504, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::AllowedRemoveLiquidityAmountPerAsset` (r:1 w:1)
	/// Proof: `CircuitBreaker::AllowedRemoveLiquidityAmountPerAsset` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:1)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	/// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `8777`
		//  Estimated: `11322`
		// Minimum execution time: 334_018_000 picoseconds.
		Weight::from_parts(336_165_000, 11322)
			.saturating_add(T::DbWeight::get().reads(26_u64))
			.saturating_add(T::DbWeight::get().writes(15_u64))
	}
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:7)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Assets` (r:2 w:2)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `DynamicFees::AssetFee` (r:2 w:0)
	/// Proof: `DynamicFees::AssetFee` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::LinkedAccounts` (r:1 w:0)
	/// Proof: `Referrals::LinkedAccounts` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::Referrer` (r:1 w:0)
	/// Proof: `Referrals::Referrer` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::AssetRewards` (r:1 w:0)
	/// Proof: `Referrals::AssetRewards` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:2 w:2)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::TotalShares` (r:1 w:1)
	/// Proof: `Referrals::TotalShares` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::ReferrerShares` (r:1 w:1)
	/// Proof: `Referrals::ReferrerShares` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::TraderShares` (r:1 w:1)
	/// Proof: `Referrals::TraderShares` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::PendingConversions` (r:1 w:1)
	/// Proof: `Referrals::PendingConversions` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::CounterForPendingConversions` (r:1 w:1)
	/// Proof: `Referrals::CounterForPendingConversions` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::AllowedTradeVolumeLimitPerAsset` (r:2 w:2)
	/// Proof: `CircuitBreaker::AllowedTradeVolumeLimitPerAsset` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Broadcast::IncrementalId` (r:1 w:1)
	/// Proof: `Broadcast::IncrementalId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn sell() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `9321`
		//  Estimated: `19071`
		// Minimum execution time: 435_149_000 picoseconds.
		Weight::from_parts(438_733_000, 19071)
			.saturating_add(T::DbWeight::get().reads(38_u64))
			.saturating_add(T::DbWeight::get().writes(23_u64))
	}
	/// Storage: `Omnipool::Assets` (r:2 w:2)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:7)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `DynamicFees::AssetFee` (r:2 w:0)
	/// Proof: `DynamicFees::AssetFee` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::LinkedAccounts` (r:1 w:0)
	/// Proof: `Referrals::LinkedAccounts` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::Referrer` (r:1 w:0)
	/// Proof: `Referrals::Referrer` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::AssetRewards` (r:1 w:0)
	/// Proof: `Referrals::AssetRewards` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:2 w:2)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::TotalShares` (r:1 w:1)
	/// Proof: `Referrals::TotalShares` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::ReferrerShares` (r:1 w:1)
	/// Proof: `Referrals::ReferrerShares` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::TraderShares` (r:2 w:2)
	/// Proof: `Referrals::TraderShares` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::PendingConversions` (r:1 w:1)
	/// Proof: `Referrals::PendingConversions` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::CounterForPendingConversions` (r:1 w:1)
	/// Proof: `Referrals::CounterForPendingConversions` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::AllowedTradeVolumeLimitPerAsset` (r:2 w:2)
	/// Proof: `CircuitBreaker::AllowedTradeVolumeLimitPerAsset` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Broadcast::IncrementalId` (r:1 w:1)
	/// Proof: `Broadcast::IncrementalId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn buy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `9321`
		//  Estimated: `19071`
		// Minimum execution time: 442_106_000 picoseconds.
		Weight::from_parts(445_948_000, 19071)
			.saturating_add(T::DbWeight::get().reads(39_u64))
			.saturating_add(T::DbWeight::get().writes(24_u64))
	}
	/// Storage: `Omnipool::Assets` (r:1 w:1)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn set_asset_tradable_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1338`
		//  Estimated: `3550`
		// Minimum execution time: 35_324_000 picoseconds.
		Weight::from_parts(35_996_000, 3550)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Omnipool::Assets` (r:1 w:0)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn refund_refused_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2710`
		//  Estimated: `6196`
		// Minimum execution time: 125_230_000 picoseconds.
		Weight::from_parts(125_560_000, 6196)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Omnipool::Positions` (r:1 w:1)
	/// Proof: `Omnipool::Positions` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:1)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `Omnipool::Assets` (r:1 w:1)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:1 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:1)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	/// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	fn sacrifice_position() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3673`
		//  Estimated: `3655`
		// Minimum execution time: 81_315_000 picoseconds.
		Weight::from_parts(81_810_000, 3655)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Omnipool::Assets` (r:1 w:1)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn set_asset_weight_cap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1338`
		//  Estimated: `3550`
		// Minimum execution time: 36_093_000 picoseconds.
		Weight::from_parts(36_504_000, 3550)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Omnipool::Assets` (r:1 w:1)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:3 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn withdraw_protocol_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5094`
		//  Estimated: `8739`
		// Minimum execution time: 175_495_000 picoseconds.
		Weight::from_parts(177_083_000, 8739)
			.saturating_add(T::DbWeight::get().reads(14_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `Omnipool::Assets` (r:1 w:1)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:4 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:2 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn remove_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4202`
		//  Estimated: `11322`
		// Minimum execution time: 177_769_000 picoseconds.
		Weight::from_parts(178_776_000, 11322)
			.saturating_add(T::DbWeight::get().reads(16_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `Omnipool::Assets` (r:2 w:2)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:7)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `DynamicFees::AssetFee` (r:2 w:0)
	/// Proof: `DynamicFees::AssetFee` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::LinkedAccounts` (r:1 w:0)
	/// Proof: `Referrals::LinkedAccounts` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::AssetRewards` (r:1 w:0)
	/// Proof: `Referrals::AssetRewards` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:2 w:2)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::TotalShares` (r:1 w:1)
	/// Proof: `Referrals::TotalShares` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::TraderShares` (r:1 w:1)
	/// Proof: `Referrals::TraderShares` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::PendingConversions` (r:1 w:1)
	/// Proof: `Referrals::PendingConversions` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::CounterForPendingConversions` (r:1 w:1)
	/// Proof: `Referrals::CounterForPendingConversions` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::AllowedTradeVolumeLimitPerAsset` (r:2 w:2)
	/// Proof: `CircuitBreaker::AllowedTradeVolumeLimitPerAsset` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Broadcast::IncrementalId` (r:1 w:1)
	/// Proof: `Broadcast::IncrementalId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_sell(_c: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2395 + e * (6209 ±0)`
		//  Estimated: `8799 + e * (12915 ±0)`
		// Minimum execution time: 68_292_000 picoseconds.
		Weight::from_parts(71_372_132, 8799)
			// Standard Error: 159_004
			.saturating_add(Weight::from_parts(373_246_704, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().reads((28_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes((22_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 12915).saturating_mul(e.into()))
	}
	/// Storage: `Omnipool::Assets` (r:2 w:2)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:7)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `DynamicFees::AssetFee` (r:2 w:0)
	/// Proof: `DynamicFees::AssetFee` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::LinkedAccounts` (r:1 w:0)
	/// Proof: `Referrals::LinkedAccounts` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::AssetRewards` (r:1 w:0)
	/// Proof: `Referrals::AssetRewards` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:2 w:2)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::TotalShares` (r:1 w:1)
	/// Proof: `Referrals::TotalShares` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::TraderShares` (r:1 w:1)
	/// Proof: `Referrals::TraderShares` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::PendingConversions` (r:1 w:1)
	/// Proof: `Referrals::PendingConversions` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Referrals::CounterForPendingConversions` (r:1 w:1)
	/// Proof: `Referrals::CounterForPendingConversions` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `CircuitBreaker::AllowedTradeVolumeLimitPerAsset` (r:2 w:2)
	/// Proof: `CircuitBreaker::AllowedTradeVolumeLimitPerAsset` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Broadcast::IncrementalId` (r:1 w:1)
	/// Proof: `Broadcast::IncrementalId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_buy(c: u32, _e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `8604`
		//  Estimated: `19071`
		// Minimum execution time: 422_223_000 picoseconds.
		Weight::from_parts(410_395_040, 19071)
			// Standard Error: 207_594
			.saturating_add(Weight::from_parts(17_522_065, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(36_u64))
			.saturating_add(T::DbWeight::get().writes(22_u64))
	}
	/// Storage: `Omnipool::Assets` (r:2 w:0)
	/// Proof: `Omnipool::Assets` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `DynamicFees::AssetFee` (r:2 w:0)
	/// Proof: `DynamicFees::AssetFee` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn calculate_spot_price_with_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2395`
		//  Estimated: `6190`
		// Minimum execution time: 69_861_000 picoseconds.
		Weight::from_parts(70_545_000, 6190)
			.saturating_add(T::DbWeight::get().reads(8_u64))
	}
}