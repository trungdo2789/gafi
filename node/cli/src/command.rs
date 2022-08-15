use crate::cli::{Cli, RelayChainCli, Subcommand};
use codec::Encode;
use cumulus_client_cli::generate_genesis_block;
use cumulus_primitives_core::ParaId;
#[cfg(feature = "runtime-benchmarks")]
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use gafi_rpc::{db_config_dir, frontier_database_dir};
use log::info;
use sc_cli::{
    ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
    NetworkParams, Result, RuntimeVersion, SharedParams, SubstrateCli,
};
use sc_service::{
    config::{BasePath, PrometheusConfig},
    DatabaseSource, PartialComponents, TaskManager,
};
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::traits::{AccountIdConversion, Block as BlockT};
use sp_runtime::AccountId32;
use std::{io::Write, net::SocketAddr};

use gafi_chain_spec::IdentifyVariant;
use gafi_service::{new_partial, GafiRuntimeExecutor};

use gafi_primitives::types::Block;

fn load_spec(id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
    Ok(match id {
        // Local
        #[cfg(feature = "with-gari")]
        "dev" => Box::new(gafi_chain_spec::gari::development_config()),

        #[cfg(feature = "with-gaki")]
        "dev" => Box::new(gafi_chain_spec::gaki::development_config()),

        // testnet
        #[cfg(feature = "with-gari")]
        "template-rococo" => Box::new(gafi_chain_spec::gari::local_testnet_config()),

        #[cfg(feature = "with-gaki")]
        "template-rococo" => Box::new(gafi_chain_spec::gaki::local_testnet_config()),

        #[cfg(feature = "with-gari")]
        "rococo" => Box::new(gafi_chain_spec::gari::rococo_config()),

        // custome chain
        #[cfg(feature = "with-gari")]
        path => Box::new(gafi_chain_spec::gari::ChainSpec::from_json_file(
            std::path::PathBuf::from(path),
        )?),

        #[cfg(feature = "with-gaki")]
        path => Box::new(gafi_chain_spec::gaki::ChainSpec::from_json_file(
            std::path::PathBuf::from(path),
        )?),
    })
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Parachain Collator Template".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        "Parachain Collator Template\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		parachain-collator <parachain-args> -- <relay-chain-args>"
            .into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/paritytech/cumulus/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2020
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(id)
    }

    fn native_runtime_version(
        spec: &Box<dyn gafi_chain_spec::ChainSpec>,
    ) -> &'static RuntimeVersion {
        #[cfg(feature = "with-gaki")]
        if spec.is_gaki() {
            return &gafi_service::gaki_runtime::VERSION;
        }

        #[cfg(not(all(feature = "with-gaki",)))]
        let _ = spec;

        #[cfg(feature = "with-gari")]
        {
            return &gafi_service::gari_runtime::VERSION;
        }

        #[cfg(not(feature = "with-gari"))]
        panic!("No runtime feature (gari, gaki) is enabled")
    }
}

impl SubstrateCli for RelayChainCli {
    fn impl_name() -> String {
        "Parachain Collator Template".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        "Parachain Collator Template\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		parachain-collator <parachain-args> -- <relay-chain-args>"
            .into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/paritytech/cumulus/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2020
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
    }

    fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        polkadot_cli::Cli::native_runtime_version(chain_spec)
    }
}

#[allow(clippy::borrowed_box)]
fn extract_genesis_wasm(chain_spec: &Box<dyn sc_service::ChainSpec>) -> Result<Vec<u8>> {
    let mut storage = chain_spec.build_storage()?;

    storage
        .top
        .remove(sp_core::storage::well_known_keys::CODE)
        .ok_or_else(|| "Could not find wasm file in genesis state!".into())
}

