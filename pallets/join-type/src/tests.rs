use super::*;
use crate::mock::*;

use frame_support::{assert_err, traits::Currency};
use gafi_primitives::{
	address_mapping::EVMAddressMapping,
	currency::{unit, NativeToken::GAKI},
	pool::{SponsoredPoolJoinType, SponsoredPoolJoinTypeHandle},
};
use sp_core::{bytes::to_hex, H160};
use sp_runtime::{
	offchain::testing,
	AccountId32,
};

fn make_deposit(account: &AccountId32, balance: u128) {
	let _ = pallet_balances::Pallet::<Test>::deposit_creating(account, balance);
}
fn new_account(account: [u8; 32], balance: u128) -> AccountId32 {
	let acc: AccountId32 = AccountId32::from(account);
	make_deposit(&acc, balance);
	assert_eq!(Balances::free_balance(&acc), balance);
	return acc
}

#[test]
fn is_can_join_should_work() {
	let (offchain, state) = testing::TestOffchainExt::new();
	ExtBuilder::default().build_and_execute_offchain(offchain, || {
		let account_balance = 1_000_000 * unit(GAKI);
		let account = new_account([1_u8; 32], account_balance);
		let origin_address: H160 = ProofAddressMapping::get_or_create_evm_address(account.clone());
		assert_eq!(
			ProofAddressMapping::insert_pair(origin_address.clone(), account.clone()),
			Ok(()),
		);

		let pool_id = [0u8; 32];
		let pool_id_str = to_hex(&pool_id, false);
		let address = to_hex(&origin_address.as_bytes(), false);
		let url = "http://localhost:3001/";

		// blacklist mode
		assert_eq!(
			SponsoredPoolJoin::set_join_type(
				pool_id,
				SponsoredPoolJoinType::Default,
				url.clone().as_bytes().to_vec(),
				account.clone(),
			),
			Ok(()),
		);
		// blacklist: true
		check_response(
			&mut state.write(),
			url,
			address.as_str(),
			pool_id_str.as_str(),
			"true",
		);
		assert_err!(
			SponsoredPoolJoin::is_can_join_pool(pool_id, account.clone()),
			Error::<Test>::Blacklist
		);
		// blacklist: false
		check_response(
			&mut state.write(),
			url,
			address.as_str(),
			pool_id_str.as_str(),
			"false",
		);
		assert_eq!(
			SponsoredPoolJoin::is_can_join_pool(pool_id, account.clone()),
			Ok(())
		);

		// whitelist mode
		assert_eq!(
			SponsoredPoolJoin::set_join_type(
				pool_id,
				SponsoredPoolJoinType::Whitelist,
				url.clone().as_bytes().to_vec(),
				account.clone(),
			),
			Ok(()),
		);
		// whitelist: true
		check_response(
			&mut state.write(),
			url,
			address.as_str(),
			pool_id_str.as_str(),
			"true",
		);
		assert_eq!(
			SponsoredPoolJoin::is_can_join_pool(pool_id, account.clone()),
			Ok(())
		);
		// whitelist: false
		check_response(
			&mut state.write(),
			url,
			address.as_str(),
			pool_id_str.as_str(),
			"false",
		);
		assert_err!(
			SponsoredPoolJoin::is_can_join_pool(pool_id, account.clone()),
			Error::<Test>::NotInWhiteList
		);

		// do not set join type
		assert_eq!(
			SponsoredPoolJoin::reset(pool_id, account.clone()),
			Ok(())
		);
		assert_eq!(
			SponsoredPoolJoin::is_can_join_pool(pool_id, account.clone()),
			Ok(())
		);
	});
}

fn check_response(
	state: &mut testing::OffchainState,
	uri: &str,
	address: &str,
	pool_id: &str,
	rs: &str,
) {
	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: (uri.to_string() + "?address=" + address + "&pool_id=" + pool_id).into(),
		response: Some(rs.into()),
		sent: true,
		..Default::default()
	});
}
