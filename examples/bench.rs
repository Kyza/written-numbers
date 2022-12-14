use std::fs;

use maplit::hashmap;
use written_numbers::*;

use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
	let number =
		fs::read_to_string("./number.txt").expect("numbers.txt should exist");

	let result = WrittenNumbers::new().to_words(
		&number,
		&ToWordsOptions { language: "en" },
		&hashmap! {
			"hundred_and" => "true",
			"commas" => "true"
		},
	);

	match result {
		Ok(_result) => {
			// println!("{result}");
		}
		Err(err) => match err {
			ToWordsError::NotANumber => {
				println!("\"{number}\" is not a valid number.");
			}
			ToWordsError::UnimplementedLanguage => {
				println!("The language \"en\" hasn't been implemented yet.");
			}
		},
	}

	let peak_mem = PEAK_ALLOC.peak_usage_as_mb();
	println!("Peak memory: {} MB", peak_mem);
}
