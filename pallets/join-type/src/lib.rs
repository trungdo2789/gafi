#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use sp_core::{H160, bytes::to_hex};
	use lite_json::{
		json::{JsonValue},
		json_parser::parse_json,
	};
	use gafi_primitives::{
		address_mapping::EVMAddressMapping,
		constant::ID,
		pool::{SponsoredPoolJoinType, SponsoredPoolJoinTypeHandle},
	};
	use sp_runtime::AccountId32;
	use sp_runtime::offchain::{http, Duration};
	use sp_std::{str, vec::Vec};

	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The maximum length a url.
		#[pallet::constant]
		type MaxLength: Get<u32>;

		/// Substrate - EVM Address Mapping
		type AddressMapping: EVMAddressMapping<AccountId32, H160>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn external_of)]
	pub(super) type ExternalOf<T: Config> =
		StorageMap<_, Twox64Concat, ID, (SponsoredPoolJoinType, BoundedVec<u8, T::MaxLength>)>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ChangeJoinType(ID, SponsoredPoolJoinType, Vec<u8>, T::AccountId),
		ResetJoinType(ID, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		UrlTooLong,
		UrlInvalid,
		NotFound,
		HttpFetchingError,
		DeserializeToStrError,
		DeserializeToObjError,
		Blacklist,
		NotInWhiteList,
		CallExternalFailed,
		NoEVMAddress,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> Pallet<T> {
		/// This function uses the `offchain::http` API to query the remote endpoint information,
		///   and returns the JSON response as vector of bytes.
		fn fetch_from_remote(uri: &str, time_out: u64) -> Result<Vec<u8>, Error<T>> {
			let request = http::Request::get(uri);
			let timeout = sp_io::offchain::timestamp().add(Duration::from_millis(time_out));
			let pending = request.deadline(timeout).send().map_err(|e| {
				log::error!("{:?}", e);
				<Error<T>>::HttpFetchingError
			})?;
			let response = pending
				.try_wait(timeout)
				.map_err(|e| {
					log::error!("{:?}", e);
					<Error<T>>::HttpFetchingError
				})?
				.map_err(|e| {
					log::error!("{:?}", e);
					<Error<T>>::HttpFetchingError
				})?;
			if response.code != 200 {
				log::error!("Unexpected http request status code: {}", response.code);
				return Err(<Error<T>>::HttpFetchingError)
			}
			Ok(response.body().collect::<Vec<u8>>())
		}

		/// Fetch from remote and deserialize the JSON to a struct
		fn fetch_n_parse(uri: &str, time_out: u64) -> Result<JsonValue, Error<T>> {
			let resp_bytes = Self::fetch_from_remote(uri, time_out).map_err(|e| {
				log::error!("fetch_from_remote error: {:?}", e);
				<Error<T>>::HttpFetchingError
			})?;
			let resp_str =
				str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::DeserializeToStrError)?;
			let info: JsonValue =
				parse_json(&resp_str).map_err(|_| <Error<T>>::DeserializeToObjError)?;
			Ok(info)
		}
	}

	impl<T: Config> SponsoredPoolJoinTypeHandle<AccountIdOf<T>> for Pallet<T>
	where
		[u8; 32]: From<<T as frame_system::Config>::AccountId>,
		AccountId32: From<<T as frame_system::Config>::AccountId>,
	{
		fn set_join_type(
			pool_id: ID,
			join_type: SponsoredPoolJoinType,
			call_check_url: Vec<u8>,
			account_id: AccountIdOf<T>,
		) -> DispatchResult {
			let bounded_url: BoundedVec<u8, T::MaxLength> =
				call_check_url.clone().try_into().map_err(|()| Error::<T>::UrlTooLong)?;
			<ExternalOf<T>>::insert(pool_id, (join_type, bounded_url));
			Self::deposit_event(Event::<T>::ChangeJoinType(
				pool_id,
				join_type,
				call_check_url,
				account_id,
			));
			Ok(())
		}
		fn reset(pool_id: ID, account_id: AccountIdOf<T>) -> DispatchResult {
			<ExternalOf<T>>::remove(pool_id);
			Self::deposit_event(Event::<T>::ResetJoinType(pool_id, account_id));
			Ok(())
		}
		fn get_join_type(pool_id: ID) -> Option<(SponsoredPoolJoinType, Vec<u8>)> {
			match <ExternalOf<T>>::get(pool_id).ok_or(Error::<T>::NotFound) {
				Ok((join_type, bounded_url)) =>
					return Some((join_type, Vec::<u8>::from(bounded_url))),
				Err(_) => return None,
			}
		}
		fn is_can_join_pool(pool_id: ID, account_id: AccountIdOf<T>) -> DispatchResult {
			let join_info = <ExternalOf<T>>::get(pool_id);
			match join_info {
				Some((join_type, bounded_url)) =>
					if bounded_url.len() > 0 {
						let account_address: H160 =
							T::AddressMapping::get_evm_address_mapping(AccountId32::from(account_id.clone()))
								.ok_or(<Error<T>>::NoEVMAddress)?;

						let pool_id_str = to_hex(&pool_id, false);
						let address = to_hex(&account_address.as_bytes(), false);

						let mut f_url = bounded_url.to_vec();
						f_url.append(&mut "?address=".as_bytes().to_vec());
						f_url.append(&mut address.as_bytes().to_vec());
						f_url.append(&mut "&pool_id=".as_bytes().to_vec());
						f_url.append(&mut pool_id_str.as_bytes().to_vec());

						let full_url = str::from_utf8(&f_url)
							.map_err(|_| <Error<T>>::DeserializeToStrError)?;
						let rs = Self::fetch_n_parse(full_url, 6000);
						match rs {
							Ok(check_info) =>
								if join_type == SponsoredPoolJoinType::Default {
									//blacklist mode
									ensure!(!check_info.to_bool().unwrap(), Error::<T>::Blacklist);
									return Ok(())
								} else {
									//whitelist mode
									ensure!(
										check_info.to_bool().unwrap(),
										Error::<T>::NotInWhiteList
									);
									return Ok(())
								},
							Err(_) => {
								ensure!(1 == 0, Error::<T>::CallExternalFailed);
								return Err(DispatchError::Other("CallExternalFailed"))
							},
						}
					} else {
						if join_type == SponsoredPoolJoinType::Default {
							return Ok(())
						} else {
							//whitelist url not set
							ensure!(1 == 0, Error::<T>::NotFound);
							return Ok(())
						};
					},
				None => return Ok(()),
			}
		}
	}
}
