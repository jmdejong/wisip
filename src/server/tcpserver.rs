

use std::io;
use std::net::SocketAddr;
use mio::net::{TcpListener, TcpStream};
use crate::util::Holder;

use super::{
	connection::Connection,
	Server,
	ConnectionId,
	Message,
	MessageUpdates,
	ServerError
};


pub struct TcpServer<T: Connection<TcpStream>> {
	listener: TcpListener,
	connections: Holder<ConnectionId, T>
}

impl <T: Connection<TcpStream>> TcpServer<T> {

	pub fn new(addr: SocketAddr) -> Result<TcpServer<T>, io::Error> {
		let listener = TcpListener::bind(addr)?;
		Ok( TcpServer {
			listener,
			connections: Holder::new()
		})
	}
}

impl <T: Connection<TcpStream>> Server for TcpServer<T> {

	fn accept_pending_connections(&mut self) -> Vec<ConnectionId> {
		let mut new_connections = Vec::new();
		while let Ok((stream, _address)) = self.listener.accept() {
			let con = Connection::new(stream).unwrap();
			let id = self.connections.insert(con);
			new_connections.push(id);
		}
		new_connections
	}


	fn recv_pending_messages(&mut self) -> MessageUpdates {
		let mut messages: Vec<Message> = Vec::new();
		let mut to_remove: Vec<ConnectionId> = Vec::new();
		for (connection_id, connection) in self.connections.iter_mut(){
			match connection.read() {
				Err(_e) => {
					to_remove.push(*connection_id);
				}
				Ok((con_messages, closed)) => {
					for message in con_messages {
						messages.push(Message{connection: *connection_id, content: message});
					}
					if closed {
						to_remove.push(*connection_id);
					}
				}
			}
		}
		for key in to_remove.iter() {
			self.connections.remove(key);
		}
		MessageUpdates{messages, to_remove}
	}

	fn broadcast(&mut self, text: &str) {
		for (_id, conn) in self.connections.iter_mut() {
			let _ = conn.send(text);
		}
	}
	
	fn send(&mut self, id: ConnectionId, text: &str) -> Result<(), ServerError> {
		match self.connections.get_mut(&id){
			Some(conn) => {
				conn.send(text).map_err(ServerError::Connection)
			}
			None => Err(ServerError::InvalidIndex(id))
		}
	}
	

}

