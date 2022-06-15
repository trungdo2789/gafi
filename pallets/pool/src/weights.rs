
//! Autogenerated weights for `pallet_pool`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-06, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/gafi-node
// benchmark
// pallet
// --chain
// dev
// --wasm-execution
// compiled
// --pallet
// pallet_pool
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// ./benchmarking/pool/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use gafi_primitives::ticket::{TicketType, SystemTicket, CustomTicket};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn join(s: u32, ticket: TicketType ) -> Weight;
	fn leave(s: u32, ) -> Weight;
}

/// Weight functions for `pallet_pool`.
pub struct PoolWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PoolWeight<T> {
		// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Pool Tickets (r:1 w:1)
	// Storage: UpfrontPool Services (r:1 w:0)
	// Storage: UpfrontPool PlayerCount (r:1 w:1)
	// Storage: UpfrontPool MaxPlayer (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: UpfrontPool NewPlayers (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Balances TotalIssuance (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: UpfrontPool Tickets (r:0 w:1)
	// Storage: StakingPool Services (r:1 w:0)
	// Storage: StakingPool PlayerCount (r:1 w:1)
	// Storage: StakingPool Tickets (r:0 w:1)
	// Storage: SponsoredPool Pools (r:1 w:0)
	// Storage: SponsoredPool Targets (r:1 w:0)
	// Storage: PalletCache DataLeft (r:1 w:0)
	// Storage: PalletCache DataRight (r:1 w:0)
	fn join(s: u32, ticket: TicketType) -> Weight {
		let total_read = 13_u64;
		let total_write = 10_u64;

		let upfront_r = 4_u64;
		let upfront_w = 3_u64;
		let staking_r = 2_u64;
		let staking_w = 2_u64;
		let sponsored_r = 2_u64;
		let sponsored_w = 0_u64;

		let base_r = total_read - upfront_r - staking_r - sponsored_r;
		let base_w = total_write - upfront_w - staking_w - sponsored_w;
		let mut weight = (43_470_000_u64).saturating_mul(s.into())
		.saturating_add(T::DbWeight::get().reads(base_r))
		.saturating_add(T::DbWeight::get().writes(base_w));

		match ticket {
   			TicketType::System(SystemTicket::Upfront(_)) => {
				   weight = (weight).saturating_add(T::DbWeight::get().reads(upfront_r))
				   .saturating_add(T::DbWeight::get().writes(upfront_w));
			   },
    		TicketType::System(SystemTicket::Staking(_)) => {
				weight = (weight).saturating_add(T::DbWeight::get().reads(staking_r))
				.saturating_add(T::DbWeight::get().writes(staking_w));
			},
			TicketType::Custom(CustomTicket::Sponsored(_)) => {
				weight = (weight).saturating_add(T::DbWeight::get().reads(sponsored_r))
				.saturating_add(T::DbWeight::get().writes(sponsored_w));
			},
		}
		weight
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Pool Tickets (r:1 w:1)
	// Storage: UpfrontPool Tickets (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: UpfrontPool Services (r:1 w:0)
	// Storage: Pool TimeService (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Balances TotalIssuance (r:1 w:1)
	// Storage: UpfrontPool PlayerCount (r:1 w:1)
	// Storage: UpfrontPool IngamePlayers (r:1 w:1)
	// Storage: UpfrontPool NewPlayers (r:1 w:1)
	// Storage: StakingPool Tickets (r:1 w:1)
	// Storage: StakingPool PlayerCount (r:1 w:1)
	// Storage: StakingPool Services (r:1 w:0)
	// Storage: PalletCache DataFlag (r:1 w:0)
	// Storage: PalletCache DataLeft (r:0 w:1)
	fn leave(s: u32 ) -> Weight {
			let total_read = 16_u64;
			let total_write = 11_u64;
	
			let upfront_r = 5_u64;
			let upfront_w = 4_u64;
			let staking_r = 3_u64;
			let staking_w = 2_u64;
			let sponsored_r = 0_u64;
			let sponsored_w = 0_u64;
	
			let base_r = total_read - upfront_r - staking_r - sponsored_r;
			let base_w = total_write - upfront_w - staking_w - sponsored_w;
			let mut weight = (46_286_000_u64).saturating_mul(s.into())
			.saturating_add(T::DbWeight::get().reads(base_r))
			.saturating_add(T::DbWeight::get().writes(base_w));
	
			// match ticket {
			// 	   TicketType::System(SystemTicket::Upfront(_)) => {
			// 		   weight = (weight).saturating_add(T::DbWeight::get().reads(upfront_r))
			// 		   .saturating_add(T::DbWeight::get().writes(upfront_w));
			// 	   },
			// 	TicketType::System(SystemTicket::Staking(_)) => {
			// 		weight = (weight).saturating_add(T::DbWeight::get().reads(staking_r))
			// 		.saturating_add(T::DbWeight::get().writes(staking_w));
			// 	},
			// 	TicketType::Custom(CustomTicket::Sponsored(_)) => {
			// 		weight = (weight).saturating_add(T::DbWeight::get().reads(sponsored_r))
			// 		.saturating_add(T::DbWeight::get().writes(sponsored_w));
			// 	},
			// }
			weight
	}
}

impl WeightInfo for () {
	fn join(s: u32, ticket: TicketType ) -> Weight {
		let total_read = 15_u64;
		let total_write = 9_u64;

		let upfront_r = 4_u64;
		let upfront_w = 3_u64;
		let staking_r = 2_u64;
		let staking_w = 2_u64;
		let sponsored_r = 2_u64;
		let sponsored_w = 0_u64;

		let base_r = total_read - upfront_r - staking_r - sponsored_r;
		let base_w = total_write - upfront_w - staking_w - sponsored_w;
		let mut weight = (43_470_000_u64).saturating_mul(s.into())
		.saturating_add(RocksDbWeight::get().reads(base_r))
		.saturating_add(RocksDbWeight::get().writes(base_w));

		match ticket {
   			TicketType::System(SystemTicket::Upfront(_)) => {
				   weight = (weight).saturating_add(RocksDbWeight::get().reads(upfront_r))
				   .saturating_add(RocksDbWeight::get().writes(upfront_w));
			   },
			TicketType::System(SystemTicket::Staking(_)) => {
				weight = (weight).saturating_add(RocksDbWeight::get().reads(staking_r))
				.saturating_add(RocksDbWeight::get().writes(staking_w));
			},
			TicketType::Custom(CustomTicket::Sponsored(_)) => {
				weight = (weight).saturating_add(RocksDbWeight::get().reads(sponsored_r))
				.saturating_add(RocksDbWeight::get().writes(sponsored_w));
			},
		}
		weight
	}

