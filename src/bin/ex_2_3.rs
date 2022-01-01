/*
 * This file is part of pna-rust-exercises.
 * Copyright (c) 2021 Joe Ma <rikkaneko23@gmail.com>
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

use std::fs::File;
use serde::{Deserialize, Serialize};
use rand::distributions::{Distribution, Uniform};
use std::io;
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Move {
	direction: Direction,
	distance: u32
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Direction {
	Left,
	Right,
	Forward,
	Backward,
	FallOutOfTheWorld
}

impl From<u8> for Direction {
	fn from(n: u8) -> Self {
		match n {
			0 => Direction::Left,
			1 => Direction::Right,
			2 => Direction::Forward,
			3 => Direction::Backward,
			_ => Direction::FallOutOfTheWorld
		}
	}
}

fn main() -> io::Result<()> {
	let mut records = Vec::with_capacity(1000);
	let mut rng = rand::thread_rng();
	let draw1 = Uniform::from(0..=4);
	let draw2 = Uniform::from(0..=128);
	let file = File::create("files/ex_2_3_data.bson")?;
	let mut writer = BufWriter::new(file);
	// Generate 1000 Move object
	for _ in 0..1000 {
		let record = Move {
			direction: Direction::from(draw1.sample(&mut rng)),
			distance: draw2.sample(&mut rng)
		};
		// println!("{:?}", record);
		records.push(record);
	}
	// Serialize all Move object in the Vector
	for record in records.iter() {
		writer.write_all(bson::to_vec(record).unwrap().as_slice())?;
	}
	drop(writer);
	// Deserialize all Move object from file
	let file = File::open("files/ex_2_3_data.bson")?;
	let mut reader = BufReader::new(file);
	let mut result: Vec<Move> = Vec::new();
	while let Ok(record) = bson::from_reader(&mut reader) {
		result.push(record);
	}
	
	// for record in result.iter() {
	// 	println!("{:?}", record);
	// }
	
	assert_eq!(result, records);
	
	Ok(())
}
