
use clap::{Parser, Subcommand, Args};

use crate::{
	server::Address,
};

#[derive(Debug, Parser)]
#[command(name = "Dezl", version, author, about)]
pub struct Config {
	
	#[command(subcommand)]
	pub world_action: WorldAction,
}


#[derive(Debug, Subcommand)]
pub enum WorldAction {
	/// Load existing world
	Load(WorldConfig),
	/// Create new world
	New(WorldConfig)
}

#[derive(Debug, Args)]
pub struct WorldConfig {
	/// The name of the world
	pub name: String,

	/// A server type and address. Allowed server types: 'inet', 'unix', 'abstract'.
	/// Example: "inet:127.0.0.1:1234" or "abstract:dezl" or "unix:/tmp/dezl" or "inet:[::1]:1234"
	#[arg(short, long)]
	pub address: Option<Vec<Address>>,

	/// The name(s) of the server admin(s)
	#[arg(long, env="USER")]
	pub admins: String,

	/// The time (in milliseconds) between two steps
	#[arg(long, default_value_t=100)]
	pub step_duration: u64,
}
