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
use serde::{Serialize, Deserialize};
use std::io;
use std::io::{BufReader, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Move {
	direction: Direction,
	distance: u32
}

#[derive(Serialize, Deserialize, Debug)]
enum Direction {
	Left,
	Right,
	Forward,
	Backward
}

// Serialize and deserialize a data structure with serde (JSON).
fn main() -> io::Result<()> {
	let a = Move {
		direction: Direction::Forward,
		distance: 15
	};
	
	println!("a = {:?}", a);
	let sdata = serde_json::to_string(&a)?;
	let mut file = File::create("files/ex_2_1_move_data.json")?;
	// Write the serialized Move data
	let n = file.write(sdata.as_bytes())?;
	println!("{} byte written.", n);
	file.flush().unwrap();
	drop(file);
	// Read back the Move data
	let file = File::open("files/ex_2_1_move_data.json")?;
	let reader = BufReader::new(file);
	let b: Move = serde_json::from_reader(reader)?;
	println!("b = {:?}", b);
	Ok(())
}