/// Parse command line arguments into service configuration.
#[cfg(feature = "with-gari")]
pub fn run_gari() -> Result<()> {
    use gafi_service::gari_runtime::RuntimeApi;
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            return runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = gafi_service::new_partial_gari::<RuntimeApi, GafiRuntimeExecutor>(
                    &config,
false
                )?;
                Ok((cmd.run(client, import_queue), task_manager))
            });
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = gafi_service::new_partial_gari::<RuntimeApi, GafiRuntimeExecutor>(
                    &config,
					false
                )?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            return runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = gafi_service::new_partial_gari::<RuntimeApi, GafiRuntimeExecutor>(
                    &config,
					false
                )?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            });
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            return runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = gafi_service::new_partial_gari::<RuntimeApi, GafiRuntimeExecutor>(
                    &config,
                    false
                )?;
                Ok((cmd.run(client, import_queue), task_manager))
            });
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            return runner.sync_run(|config| {
                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relay_chain_args.iter()),
                );

                let polkadot_config = SubstrateCli::create_configuration(
                    &polkadot_cli,
                    &polkadot_cli,
                    config.tokio_handle.clone(),
                )
                .map_err(|err| format!("Relay chain argument error: {}", err))?;

                cmd.run(config, polkadot_config)
            });
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            return runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    backend,
                    ..
                } = gafi_service::new_partial_gari::<RuntimeApi, GafiRuntimeExecutor>(
                    &config,
					false
                )?;
                let aux_revert = Box::new(|client, _, blocks| {
                    sc_finality_grandpa::revert(client, blocks)?;
                    Ok(())
                });
                Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
            });
        }
        Some(Subcommand::ExportGenesisState(params)) => {
            let mut builder = sc_cli::LoggerBuilder::new("");
            builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
            let _ = builder.init();

            let spec = load_spec(&params.chain.clone().unwrap_or_default())?;
            let state_version = Cli::native_runtime_version(&spec).state_version();
            let block: Block = generate_genesis_block(&*spec, state_version)?;
            let raw_header = block.header().encode();
            let output_buf = if params.raw {
                raw_header
            } else {
                format!("0x{:?}", HexDisplay::from(&block.header().encode())).into_bytes()
            };

            if let Some(output) = &params.output {
                std::fs::write(output, output_buf)?;
            } else {
                std::io::stdout().write_all(&output_buf)?;
            }

            Ok(())
        }
        Some(Subcommand::ExportGenesisWasm(params)) => {
            let mut builder = sc_cli::LoggerBuilder::new("");
            builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
            let _ = builder.init();

            let raw_wasm_blob =
                extract_genesis_wasm(&cli.load_spec(&params.chain.clone().unwrap_or_default())?)?;
            let output_buf = if params.raw {
                raw_wasm_blob
            } else {
                format!("0x{:?}", HexDisplay::from(&raw_wasm_blob)).into_bytes()
            };

            if let Some(output) = &params.output {
                std::fs::write(output, output_buf)?;
            } else {
                std::io::stdout().write_all(&output_buf)?;
            }

            Ok(())
        }
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        #[cfg(feature = "runtime-benchmarks")]
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            // Switch on the concrete benchmark sub-command-
            match cmd {
                BenchmarkCmd::Pallet(cmd) => {
                    if cfg!(feature = "runtime-benchmarks") {
                        runner.sync_run(|config| cmd.run::<Block, GafiRuntimeExecutor>(config))
                    } else {
                        Err("Benchmarking wasn't enabled when building the node. \
					You can enable it with `--features runtime-benchmarks`."
                            .into())
                    }
                }
                BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
                    let partials = new_partial::<RuntimeApi, GafiRuntimeExecutor, _>(
                        &config,
                        gafi_service::gari_build_import_queue,
                    )?;
                    cmd.run(partials.client)
                }),
                BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
                    let partials = new_partial::<RuntimeApi, GafiRuntimeExecutor, _>(
                        &config,
                        gafi_service::gari_build_import_queue,
                    )?;
                    let db = partials.backend.expose_db();
                    let storage = partials.backend.expose_storage();

                    cmd.run(config, partials.client.clone(), db, storage)
                }),
                BenchmarkCmd::Overhead(_) => Err("Unsupported benchmarking command".into()),
                BenchmarkCmd::Machine(cmd) => {
                    return runner
                        .sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()));
                }
            }
        }
        Some(Subcommand::TryRuntime(cmd)) => {
            if cfg!(feature = "try-runtime") {
                let runner = cli.create_runner(cmd)?;

                // grab the task manager.
                let registry = &runner
                    .config()
                    .prometheus_config
                    .as_ref()
                    .map(|cfg| &cfg.registry);
                let task_manager =
                    TaskManager::new(runner.config().tokio_handle.clone(), *registry)
                        .map_err(|e| format!("Error: {:?}", e))?;

                runner.async_run(|config| {
                    Ok((cmd.run::<Block, GafiRuntimeExecutor>(config), task_manager))
                })
            } else {
                Err("Try-runtime must be enabled by `--features try-runtime`.".into())
            }
        }
        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;
            let collator_options = cli.run.collator_options();
            runner.run_node_until_exit(|config| async move {
                let para_id = gafi_chain_spec::gari::Extensions::try_get(&*config.chain_spec)
                    .map(|e| e.para_id)
                    .ok_or_else(|| "Could not find parachain ID in chain-spec.")?;

                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relay_chain_args.iter()),
                );

                let id = ParaId::from(para_id);

                let parachain_account =
                    AccountIdConversion::<polkadot_primitives::v2::AccountId>::try_into_account(
                        &id,
                    );

                let state_version =
                    RelayChainCli::native_runtime_version(&config.chain_spec).state_version();
                let block: Block = generate_genesis_block(&*config.chain_spec, state_version)
                    .map_err(|e| format!("{:?}", e))?;
                let genesis_state = format!("0x{:?}", HexDisplay::from(&block.header().encode()));

                let tokio_handle = config.tokio_handle.clone();
                let polkadot_config =
                    SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
                        .map_err(|err| format!("Relay chain argument error: {}", err))?;

                info!("Parachain id: {:?}", id);
                if let Some(account) = parachain_account {
                    info!("Parachain Account: {}", account);
                } else {
                    info!("Can not get Parachain Account");
                }
                info!("Parachain genesis state: {}", genesis_state);
                info!(
                    "Is collating: {}",
                    if config.role.is_authority() {
                        "yes"
                    } else {
                        "no"
                    }
                );

                gafi_service::start_node::<
				gafi_service::gari_runtime::RuntimeApi,
				gafi_service::GafiRuntimeExecutor,
			>(config, polkadot_config, collator_options, id)
                    .await
                    .map(|r| r.0)
                    .map_err(Into::into)
            })
        }
    }
}

