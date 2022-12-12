use std::{fs, io};

use maplit::hashmap;
use written_numbers::*;

pub fn get_input(prompt: &str) -> String {
	println!("{prompt} ");
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
		Ok(_goes_into_input_above) => {}
		Err(_no_updates_is_fine) => {}
	}
	input.trim().to_string()
}

fn main() {
	let number = fs::read_to_string("./number.txt")
		.expect("Should have been able to read the file");

	let wn = WrittenNumbers::new();

	let result = wn.to_words(
		&number,
		&ToWordsOptions { language: "en" },
		&hashmap! {
			"hundred and" => "true",
			"commas" => "true"
		},
	);

	match result {
		Ok(result) => {
			// println!("\"{result}\"");
			// fs::write("./number_words.txt", result)
			// 	.expect("Unable to write file");
		}
		Err(err) => {
			match err {
				ToWordsError::NotANumber => {
					println!("\"{number}\" is not a valid number.");
				}
				ToWordsError::UnimplementedLanguage => {
					println!("The language hasn't been implemented yet.");
				}
			}
			main();
		}
	}
}
