
use structopt::StructOpt;
use crate::{
	server::Address,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "Dezl", about = "Experimental ascii mmorpg")]
pub struct Config {
	
	#[structopt(short, long, help="A server type and address. Allowed server types: 'inet', 'unix', 'abstract'. Example: \"inet:127.0.0.1:1234\" or \"abstract:dezl\" or \"unix:/tmp/dezl\" or \"inet:[::1]:1234\"")]
	pub address: Option<Vec<Address>>,
	
	#[structopt(long, env="USER", help="The name(s) of the server admin(s)")]
	pub admins: String,
	
	#[structopt(long, default_value="100", help="The time (in milliseconds) between two steps")]
	pub step_duration: u64,
	
	
}
