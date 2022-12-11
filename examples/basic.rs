use std::io;

use maplit::hashmap;
use written_numbers::*;

pub fn get_input(prompt: &str) -> String {
	println!("{} ", prompt);
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
		Ok(_goes_into_input_above) => {}
		Err(_no_updates_is_fine) => {}
	}
	input.trim().to_string()
}

fn main() {
	let number: String = get_input("Enter a number:");
	let lang: String = get_input("Enter a language:");

	let wn = WrittenNumbers::new();

	let result = wn.to_words(
		&number,
		&ToWordsOptions { language: &lang },
		&hashmap! {
			"hundred and".to_string() => "true".to_string(),
			"commas".to_string() => "true".to_string()
		},
	);

	match result {
		Ok(result) => println!("\"{}\"", result),
		Err(err) => {
			match err {
				ToWordsError::NotANumber => {
					println!("\"{}\" is not a valid number.", number);
				}
				ToWordsError::UnimplementedLanguage => {
					println!("The language hasn't been implemented yet.");
				}
			}
			main();
		}
	}
}
