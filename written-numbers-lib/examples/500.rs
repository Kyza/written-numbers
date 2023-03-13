#![feature(fs_try_exists, let_chains)]

use std::fs::{self, OpenOptions};
use std::io::Write;

use maplit::hashmap;
use written_numbers::*;

use peak_alloc::PeakAlloc;

fn main() {
	if let Ok(exists) = fs::try_exists("number-names.txt") && exists {
		fs::remove_file("number-names.txt").unwrap();
	}

	let mut i = 1;

	let mut names = "".to_string();

	loop {
		if i % 100000 == 0 {
			println!("{}", i);
		}

		let name = written_numbers::english::illion_name(i);

		if names.len() + name.len() + 1 > 524288000 {
			let mut file = OpenOptions::new()
				.create(true)
				.write(true)
				.append(true)
				.open("number-names.txt")
				.unwrap();

			writeln!(file, "{}", names.trim_end_matches('\n'));
			return;
		}

		names.push_str(&name);
		names.push('\n');

		i += 1;
	}
}