	fn leave(s: u32,) -> Weight {
		let total_read = 16_u64;
			let total_write = 11_u64;
	
			let upfront_r = 5_u64;
			let upfront_w = 4_u64;
			let staking_r = 3_u64;
			let staking_w = 2_u64;
			let sponsored_r = 0_u64;
			let sponsored_w = 0_u64;
	
			let base_r = total_read - upfront_r - staking_r - sponsored_r;
			let base_w = total_write - upfront_w - staking_w - sponsored_w;
			let mut weight = (46_286_000_u64).saturating_mul(s.into())
			.saturating_add(RocksDbWeight::get().reads(base_r))
			.saturating_add(RocksDbWeight::get().writes(base_w));
	
			// match ticket {
			// 	   TicketType::System(SystemTicket::Upfront(_)) => {
			// 		   weight = (weight).saturating_add(RocksDbWeight::get().reads(upfront_r))
			// 		   .saturating_add(RocksDbWeight::get().writes(upfront_w));
			// 	   },
			// 	TicketType::System(SystemTicket::Staking(_)) => {
			// 		weight = (weight).saturating_add(RocksDbWeight::get().reads(staking_r))
			// 		.saturating_add(RocksDbWeight::get().writes(staking_w));
			// 	},
			// 	TicketType::Custom(CustomTicket::Sponsored(_)) => {
			// 		weight = (weight).saturating_add(RocksDbWeight::get().reads(sponsored_r))
			// 		.saturating_add(RocksDbWeight::get().writes(sponsored_w));
			// 	},
			// }
			weight
	}
}
