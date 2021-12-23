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

use serde::{Serialize, Deserialize};

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

// Serialize and deserialize a data structure to a buffer with serde (RON).
fn main() {
	let a = Move {
		direction: Direction::Forward,
		distance: 15
	};
	
	println!("a = {:?}", a);
	let sdata = ron::to_string(&a).unwrap();
	let buf = sdata.into_bytes();
	let ddata = String::from_utf8(buf).unwrap();
	println!("ddata = {}", ddata);
	let b: Move = ron::from_str(&ddata).unwrap();
	print!("b = {:?}", b);
}
