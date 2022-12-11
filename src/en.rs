use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

use crate::{util::map_has_value, LanguageOptions, ToWordsReturn};

lazy_static! {
	static ref ONES: HashMap<char, String> = hashmap! {
			'0' => "zero".to_string(),
			'1' => "one".to_string(),
			'2' => "two".to_string(),
			'3' => "three".to_string(),
			'4' => "four".to_string(),
			'5' => "five".to_string(),
			'6' => "six".to_string(),
			'7' => "seven".to_string(),
			'8' => "eight".to_string(),
			'9' => "nine".to_string(),
	};
	static ref TEENS: HashMap<char, String> = hashmap! {
			'0' => "ten".to_string(),
			'1' => "eleven".to_string(),
			'2' => "twelve".to_string(),
			'3' => "thirteen".to_string(),
			'4' => "fourteen".to_string(),
			'5' => "fifteen".to_string(),
			'6' => "sixteen".to_string(),
			'7' => "seventeen".to_string(),
			'8' => "eighteen".to_string(),
			'9' => "ninteen".to_string(),
	};
	static ref TENS: HashMap<char, String> = hashmap! {
			'0' => "zero".to_string(),
			'1' => "ten".to_string(),
			'2' => "twenty".to_string(),
			'3' => "thirty".to_string(),
			'4' => "forty".to_string(),
			'5' => "fifty".to_string(),
			'6' => "sixty".to_string(),
			'7' => "seventy".to_string(),
			'8' => "eighty".to_string(),
			'9' => "ninety".to_string(),
	};
}

pub fn ones_word(digit: char) -> String {
	ONES.get(&digit)
		.expect(&format!("\"{digit}\" is not a valid digit"))
		.to_string()
}

pub fn teens_word(digit: char) -> String {
	TEENS
		.get(&digit)
		.expect(&format!("\"{digit}\" is not a valid digit"))
		.to_string()
}

pub fn tens_word((tens_digit, ones_digit): (char, char)) -> String {
	if tens_digit == '0' {
		return ones_word(ones_digit);
	} else if tens_digit == '1' {
		return teens_word(ones_digit);
	}

	let tens_word = TENS
		.get(&tens_digit)
		.expect(&format!("\"{tens_digit}\" is not a valid digit"))
		.to_string();

	if ones_digit == '0' {
		return tens_word;
	}
	return format!("{tens_word}-{}", ones_word(ones_digit));
}

pub fn hundreds_word(
	(hundreds_digit, tens_digit, ones_digit): (char, char, char),
	options: &LanguageOptions,
) -> String {
	if hundreds_digit == '0' && tens_digit == '0' {
		return ones_word(ones_digit);
	}
	if hundreds_digit == '0' {
		return tens_word((tens_digit, ones_digit));
	}

	let result = format!("{} hundred", ones_word(hundreds_digit));

	if tens_digit == '0' && ones_digit == '0' {
		return result;
	}

	let joiner = if map_has_value(
		options,
		&"hundred and".to_string(),
		&"true".to_string(),
	) {
		" and"
	} else {
		""
	};

	if tens_digit == '0' {
		return format!("{result}{} {}", joiner, ones_word(ones_digit));
	}

	return format!(
		"{result}{joiner} {}",
		tens_word((tens_digit, ones_digit))
	);
}

pub fn thousands_word(
	digits: (char, char, char),
	options: &LanguageOptions,
) -> String {
	format!("{} thousand", hundreds_word(digits, options))
}

pub fn to_words(number: &str, options: &LanguageOptions) -> ToWordsReturn {
	let mut words = String::from("");
	let mut number = number;

	println!("{}", number);

	match number {
		"0" => return Ok(ones_word('0')),
		"-0" => return Ok(format!("negative {}", ones_word('0'))),
		_ => {}
	}

	// Split off the decimals from the actual number.
	let mut decimals = "";
	if let Some(decimal_point_index) = number.find(".") {
		decimals = &number[decimal_point_index + 1..];
		number = &number[..decimal_point_index];
	}

	let mut index = number.len();
	let mut iteration = 0;
	while index > 0 {
		let mut chunk =
			number[index.checked_sub(3).unwrap_or(0)..index].to_string();

		// Ensure the chunk is always three long.
		while chunk.len() < 3 {
			chunk.insert(0, '0');
		}

		if chunk == "000" {
			// Skip empty chunks.
			continue;
		}

		let chunk = chunk.as_bytes();

		words.insert_str(
			0,
			&format!(
				"{}{} ",
				&match iteration {
					// If it's the first iteration, handle the hundreds.
					0 => hundreds_word(
						(
							chunk[0] as char,
							chunk[1] as char,
							chunk[2] as char,
						),
						options,
					),
					1 => thousands_word(
						(
							chunk[0] as char,
							chunk[1] as char,
							chunk[2] as char,
						),
						options,
					),
					_ => "".to_string(),
				},
				if map_has_value(
					options,
					&"commas".to_string(),
					&"true".to_string()
				) {
					","
				} else {
					""
				}
			),
		);

		index = index.checked_sub(3).unwrap_or(0);
		iteration += 1;
	}

	// Remove the extra space and comma.
	words = words[..words.len() - 2].to_string();

	// TODO: 0.8009 => zero point eight thousand nine ten-thousandths
	if decimals.len() > 0 {
		words.push_str(" point");
		for decimal in decimals.chars() {
			words.push_str(" ");
			words.push_str(&ones_word(decimal));
		}
	}

	Ok(words)
}
