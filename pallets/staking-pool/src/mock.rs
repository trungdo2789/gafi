/*
* This unittest should only test logic function e.g. Storage, Computation
* and not related with Currency e.g. Balances, Transaction Payment
*/

use crate::{self as staking_pool};
use frame_support::{parameter_types, traits::GenesisBuild};
use frame_system as system;

use frame_support::{
	dispatch::Vec,
	traits::{OnFinalize, OnInitialize},
};
use gafi_primitives::{
	constant::ID,
	system_services::{SystemDefaultServices, SystemService},
	currency::{unit, NativeToken::GAKI}
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32,
	Permill
};
pub use pallet_balances::Call as BalancesCall;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const STAKING_BASIC_ID: ID = [0_u8; 32];
pub const STAKING_MEDIUM_ID: ID = [1_u8; 32];
pub const STAKING_ADVANCE_ID: ID = [2_u8; 32];

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		StakingPool: staking_pool::{Pallet, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
	}
);

impl pallet_randomness_collective_flip::Config for Test {}

pub const EXISTENTIAL_DEPOSIT: u128 = 1000;

parameter_types! {
	pub ExistentialDeposit: u128 = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u128;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const INIT_TIMESTAMP: u64 = 30_000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}


parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 24;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId32;
	type AccountData = pallet_balances::AccountData<u128>;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub struct StakingPoolDefaultServices {}

impl SystemDefaultServices for StakingPoolDefaultServices {
	fn get_default_services () -> [(ID, SystemService); 3] {
		[
			(
				STAKING_BASIC_ID,
				SystemService::new(STAKING_BASIC_ID, 10_u32, Permill::from_percent(30), 1000 * unit(GAKI)),
			),
			(
				STAKING_MEDIUM_ID,
				SystemService::new(STAKING_MEDIUM_ID, 10_u32, Permill::from_percent(50), 1500 * unit(GAKI)),
			),
			(
				STAKING_ADVANCE_ID,
				SystemService::new(STAKING_ADVANCE_ID, 10_u32, Permill::from_percent(70), 2000 * unit(GAKI)),
			),
		]
	}
}

impl staking_pool::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type WeightInfo = ();
	type StakingServices = StakingPoolDefaultServices;
	type Players = ();
}

// Build genesis storage according to the mock runtime.
pub fn _new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Timestamp::set_timestamp((System::block_number() as u64 * MILLISECS_PER_BLOCK) + INIT_TIMESTAMP);
	}
}

pub struct ExtBuilder {
	balances: Vec<(AccountId32, u128)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			balances: vec![],
		}
	}
}

impl ExtBuilder {
	fn build(self) -> sp_io::TestExternalities {
		let mut storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		let _ = pallet_balances::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut storage);

		GenesisBuild::<Test>::assimilate_storage(
			&staking_pool::GenesisConfig::default(),
			&mut storage,
		)
		.unwrap();

		let ext = sp_io::TestExternalities::from(storage);
		ext
	}

	pub fn build_and_execute(self, test: impl FnOnce() -> ()) {
		let mut ext = self.build();
		ext.execute_with(test);
		ext.execute_with(|| System::set_block_number(1));
	}
}
