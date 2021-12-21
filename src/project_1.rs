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

pub struct KvStore;

impl KvStore {
	pub fn new() -> KvStore {
		KvStore {}
	}
	
	/// Set the value of a string key to a string
	pub fn set(&mut self, key: String, value: String) {
		unimplemented!()
	}
	
	/// Get the string value of a given string key
	pub fn get(&self, key: String) -> Option<String> {
		unimplemented!()
	}
	
	/// Remove a given key `key`
	pub fn remove(&mut self, key: String) {
		unimplemented!()
	}
}
