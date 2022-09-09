use std::io;

pub mod tcpserver;
pub mod unixserver;
pub mod websocketserver;
pub mod address;
pub mod holder;

mod streamconnection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionId(pub usize);

impl holder::HolderId for ConnectionId {
	fn next(&self) -> Self { ConnectionId(self.0 + 1) }
	fn initial() -> Self { ConnectionId(1) }
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


pub trait Server {
	
	fn accept_pending_connections(&mut self) -> Vec<ConnectionId>;
	
	fn recv_pending_messages(&mut self) -> MessageUpdates;
	
	fn send(&mut self, id: ConnectionId, text: &str) -> Result<(), ConnectionError>;
	
	fn broadcast(&mut self, text: &str);
	
	fn get_name(&self, _id: ConnectionId) -> Option<String> {
		None
	}
}



