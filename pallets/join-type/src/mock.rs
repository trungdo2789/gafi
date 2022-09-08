use crate as pallet_join_type;
use frame_support::{
	parameter_types,
	traits::{ConstU32, OnFinalize, OnInitialize},
};
use gafi_primitives::currency::{unit, NativeToken::GAKI};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32, offchain::{testing::TestOffchainExt, OffchainWorkerExt},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		SponsoredPoolJoin: pallet_join_type::{Pallet, Storage, Event<T>},
		ProofAddressMapping: proof_address_mapping::{Pallet, Call, Storage, Event<T>},
	}
);

impl proof_address_mapping::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type WeightInfo = ();
	type MessagePrefix = Prefix;
	type ReservationFee = ReservationFee;
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 24;
}

impl frame_system::Config for Test {
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
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

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

pub const RESERVATION_FEE: u128 = 1;

parameter_types! {
	pub Prefix: &'static [u8] =  b"Bond Aurora Network account:";
	pub ReservationFee: u128 = RESERVATION_FEE * unit(GAKI);
}
impl pallet_join_type::Config for Test {
	type MaxLength = ConstU32<255>;
	type Event = Event;
	type AddressMapping = ProofAddressMapping;
}

// Build genesis storage according to the mock runtime.
// pub fn new_test_ext() -> sp_io::TestExternalities {
// 	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
// }

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
	}
}

pub struct ExtBuilder {
	balances: Vec<(AccountId32, u128)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { balances: vec![] }
	}
}

impl ExtBuilder {
	fn build(self) -> sp_io::TestExternalities {
		let mut storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		let _ = pallet_balances::GenesisConfig::<Test> {
			balances: self.balances,
		}
		.assimilate_storage(&mut storage);

		let ext = sp_io::TestExternalities::from(storage);
		ext
	}

	pub fn build_and_execute(self, test: impl FnOnce() -> ()) {
		let mut ext = self.build();
		ext.execute_with(test);
		ext.execute_with(|| System::set_block_number(1));
	}
	pub fn build_and_execute_offchain(self,offchain: TestOffchainExt, test: impl FnOnce() -> ()) {
		let mut ext = self.build();
		ext.register_extension(OffchainWorkerExt::new(offchain));
		ext.execute_with(test);
		ext.execute_with(|| System::set_block_number(1));
	}
}