/// Parse command line arguments into service configuration.
#[cfg(feature = "with-gaki")]
pub fn run_gaki() -> Result<()> {
    use gafi_service::gaki_runtime::RuntimeApi;
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            return runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = gafi_service::new_partial::<RuntimeApi, GafiRuntimeExecutor, _>(
                    &config,
                    gafi_service::gaki_build_import_queue,
                )?;
                Ok((cmd.run(client, import_queue), task_manager))
            });
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = gafi_service::new_partial::<RuntimeApi, GafiRuntimeExecutor, _>(
                    &config,
                    gafi_service::gaki_build_import_queue,
                )?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            return runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = gafi_service::new_partial::<RuntimeApi, GafiRuntimeExecutor, _>(
                    &config,
                    gafi_service::gaki_build_import_queue,
                )?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            });
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            return runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = gafi_service::new_partial::<RuntimeApi, GafiRuntimeExecutor, _>(
                    &config,
                    gafi_service::gaki_build_import_queue,
                )?;
                Ok((cmd.run(client, import_queue), task_manager))
            });
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            return runner.sync_run(|config| {
                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relay_chain_args.iter()),
                );

                let polkadot_config = SubstrateCli::create_configuration(
                    &polkadot_cli,
                    &polkadot_cli,
                    config.tokio_handle.clone(),
                )
                .map_err(|err| format!("Relay chain argument error: {}", err))?;

                cmd.run(config, polkadot_config)
            });
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            return runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    backend,
                    ..
                } = gafi_service::new_partial::<RuntimeApi, GafiRuntimeExecutor, _>(
                    &config,
                    gafi_service::gaki_build_import_queue,
                )?;
                let aux_revert = Box::new(|client, _, blocks| {
                    sc_finality_grandpa::revert(client, blocks)?;
                    Ok(())
                });
                Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
            });
        }
        Some(Subcommand::ExportGenesisState(params)) => {
            let mut builder = sc_cli::LoggerBuilder::new("");
            builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
            let _ = builder.init();

            let spec = load_spec(&params.chain.clone().unwrap_or_default())?;
            let state_version = Cli::native_runtime_version(&spec).state_version();
            let block: Block = generate_genesis_block(&*spec, state_version)?;
            let raw_header = block.header().encode();
            let output_buf = if params.raw {
                raw_header
            } else {
                format!("0x{:?}", HexDisplay::from(&block.header().encode())).into_bytes()
            };

            if let Some(output) = &params.output {
                std::fs::write(output, output_buf)?;
            } else {
                std::io::stdout().write_all(&output_buf)?;
            }

            Ok(())
        }
        Some(Subcommand::ExportGenesisWasm(params)) => {
            let mut builder = sc_cli::LoggerBuilder::new("");
            builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
            let _ = builder.init();

            let raw_wasm_blob =
                extract_genesis_wasm(&cli.load_spec(&params.chain.clone().unwrap_or_default())?)?;
            let output_buf = if params.raw {
                raw_wasm_blob
            } else {
                format!("0x{:?}", HexDisplay::from(&raw_wasm_blob)).into_bytes()
            };

            if let Some(output) = &params.output {
                std::fs::write(output, output_buf)?;
            } else {
                std::io::stdout().write_all(&output_buf)?;
            }

            Ok(())
        }
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        #[cfg(feature = "runtime-benchmarks")]
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            // Switch on the concrete benchmark sub-command-
            match cmd {
                BenchmarkCmd::Pallet(cmd) => {
                    if cfg!(feature = "runtime-benchmarks") {
                        runner.sync_run(|config| cmd.run::<Block, GafiRuntimeExecutor>(config))
                    } else {
                        Err("Benchmarking wasn't enabled when building the node. \
					You can enable it with `--features runtime-benchmarks`."
                            .into())
                    }
                }
                BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
                    let partials = new_partial::<RuntimeApi, GafiRuntimeExecutor, _>(
                        &config,
                        gafi_service::gaki_build_import_queue,
                    )?;
                    cmd.run(partials.client)
                }),
                BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
                    let partials = new_partial::<RuntimeApi, GafiRuntimeExecutor, _>(
                        &config,
                        gafi_service::gaki_build_import_queue,
                    )?;
                    let db = partials.backend.expose_db();
                    let storage = partials.backend.expose_storage();

                    cmd.run(config, partials.client.clone(), db, storage)
                }),
                BenchmarkCmd::Overhead(_) => Err("Unsupported benchmarking command".into()),
                BenchmarkCmd::Machine(cmd) => {
                    return runner
                        .sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()));
                }
            }
        }
        Some(Subcommand::TryRuntime(cmd)) => {
            if cfg!(feature = "try-runtime") {
                let runner = cli.create_runner(cmd)?;

                // grab the task manager.
                let registry = &runner
                    .config()
                    .prometheus_config
                    .as_ref()
                    .map(|cfg| &cfg.registry);
                let task_manager =
                    TaskManager::new(runner.config().tokio_handle.clone(), *registry)
                        .map_err(|e| format!("Error: {:?}", e))?;

                runner.async_run(|config| {
                    Ok((cmd.run::<Block, GafiRuntimeExecutor>(config), task_manager))
                })
            } else {
                Err("Try-runtime must be enabled by `--features try-runtime`.".into())
            }
        }
        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;
            let collator_options = cli.run.collator_options();
            runner.run_node_until_exit(|config| async move {
                let para_id = gafi_chain_spec::gaki::Extensions::try_get(&*config.chain_spec)
                    .map(|e| e.para_id)
                    .ok_or_else(|| "Could not find parachain ID in chain-spec.")?;

                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relay_chain_args.iter()),
                );

                let id = ParaId::from(para_id);

                let parachain_account =
                    AccountIdConversion::<polkadot_primitives::v2::AccountId>::try_into_account(
                        &id,
                    );

                let state_version =
                    RelayChainCli::native_runtime_version(&config.chain_spec).state_version();
                let block: Block = generate_genesis_block(&*config.chain_spec, state_version)
                    .map_err(|e| format!("{:?}", e))?;
                let genesis_state = format!("0x{:?}", HexDisplay::from(&block.header().encode()));

                let tokio_handle = config.tokio_handle.clone();
                let polkadot_config =
                    SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
                        .map_err(|err| format!("Relay chain argument error: {}", err))?;

                info!("Parachain id: {:?}", id);
                if let Some(account) = parachain_account {
                    info!("Parachain Account: {}", account);
                } else {
                    info!("Can not get Parachain Account");
                }
                info!("Parachain genesis state: {}", genesis_state);
                info!(
                    "Is collating: {}",
                    if config.role.is_authority() {
                        "yes"
                    } else {
                        "no"
                    }
                );

                gafi_service::start_node::<
				// temporary put this to prevent error
				gafi_service::gari_runtime::RuntimeApi,
				gafi_service::GafiRuntimeExecutor,
			>(config, polkadot_config, collator_options, id)
                    .await
                    .map(|r| r.0)
                    .map_err(Into::into)
            })
        }
    }
}

