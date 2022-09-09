
use std::path::PathBuf;
use std::net::SocketAddr;
use std::str::FromStr;
use crate::{
	Result,
	aerr,
	errors::AnyError
};
use super::{
	tcpserver::TcpServer,
	unixserver::UnixServer,
	websocketserver::WebSocketServer,
	Server
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Address {
	Inet(SocketAddr),
	Unix(PathBuf),
	Web(SocketAddr)
}

impl Address {
	pub fn to_server(&self) -> Result<Box<dyn Server>> {
		match self {
			Address::Inet(addr) => Ok(Box::new(TcpServer::new(addr.clone())?)),
			Address::Unix(path) => Ok(Box::new(UnixServer::new(path)?)),
			Address::Web(addr) => Ok(Box::new(WebSocketServer::new(addr.clone())?))
		}
	}
}

impl FromStr for Address {
	type Err = AnyError;
	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		let parts: Vec<&str> = s.splitn(2, ':').collect();
		if parts.len() != 2 {
			return Err(aerr!("Address string has the wrong length!"));
		}
		let typename = parts[0];
		let text = parts[1];
		match typename {
			"inet" => Ok(Address::Inet(text.parse().map_err(|e| aerr!("'{}' is not a valid inet address: {}", text, e))?)),
			"unix" => Ok(Address::Unix(PathBuf::new().join(text))),
			"abstract" => {
					if cfg!(target_os = "linux") {
						Ok(Address::Unix(PathBuf::new().join(&format!("\0{}", text))))
					} else {
						Err(aerr!("abstract adresses are only for linux"))
					}
				}
			"web" => Ok(Address::Web(text.parse().map_err(|e| aerr!("'{}' is not a valid websocket address: {}", text, e))?)),
			_ => Err(aerr!("'{}' is not a valid address type", typename))
		}
	}
}
