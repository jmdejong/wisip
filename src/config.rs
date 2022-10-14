
use clap::Parser;

use crate::{
	server::Address,
};

#[derive(Debug, Parser)]
#[command(name = "Dezl", version, author, about)]
pub struct Config {
	
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
