use clap::{Args, Parser, Subcommand};
use config::{Config};
use serde_derive::Deserialize;

mod idrac;
mod chassis;
mod lib;
mod storage;
mod network;

use crate::idrac::get_idrac_version::get_idrac_version;

use crate::chassis::get_chassis_info::get_chassis_info;

use crate::network::list_network_adapters::list_network_adapters;
use crate::network::get_network_adapter::get_network_adapter;
use crate::network::get_network_port::get_network_port;

use crate::storage::get_storage_controller::get_storage_controller;
use crate::storage::get_storage_disk::get_storage_disk;
use crate::storage::get_storage_volume::get_storage_volume;
use crate::storage::list_storage_options::list_storage_controllers;
use crate::storage::list_storage_volumes::list_storage_volumes;

/// A simple command line interface for interacting with iDRAC
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "idrac-cli")]
#[command(about = "A simple command line interface for interacting with iDRAC", long_about = None)]
struct Opts {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Idrac(Idrac),
    Chassis(Chassis),
    Network(Network),
    Storage(Storage),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Idrac {
    #[command(subcommand)]
    command: Option<IdracCommands>,
}

#[derive(Debug, Subcommand)]
enum IdracCommands {
    Version
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Chassis {
    #[command(subcommand)]
    command: Option<ChassisCommands>,
}

#[derive(Debug, Subcommand)]
enum ChassisCommands {
    Info
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Network {
    #[command(subcommand)]
    command: Option<NetworkCommands>,
}

#[derive(Debug, Subcommand)]
enum NetworkCommands {
    ListNetworkAdapters,
    GetNetworkAdapter(NetworkAdapter),
    GetNetworkPort(NetworkPort)
}

#[derive(Debug, Args)]
struct NetworkAdapter {
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short, long)]
    detailed: bool,
}

#[derive(Debug, Args)]
struct NetworkPort {
    #[arg(short, long)]
    adapter: Option<String>,
    #[arg(short, long)]
    port: Option<String>,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Storage {
    #[command(subcommand)]
    command: Option<StorageCommands>,
}

#[derive(Debug, Subcommand)]
enum StorageCommands {
    ListStorageControllers,
    GetStorageController(StorageController),
    ListStorageVolumes(StorageVolumes),
    GetStorageVolume(StorageVolume),
    GetStorageDisk(StorageDisk)
}

#[derive(Debug, Args)]
struct StorageController {
    #[arg(short, long)]
    name: Option<String>,
}

#[derive(Debug, Args)]
struct StorageVolumes {
    #[arg(short, long)]
    controller: Option<String>,
}

#[derive(Debug, Args)]
struct StorageVolume {
    #[arg(short, long)]
    name: Option<String>,
}

#[derive(Debug, Args)]
struct StorageDisk {
    #[arg(short, long)]
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    host: String,
    user: String,
    password: String
}

fn make() -> Config {
    Config::builder()
        .add_source(config::File::with_name("./settings"))
        .build()
        .unwrap()
}

fn main() {
    let opts: Opts = Opts::parse();
    let c = make();
    let settings: Settings = c.try_deserialize().unwrap();

    match &opts.command {
        Commands::Idrac(idrac) => {
            match idrac.command.as_ref().unwrap() {
                IdracCommands::Version => get_idrac_version(settings).expect("Panic!")
            }
        }
        Commands::Chassis(chassis) => {
            match chassis.command.as_ref().unwrap() {
                ChassisCommands::Info => get_chassis_info(settings).expect("Panic!"),
            }
        }
        Commands::Network(network) => {
            match network.command.as_ref().unwrap() {
                NetworkCommands::ListNetworkAdapters => list_network_adapters(settings).expect("Panic!"),
                NetworkCommands::GetNetworkAdapter(network_adapter) => {
                    if network_adapter.detailed {
                        get_network_adapter(&network_adapter.name, settings, true).expect("Panic!")
                    } else {
                        get_network_adapter(&network_adapter.name, settings, false).expect("Panic!")
                    }
                },
                NetworkCommands::GetNetworkPort(network_port) => get_network_port(network_port.adapter.as_ref().unwrap(), network_port.port.as_ref().unwrap(), &settings),
            }
        }
        Commands::Storage(storage) => {
            match storage.command.as_ref().unwrap() {
                StorageCommands::ListStorageControllers => list_storage_controllers(settings).expect("Panic!"),
                StorageCommands::GetStorageController(storage_controller) => get_storage_controller(&storage_controller.name, settings).expect("Panic!"),
                StorageCommands::ListStorageVolumes(storage_volume) => list_storage_volumes(&storage_volume.controller, settings).expect("Panic!"),
                StorageCommands::GetStorageVolume(storage_volume) => get_storage_volume(&storage_volume.name, settings).expect("Panic!"),
                StorageCommands::GetStorageDisk(storage_disk) => get_storage_disk(&storage_disk.name, settings).expect("Panic!"),
            }
        }
    }
}