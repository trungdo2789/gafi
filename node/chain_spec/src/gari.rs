use cumulus_primitives_core::ParaId;
use gafi_primitives::currency::{unit, GafiCurrency, NativeToken::GAFI, TokenInfo};
use gari_runtime::{
	AccountId, AuthorFilterConfig, EVMConfig, EligibilityValue, EthereumConfig,
	ParachainStakingConfig, AuthorMappingConfig, Signature, TxHandlerConfig, EXISTENTIAL_DEPOSIT,
	Balance, Range, InflationInfo
};
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public, U256};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::collections::BTreeMap;
use nimbus_primitives::NimbusId;
use sp_runtime::Perbill;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<gari_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

pub fn moonbeam_inflation_config() -> InflationInfo<Balance> {
	fn to_round_inflation(annual: Range<Perbill>) -> Range<Perbill> {
		use pallet_parachain_staking::inflation::{
			perbill_annual_to_perbill_round, BLOCKS_PER_YEAR,
		};
		perbill_annual_to_perbill_round(
			annual,
			// rounds per year
			BLOCKS_PER_YEAR /
				gari_runtime::get!(pallet_parachain_staking, DefaultBlocksPerRound, u32),
		)
	}
	let annual = Range {
		min: Perbill::from_percent(4),
		ideal: Perbill::from_percent(5),
		max: Perbill::from_percent(5),
	};
	InflationInfo {
		// staking expectations
		expect: Range {
			min: 100_000_000_000_000_000_000_000_u128,
			ideal: 200_000_000_000_000_000_000_000_u128,
			max: 500_000_000_000_000_000_000_000_u128,
		},
		// annual inflation
		annual,
		round: to_round_inflation(annual),
	}
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> NimbusId {
	get_public_from_seed::<NimbusId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
}

pub fn rococo_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut props = sc_chain_spec::Properties::new();

	let gafi = GafiCurrency::token_info(GAFI);
	let symbol = json!(String::from_utf8(gafi.symbol).unwrap_or("GAFI".to_string()));
	let name = json!(String::from_utf8(gafi.name).unwrap_or("Gafi Token".to_string()));
	let decimals = json!(gafi.decimals);
	props.insert("tokenSymbol".to_string(), symbol);
	props.insert("tokenName".to_string(), name);
	props.insert("tokenDecimals".to_string(), decimals);
	let id: ParaId = 4015.into();

	ChainSpec::from_genesis(
		// Name
		"Gari",
		// ID
		"gafi_rococo",
		ChainType::Live,
		move || {
			testnet_genesis(
				vec![
					//5FYu1DAUqRax7Pe8tQxQQccs1SyZNLn8oPaLgJo9RtaBge5o
					(
						hex!("9a3518c4346239d5384abd80659d358f23271e1049398dca23734203ab44b811")
							.into(),
						500_000_000_000_000_000_000_000_000_u128,
					),
					//5Gp4fsJUTCtKXfXwjsJvNevtJeZrKFGm3kvbrtWePqpCqtCZ
					(
						hex!("d2028d37ded894f5544d2c93efa810772e59f5340e169c54230d88ef1ef2ff1f")
							.into(),
						500_000_000_000_000_000_000_000_000_u128,
					),
				],
				// Collator Candidate
				vec![
				// Alice -> Alith
				(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_collator_keys_from_seed("Alice"),
					1_000_000_000_000_000_000_000_u128,
				),
				// Bob -> Baltathar
				(
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_collator_keys_from_seed("Bob"),
					1_000_000_000_000_000_000_000_u128,
				)
				],
				// Delegations
				vec![],
				id,
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
						500_000_000_000_000_000_000_000_000_u128,
					),
				],
				vec![
				// Alice -> Alith
				(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_collator_keys_from_seed("Alice"),
					1_000_000_000_000_000_000_000_u128,
				),
				// Bob -> Baltathar
				(
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_collator_keys_from_seed("Bob"),
					1_000_000_000_000_000_000_000_u128,
				)
				],
				// Delegations
				vec![],
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 1000,
		},
	)
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						500_000_000_000_000_000_000_000_000_u128,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
						500_000_000_000_000_000_000_000_000_u128,
					),
				],
				vec![
				// Alice -> Alith
				(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_collator_keys_from_seed("Alice"),
					1_000_000_000_000_000_000_000_u128,
				),
				// Bob -> Baltathar
				(
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_collator_keys_from_seed("Bob"),
					1_000_000_000_000_000_000_000_u128,
				)
				],
				// Delegations
				vec![],
				2000.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("template-local"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
}

fn testnet_genesis(
	endowed_accounts: Vec<(AccountId, u128)>,
	candidates: Vec<(AccountId, NimbusId, Balance)>,
	delegations: Vec<(AccountId, AccountId, Balance)>,
	id: ParaId,
) -> gari_runtime::GenesisConfig {
	let min_gas_price: U256 = U256::from(4_000_000_000_000u128);

	gari_runtime::GenesisConfig {
		system: gari_runtime::SystemConfig {
			code: gari_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: gari_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k.0, k.1)).collect(),
		},
		parachain_info: gari_runtime::ParachainInfoConfig { parachain_id: id },
		parachain_system: Default::default(),
		author_filter: AuthorFilterConfig {
			eligible_count: EligibilityValue::new_unchecked(50),
		},
		parachain_staking: ParachainStakingConfig {
			candidates: candidates
				.iter()
				.cloned()
				.map(|(account, _, bond)| (account, bond))
				.collect(),
			delegations,
			inflation_config: moonbeam_inflation_config(),
		},
		author_mapping: AuthorMappingConfig {
			mappings: candidates
				.iter()
				.cloned()
				.map(|(account_id, author_id, _)| (author_id, account_id))
				.collect(),
		},
		polkadot_xcm: gari_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
		evm: EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map
			},
		},
		ethereum: EthereumConfig {},
		tx_handler: TxHandlerConfig {
			gas_price: U256::from(min_gas_price),
		},
		pool: Default::default(),
		upfront_pool: Default::default(),
		treasury: Default::default(),
	}
}
