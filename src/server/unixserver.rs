

use std::io;
use std::path::Path;
use std::os::unix::io::AsRawFd;
use mio::net::{UnixListener, UnixStream};
use nix::sys::socket::getsockopt;
use nix::sys::socket::sockopt;
use crate::util::Holder;

use super::{
	connection::{Connection, StreamConnection},
	Server,
	ConnectionId,
	Message,
	MessageUpdates,
	ServerError
};


pub struct UnixServer {
	listener: UnixListener,
	connections: Holder<ConnectionId, (StreamConnection<UnixStream>, std::os::unix::io::RawFd)>
}

impl UnixServer {

	pub fn new(addr: &Path) -> Result<UnixServer, io::Error> {
		let listener = UnixListener::bind(addr)?;
		Ok( UnixServer {
			listener,
			connections: Holder::new()
		})
	}
	
	
}

impl Server for UnixServer {

	fn accept_pending_connections(&mut self) -> Vec<ConnectionId> {
		let mut new_connections = Vec::new();
		while let Ok((stream, _address)) = self.listener.accept() {
			let fd = stream.as_raw_fd();
			let con = StreamConnection::new(stream).unwrap();
			let id = self.connections.insert((con, fd));
			new_connections.push(id);
		}
		new_connections
	}


	fn recv_pending_messages(&mut self) -> MessageUpdates{
		let mut messages: Vec<Message> = Vec::new();
		let mut to_remove: Vec<ConnectionId> = Vec::new();
		for (connection_id, (connection, _fd)) in self.connections.iter_mut(){
			match connection.read() {
				Err(_e) => {
					to_remove.push(*connection_id);
				}
				Ok((con_messages, closed)) => {
					for message in con_messages {
						messages.push(Message{connection: *connection_id, content: message})
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
		for (_id, (conn, _fd)) in self.connections.iter_mut() {
			let _ = conn.send(text);
		}
	}
	
	fn send(&mut self, id: ConnectionId, text: &str) -> Result<(), ServerError> {
		match self.connections.get_mut(&id){
			Some((conn, _fd)) => {
				conn.send(text).map_err(ServerError::Connection)
			}
			None => Err(ServerError::InvalidIndex(id))
		}
	}
	
	#[cfg(any(target_os = "linux", target_os = "android"))]
	fn get_name(&self, id: ConnectionId) -> Option<String> {
		let (_conn, fd) = self.connections.get(&id)?;
// 		let fd = connection.stream.as_raw_fd();
		let peercred = getsockopt(*fd, sockopt::PeerCredentials).ok()?;
		let uid = peercred.uid();
		let user = users::get_user_by_uid(uid)?;
		let name = user.name();
		Some(name.to_string_lossy().to_string())
	}
	
	#[cfg(not(any(target_os = "linux", target_os = "android")))]
	fn get_name(&self, id: ConnectionId) -> Option<String> {
		None
	}
}

