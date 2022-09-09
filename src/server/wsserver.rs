

use std::io;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::io::{Read, Write};
use mio::net::{TcpListener, TcpStream};

use super::{
	Server,
	ConnectionId,
	Message,
	MessageUpdates,
	ConnectionError,
	holder::Holder
};

enum WSError { Incomplete, Invalid(String)}
use WSError::*;

fn buffer_preread<'a, T>(buffer: &'a[T], index: &mut usize, mid: usize) -> Result<&'a[T], WSError> {
	if *index + mid > buffer.len() {
		Err(Incomplete)
	} else {
		let start: usize = *index;
		let end: usize = *index + mid;
		*index += mid;
		Ok(&buffer[start..end])
	}
}

fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
    where for<'a> &'a [T]: PartialEq
{
    haystack.windows(needle.len()).position(|window| window == needle)
}


enum WSStatus { New, Open, Closed } 

pub struct WSConnection<T: Read+Write> {
	pub stream: T,
	buffer: Vec<u8>,
	status: WSStatus
}

impl <T: Read+Write> WSConnection<T> {
	
	pub fn new(stream: T) -> WSConnection<T> {
		WSConnection {
			stream,
			buffer: Vec::new(),
			status: WSStatus::New
		}
	}
	
	
	pub fn read(&mut self) -> Result<(Vec<String>, bool), io::Error> {
		let mut buf = [0; 2048];
		let mut closed = false;
		let mut stale = true;
		loop {
			match self.stream.read(&mut buf) {
				Err(e) => {
					if e.kind() == io::ErrorKind::WouldBlock {
						break;
					} else {
						return Err(e);
					}
				}
				Ok(0) => {
					closed = true;
					break;
				}
				Ok(i) => {
					self.buffer.extend_from_slice(&buf[..i]);
					stale = false;
				}
			}
		}
		let mut messages = Vec::new();
		if !stale {
			loop {
				if let Ok((len, opcode, bytes)) = self.read_frame() {
					self.buffer = self.buffer[len..self.buffer.len()].into_iter().cloned().collect();
				} else {
					break;
				}
			}
		}
		Ok((messages, closed))
	}
	
	fn read_frame(&self) -> Result<(usize, u8, Vec<u8>), WSError> {
		
		let buffer = &self.buffer;
		let mut ind: usize = 0;
		let typeoctet = buffer_preread(&self.buffer, &mut ind, 1)?[0];
		if typeoctet & 0b1000_0000 == 0 { // bit 0 (fin) not set
			return Err(Invalid("multi frame messages not supported".to_string()));
		} else if typeoctet & 0b0111_0000 != 0 { // bit 1, 2, or 3 (extension) set
			return Err(Invalid("unsupported protocol extension".to_string()));
		}
		let opcode = typeoctet & 0b0000_1111;
		
		let sizeoctet = buffer_preread(&self.buffer, &mut ind, 1)?[0];
		let masked = sizeoctet & 0b1000_0000 == 0b1000_0000;
		if !masked {
			return Err(Invalid("client to server messages must be masked".to_string()));
		}
		let raw_len = sizeoctet & 0b0111_1111;
		let len: usize = if raw_len <= 125 {
				raw_len.into()
			} else if raw_len == 126 {
				let size_header = buffer_preread(&self.buffer, &mut ind, 2)?;
				u16::from_be_bytes(size_header.try_into().unwrap()).into()
			} else if raw_len == 127 {
				let size_header = buffer_preread(&self.buffer, &mut ind, 8)?;
				u64::from_be_bytes(size_header.try_into().unwrap()) as usize
			} else {
				return Err(Invalid(format!("raw_len is {}", raw_len)))
			};
		
		let default_mask = [0; 4];
		let mask = if masked {
				buffer_preread(&self.buffer, &mut ind, 4)?
			} else {
				&default_mask
			};
		
		let payload = buffer_preread(&self.buffer, &mut ind, len)?;
		let bytes = payload.into_iter().cloned().collect::<Vec<u8>>();
		Ok((ind, opcode, bytes))
	}
	
	fn handle_handshake(&mut self) -> Result<(), WSError>{
		let end_sequence = [b'\r', b'\n', b'\r', b'\n'];
		let header_end = find_subsequence(&self.buffer, &end_sequence).ok_or(Incomplete)?;
		let header_text = String::from_utf8_lossy(&self.buffer[..header_end]).to_string();
		let mut header_lines = header_text.split("\r\n");
		let status = header_lines.next().unwrap();
		if !status.trim().starts_with("GET") {
			return Err(Invalid(format!("Expected handshake to start with GET, not '{}'", status)));
		}
		let headers: HashMap<String, &str> = header_lines
			.map(|line| {
				let (key, value) = line.split_once(":").unwrap();
				(key.trim().to_lowercase(), value.trim())
			})
			.collect();
		let websocket_key = headers.get("sec-websocket-key")
			.ok_or(Invalid("Websocket key header missing".to_string()))?;
// 		let mut m = sha1_smol::Sha1::new();
// 		m.update(websocket_key);
		self.buffer = self.buffer[(header_end + end_sequence.len())..self.buffer.len()].into_iter().cloned().collect();
		Ok(())
	}
	
	pub fn send(&mut self, text: &str) -> Result<(), io::Error> {
		let mut header: Vec<u8> = vec![129]; // single frame message, text type, no extensions
		let bytes: &[u8] = text.as_bytes();
		let len = bytes.len();
		if len <= 125 {
			header.push(len as u8);
		} else if len <= 0xffff {
			header.push(126);
			let len_bytes: [u8; 2] = (len as u16).to_be_bytes();
			header.extend_from_slice(&len_bytes);
		} else {
			header.push(127);
			let len_bytes: [u8; 8] = (len as u64).to_be_bytes();
			header.extend_from_slice(&len_bytes);
		}
		self.stream.write_all(&header)?;
		self.stream.write_all(bytes)
	}
}
