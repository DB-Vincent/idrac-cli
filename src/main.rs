use clap::{Args, Parser, Subcommand};
use config::{Config};
use serde_derive::Deserialize;

mod idrac;
mod chassis;

use crate::idrac::get_idrac_version::get_idrac_version;

use crate::chassis::list_network_adapters::list_network_adapters;
use crate::chassis::get_chassis_info::get_chassis_info;
use crate::chassis::get_network_adapter::get_network_adapter;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "idrac-cli")]
#[command(about = "A simple command line interface for interacting with iDrac", long_about = None)]
struct Opts {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Idrac(Idrac),
    Chassis(Chassis)
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
    Info,
    ListNetworkAdapters,
    GetNetworkAdapter(NetworkAdapter)
}

#[derive(Debug, Args)]
struct NetworkAdapter {
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
                ChassisCommands::ListNetworkAdapters => list_network_adapters(settings).expect("Panic!"),
                ChassisCommands::GetNetworkAdapter(network_adapter) => get_network_adapter(&network_adapter.name, settings).expect("Panic!")
            }
        }
    }
}