impl DefaultConfigurationValues for RelayChainCli {
    fn p2p_listen_port() -> u16 {
        30334
    }

    fn rpc_ws_listen_port() -> u16 {
        9945
    }

    fn rpc_http_listen_port() -> u16 {
        9934
    }

    fn prometheus_listen_port() -> u16 {
        9616
    }
}

impl CliConfiguration<Self> for RelayChainCli {
    fn shared_params(&self) -> &SharedParams {
        self.base.base.shared_params()
    }

    fn import_params(&self) -> Option<&ImportParams> {
        self.base.base.import_params()
    }

    fn network_params(&self) -> Option<&NetworkParams> {
        self.base.base.network_params()
    }

    fn keystore_params(&self) -> Option<&KeystoreParams> {
        self.base.base.keystore_params()
    }

    fn base_path(&self) -> Result<Option<BasePath>> {
        Ok(self
            .shared_params()
            .base_path()
            .or_else(|| self.base_path.clone().map(Into::into)))
    }

    fn rpc_http(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
        self.base.base.rpc_http(default_listen_port)
    }

    fn rpc_ipc(&self) -> Result<Option<String>> {
        self.base.base.rpc_ipc()
    }

    fn rpc_ws(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
        self.base.base.rpc_ws(default_listen_port)
    }

