

use std::io;
use std::net::SocketAddr;
use mio::net::{TcpListener, TcpStream};
use tungstenite::{
	accept,
	Message as WsMessage,
	WebSocket,
	error::Error as WsError
};

use super::{
	Server,
	ConnectionId,
	Message,
	MessageUpdates,
	ConnectionError,
	holder::Holder
};

fn is_wouldblock_error(error: &WsError) -> bool {
	if let WsError::Io(io_err) = error {
		io_err.kind() == std::io::ErrorKind::WouldBlock 
	} else {
		false
	}
}

pub struct WebSocketServer {
	listener: TcpListener,
	connections: Holder<ConnectionId, WebSocket<TcpStream>>
}

impl WebSocketServer {

	pub fn new(addr: SocketAddr) -> Result<Self, io::Error> {
		let listener = TcpListener::bind(addr)?;
		Ok( Self {
			listener,
			connections: Holder::new()
		})
	}
}

impl Server for WebSocketServer {

	fn accept_pending_connections(&mut self) -> Vec<ConnectionId> {
		let mut new_connections = Vec::new();
		loop {
			match self.listener.accept() {
				Err(_e) => {
					break;
				}
				Ok((stream, _address)) => {
					match accept(stream) {
						Ok(websocket) => {
							let id = self.connections.insert(websocket);
							new_connections.push(id);
						}
						Err(err) => {
							println!("failed to open websocket connection: {:?}", err);
						}
					}
				}
			}
		}
		new_connections
	}


	fn recv_pending_messages(&mut self) -> MessageUpdates {
		let mut messages: Vec<Message> = Vec::new();
		let mut to_remove: Vec<ConnectionId> = Vec::new();
		for (connection_id, connection) in self.connections.iter_mut(){
			match connection.read_message() {
				Err(e) => {
					if is_wouldblock_error(&e) {
						break;
					}
					println!("error reading websocket message: {:?}", e);
					to_remove.push(*connection_id);
					connection.close(None);
				}
				Ok(WsMessage::Text(text)) => {
					println!("websocket text: {}", text.clone());
					messages.push(Message{connection: *connection_id, content: text});
				}
				Ok(WsMessage::Close(_)) => {
					println!("websocket close");
					to_remove.push(*connection_id);
				}
				Ok(_) => {
					println!("websocket other");
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
			let _ = conn.write_message(WsMessage::Text(text.to_string()));
		}
	}
	
	fn send(&mut self, id: ConnectionId, text: &str) -> Result<(), ConnectionError> {
		match self.connections.get_mut(&id){
			Some(conn) => {
				conn.write_message(WsMessage::Text(text.to_string())).map_err(|err| ConnectionError::Tungstenite(err))
			}
			None => Err(ConnectionError::InvalidIndex(id))
		}
	}
}

