extern crate forkengine;

use std::io;
use std::ops::Range;

use forkengine::Runtime;

fn preceding_indexes(center: usize, length: usize, count: usize) -> Range<usize> {
	if length <= count {
		return 0..length;
	}

	if count <= (center + 1) {
		return (center + 1 - count)..(center + 1);
	} else { // if the surplus is negative (a shortage of space)
		let deficiency = count - (center + 1);
		return 0..(center + deficiency + 1); // get the shortage back from the end bound
	}
}

fn center_indexes(center: usize, length: usize, padding: usize) -> Range<usize> {
	let result_length = (padding * 2) + 1;
	if length <= result_length {
		return 0..length;
	}

	let left_space = center;
	let right_space = (length - center) - 1;

	let enough_left_space = left_space >= padding;
	let enough_right_space = right_space >= padding;

	if enough_left_space && !enough_right_space {
		let shortage = padding - right_space;
		return (center - padding - shortage)..length;
	}
	else if enough_right_space && !enough_left_space {
		return 0..result_length;
	}
	else {
		assert!(enough_left_space && enough_right_space); // TODO: remove
		return (center - padding)..(center + padding + 1);
	}
}

fn subsequent_indexes(center: usize, length: usize, count: usize) -> Range<usize> {
	if length <= count {
		return 0..length;
	}

	if (length - center) >= count { // if enough indexes exist to the right of 'center'
		return center..(center + count);
	} else { // otherwise, add some indexes to the left to compensate
		let deficiency = count - (length - center);
		return (center - deficiency)..length;
	}
}

fn main() {
	println!("brainfuck instructions: ");
	let mut instructions = String::new();
	io::stdin().read_line(&mut instructions)
		.expect("failed to read input from stdin!");
	instructions = instructions.trim().to_string();

	println!("program input: ");
	let mut input = String::new();
	io::stdin().read_line(&mut input)
		.expect("failed to read input from stdin!");
	input = input.trim().to_string();

	println!("\n\n");

	let product = Runtime::new(instructions.clone(), input.clone().into_bytes()).run();
	for snapshot in product.snapshots {

		print!("instruction tape [{}]:  ", snapshot.instruction_pointer);
		let instruction_range = center_indexes(snapshot.instruction_pointer, instructions.len(), 4);

		if instruction_range.start > 0 {
			print!("(...)");
		} else {
			print!("{{{{");
		}

		for i in instruction_range.clone() {
			let this_char = instructions.chars().nth(i).unwrap();

			if i == snapshot.instruction_pointer {
				print!(" {{{}}} ", this_char);
			} else {
				print!(" {} ", this_char);
			}
		}

		if instruction_range.end < instructions.len() {
			println!("(...)");
		} else {
			println!("}}}}");
		}


		if snapshot.is_error {
			println!("[ ERROR: {} ]\n", snapshot.message);
		} else {
			println!("\t{}\n", snapshot.message);
		}


		if snapshot.memory.is_empty() {
			println!("memory tape: (empty)");
		} else {
			print!("memory tape [{}]:  ", snapshot.memory_pointer);
			let memory_range = center_indexes(snapshot.memory_pointer, snapshot.memory.len(), 4);

			if memory_range.start > 0 {
				print!("(...)");
			} else {
				print!("{{{{");
			}

			for i in memory_range.clone() {
				let this_byte = snapshot.memory[i];

				if i == snapshot.memory_pointer {
					print!(" {{{:>03}}} ", this_byte);
				} else {
					print!(" {:>03} ", this_byte);
				}
			}

			if memory_range.end < snapshot.memory.len() {
				println!("(...)");
			} else {
				println!("}}}}");
			}
		}


		if input.is_empty() {
			println!("input: (empty)");
		} else {
			if snapshot.input_pointer == input.len() {
				println!("input [end]");
			} else {
				print!("input [{}]: ", snapshot.input_pointer);
				let input_range = subsequent_indexes(snapshot.input_pointer, input.len(), 5);

				if input_range.start > 0 {
					print!("(...)");
				} else {
					print!("{{{{");
				}

				for i in input_range.clone() {
					let this_char = input.chars().nth(i).unwrap();

					if i == snapshot.input_pointer {
						print!(" {{'{}'}} ", this_char);
					} else {
						print!(" '{}' ", this_char);
					}
				}

				if input_range.end < input.len() {
					println!("(...)");
				} else {
					println!("}}}}");
				}
			}
		}


		if snapshot.output.is_empty() {
			println!("output: (empty)");
		} else {
			print!("output [{}]: ", snapshot.output.len());
			let last_index = snapshot.output.len() - 1;
			let output_range = preceding_indexes(last_index, snapshot.output.len(), 5);

			if output_range.start > 0 {
				print!("(...)");
			} else {
				print!("{{{{");
			}

			for i in output_range.clone() {
				let this_byte = snapshot.output[i];

				let formatted: String;
				if this_byte <= 127 {
					formatted = format!("{:>03} ({})", this_byte, (this_byte as char).escape_debug());
				} else {
					formatted = format!("{:>03}", this_byte);
				}

				if i == last_index {
					print!(" {{{}}} ", formatted);
				} else {
					print!(" {} ", formatted);
				}
			}

			println!("}}}}");
		}


		println!("\n\n");
	}

	let final_output: String;
	match String::from_utf8(product.output.clone()) {
		Ok(string) => final_output = string,
		Err(_) => final_output = format!("{:?}", product.output)
	}

	println!("executed {} instructions in {:.2} ms", product.executions, (product.time as f64 / 1000000.0) as f64);
	println!("program result: {}", final_output);
}