    fn prometheus_config(
        &self,
        default_listen_port: u16,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<PrometheusConfig>> {
        self.base
            .base
            .prometheus_config(default_listen_port, chain_spec)
    }

    fn init<F>(
        &self,
        _support_url: &String,
        _impl_version: &String,
        _logger_hook: F,
        _config: &sc_service::Configuration,
    ) -> Result<()>
    where
        F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
    {
        unreachable!("PolkadotCli is never initialized; qed");
    }

    fn chain_id(&self, is_dev: bool) -> Result<String> {
        let chain_id = self.base.base.chain_id(is_dev)?;

        Ok(if chain_id.is_empty() {
            self.chain_id.clone().unwrap_or_default()
        } else {
            chain_id
        })
    }

    fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
        self.base.base.role(is_dev)
    }

    fn transaction_pool(&self) -> Result<sc_service::config::TransactionPoolOptions> {
        self.base.base.transaction_pool()
    }

    fn state_cache_child_ratio(&self) -> Result<Option<usize>> {
        self.base.base.state_cache_child_ratio()
    }

    fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
        self.base.base.rpc_methods()
    }

    fn rpc_ws_max_connections(&self) -> Result<Option<usize>> {
        self.base.base.rpc_ws_max_connections()
    }

    fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
        self.base.base.rpc_cors(is_dev)
    }

    fn default_heap_pages(&self) -> Result<Option<u64>> {
        self.base.base.default_heap_pages()
    }

    fn force_authoring(&self) -> Result<bool> {
        self.base.base.force_authoring()
    }

    fn disable_grandpa(&self) -> Result<bool> {
        self.base.base.disable_grandpa()
    }

    fn max_runtime_instances(&self) -> Result<Option<usize>> {
        self.base.base.max_runtime_instances()
    }

    fn announce_block(&self) -> Result<bool> {
        self.base.base.announce_block()
    }

    fn telemetry_endpoints(
        &self,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
        self.base.base.telemetry_endpoints(chain_spec)
    }

    fn node_name(&self) -> Result<String> {
        self.base.base.node_name()
    }
}
