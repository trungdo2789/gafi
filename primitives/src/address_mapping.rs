
use frame_support::pallet_prelude::DispatchResult;
pub trait EVMAddressMapping<AccountId,EVMAddress> {
	fn get_evm_address_mapping(account_id: AccountId) -> Option<EVMAddress>;
	fn get_account_mapping(address: EVMAddress) -> Option<AccountId>;
	fn insert_pair(address: EVMAddress,account_id: AccountId) -> DispatchResult;
	fn remove_pair(address: EVMAddress,account_id: AccountId) -> DispatchResult;
}
