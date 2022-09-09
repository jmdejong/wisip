mod tcpserver;
mod unixserver;
mod websocketserver;
mod address;

use std::io;
use enum_dispatch::enum_dispatch;

use crate::util::HolderId;
pub use tcpserver::TcpServer;
pub use unixserver::UnixServer;
pub use websocketserver::WebSocketServer;
pub use address::Address;

mod streamconnection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionId(pub usize);

impl HolderId for ConnectionId {
	fn next(&self) -> Self { Self(self.0 + 1) }
	fn initial() -> Self { Self(1) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
	pub connection: ConnectionId,
	pub content: String
}

#[derive(Debug, Clone)]
pub struct MessageUpdates {
	pub messages: Vec<Message>,
	pub to_remove: Vec<ConnectionId>
}

#[derive(Debug)]
pub enum ConnectionError {
	IO(io::Error),
	InvalidIndex(ConnectionId),
	Tungstenite(tungstenite::Error),
	Custom(String)
}



#[enum_dispatch]
pub trait Server {
	
	fn accept_pending_connections(&mut self) -> Vec<ConnectionId>;
	
	fn recv_pending_messages(&mut self) -> MessageUpdates;
	
	fn send(&mut self, id: ConnectionId, text: &str) -> Result<(), ConnectionError>;
	
	fn broadcast(&mut self, text: &str);
	
	fn get_name(&self, _id: ConnectionId) -> Option<String> {
		None
	}
}

#[enum_dispatch(Server)]
pub enum ServerEnum {
	TcpServer,
	UnixServer,
	WebSocketServer
}


