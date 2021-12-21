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

#[macro_use]
extern crate clap;
use std::process::exit;
use clap::App;


fn main() {
	let yaml = load_yaml!("project_1_cli.yaml");
	let args = App::from_yaml(yaml)
		.version(env!("CARGO_PKG_VERSION"))
		.get_matches();
	
	match args.subcommand() {
		("set", Some(_)) => {
			eprint!("unimplemented");
			exit(255);
		},
		
		("get", Some(_)) => {
			eprint!("unimplemented");
			exit(255);
		},
		
		("rm", Some(_)) => {
			eprint!("unimplemented");
			exit(255);
		},
		
		_ => {
			exit(255);
		}
	}
}
