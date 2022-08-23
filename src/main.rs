use std::process;
use clap::Parser;

fn main() {
	let data = Data::parse();
	let v_type = data.v_type;
	let v_num = data.v_num;

	let value: i32 = match v_num.trim().parse() {
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
		let output = value * (9/5) + 32;
		println!("\nResult c -> fh = {output}")
	}
	if v_type.contains('c') {
		let output = (value-32) * (5/9);
		println!("\nResult fh -> c = {output}")
	}
}

#[derive(Parser)]
#[clap(name = "fahrcel")]
#[clap(author = "Cromega <cr.jrg08@gmail.com>")]
#[clap(version = "1.0.0")]
#[clap(about = "Fahr <-> Cel converter", long_about = "A CLI tool to make Fahrenheit <--> Celsius convertions")]
#[clap(allow_negative_numbers = true)]
struct Data {
	#[clap(name = "type", help = "f || c", long_help = "option to convert: f = 'to fahrenheit' || c = 'to celsius'", value_parser)]
	v_type: String,
	#[clap(name = "number", help = "ex. 5", long_help = "value to convert: 5 || -5 || 0 || 100", value_parser)]
	v_num: String
}
