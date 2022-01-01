/*
 * This file is part of pna-rust-exercises.
 * Copyright (c) 2022 Joe Ma <rikkaneko23@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
#![allow(clippy::manual_flatten)]
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use thiserror::Error;
type Result<T> = std::result::Result<T, BundledError>;

#[derive(Error, Debug)]
enum BundledError {
	#[error("I/O error")]
	IOError(#[from] std::io::Error),
	#[error("Parse error")]
	ParseError(#[from] std::num::ParseIntError)
}

fn handle_request(stream: &mut TcpStream) -> Result<String> {
	let mut buf = [0; 1024];
	let len = stream.read(&mut buf)?;
	let request = String::from_utf8_lossy(&buf[..len]);
	// Redis inline command
	let cmd = request.chars().take_while(|&ch| !ch.is_ascii_whitespace()).collect::<String>();
	if cmd.is_empty() { return Ok(String::new()) }
	if cmd != "PING" { return Ok(format!("Command \"{}\" does not exist.\n", cmd)) }
	else {
		let argument = request.chars().skip(4).skip_while(|&ch| ch.is_ascii_whitespace()).collect::<String>();
		if argument.is_empty() { return Ok("PONG\n".to_string()) }
		Ok(argument)
	}
}

fn main() -> Result<()> {
	let listener = TcpListener::bind("127.0.0.1:6379")?;
	for stream in listener.incoming() {
		if let Ok(mut stream) = stream {
			println!("Client connected from {}", stream.peer_addr()?);
			let reply = handle_request(&mut stream)?;
			stream.write_all(reply.as_bytes())?;
			stream.flush()?;
		}
	}
	Ok(())
}
