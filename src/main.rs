// <fahRcel: A CLI tool to make conversions among fahrenheit and celsius units.>
// Copyright (C) <2022>  <Cromega>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::process;
use clap::Parser;

fn main() {
	let data = Data::parse();
	let v_type = data.v_type;
	let v_num = data.v_num;

	let value: f32 = match v_num.trim().parse::<f32>() {
		Ok(num) => num,
		Err(_) => {
			println!("\n=> Incorrect input, need to include a numeric value\n");
			process::exit(1);
		}
	};

	if v_type.contains("c") && v_type.contains("f") {
		println!("\n=> Incorrect type of value, only fahrenheit(f) or celsius(c), non both");
		process::exit(1);
	}
	if !v_type.contains("c") && !v_type.contains("f") {
		println!("\n=> Need to indlude a type of value: fahrenheit(f) || celsius(c)");
		process::exit(1);
	}

	if v_type.contains('f') {
		let output = value * 1.8 + 32.0;
		println!("\nResult: {value}c = {output}fh")
	}
	if v_type.contains('c') {
		let output = (value-32.0) * 0.555555556;
		println!("\nResult: {value}fh = {output}c");
	}
}

#[derive(Parser)]
#[clap(name = "fahrcel")]
#[clap(author = "Cromega <cr.jrg08@gmail.com>")]
#[clap(version = "1.0.0")]
#[clap(about = "Fahr <-> Cel converter", long_about = "A CLI tool to make Fahrenheit <--> Celsius convertions")]
#[clap(allow_negative_numbers = true)]
struct Data {
	#[clap(name = "output_type", help = "f || c", long_help = "option to convert: f = 'to fahrenheit' || c = 'to celsius'", value_parser)]
	v_type: String,
	#[clap(name = "number", help = "ex. 5", long_help = "value to convert: 5 || -5 || 0 || 100", value_parser)]
	v_num: String
}
