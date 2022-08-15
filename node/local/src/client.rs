use sp_runtime::{
	generic::BlockId,
	traits::BlakeTwo256,
};
use gafi_primitives::types::{AccountId, Balance, Block, Hash, Index};

/// A set of APIs that polkadot-like runtimes must implement.
///
/// This trait has no methods or associated type. It is a concise marker for all the trait bounds
/// that it contains.
pub trait RuntimeApiCollection:
	sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
	+ sp_api::ApiExt<Block>
	+ sp_block_builder::BlockBuilder<Block>
	+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>
	+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
	+ sp_api::Metadata<Block>
	+ sp_offchain::OffchainWorkerApi<Block>
	+ sp_session::SessionKeys<Block>
	+ fp_rpc::ConvertTransactionRuntimeApi<Block>
	+ fp_rpc::EthereumRuntimeRPCApi<Block>
	// + moonbeam_rpc_primitives_debug::DebugRuntimeApi<Block>
	// + moonbeam_rpc_primitives_txpool::TxPoolRuntimeApi<Block>
	+ nimbus_primitives::NimbusApi<Block>
	+ nimbus_primitives::AuthorFilterAPI<Block, nimbus_primitives::NimbusId>
	+ cumulus_primitives_core::CollectCollationInfo<Block>
	+ session_keys_primitives::VrfApi<Block>
where
	<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> RuntimeApiCollection for Api
where
	Api: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::ApiExt<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>
		+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
		+ sp_api::Metadata<Block>
		+ sp_offchain::OffchainWorkerApi<Block>
		+ sp_session::SessionKeys<Block>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		// + moonbeam_rpc_primitives_debug::DebugRuntimeApi<Block>
		// + moonbeam_rpc_primitives_txpool::TxPoolRuntimeApi<Block>
		+ nimbus_primitives::NimbusApi<Block>
		+ nimbus_primitives::AuthorFilterAPI<Block, nimbus_primitives::NimbusId>
		+ cumulus_primitives_core::CollectCollationInfo<Block>
		+ session_keys_primitives::VrfApi<Block>,
	<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}
