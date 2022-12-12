use lazy_static::lazy_static;
use phf::phf_map;
use regex::Regex;

use crate::{
	util::{chunk_number, map_has_value},
	LanguageOptions, ToWordsReturn,
};

pub static ONES: phf::Map<char, &'static str> = phf_map! {
		'0' => "zero",
		'1' => "one",
		'2' => "two",
		'3' => "three",
		'4' => "four",
		'5' => "five",
		'6' => "six",
		'7' => "seven",
		'8' => "eight",
		'9' => "nine",
};
pub static TEENS: phf::Map<char, &'static str> = phf_map! {
		'0' => "ten",
		'1' => "eleven",
		'2' => "twelve",
		'3' => "thirteen",
		'4' => "fourteen",
		'5' => "fifteen",
		'6' => "sixteen",
		'7' => "seventeen",
		'8' => "eighteen",
		'9' => "ninteen",
};
pub static TENS: phf::Map<char, &'static str> = phf_map! {
		'0' => "zero",
		'1' => "ten",
		'2' => "twenty",
		'3' => "thirty",
		'4' => "forty",
		'5' => "fifty",
		'6' => "sixty",
		'7' => "seventy",
		'8' => "eighty",
		'9' => "ninety",
};
pub static ONES_ILLION_PARTS: phf::Map<&'static str, &'static str> = phf_map! {
	"un" => "mi",
	"duo" => "bi",
	"tre" => "tri",
	"quattuor" => "quadri",
	"quin" => "quinti",
	"se" => "sexti",
	"septe" => "septi",
	"octo" => "octi",
	"nove" => "noni",
};

pub static ILLION_PARTS: phf::Map<u8, phf::Map<u8, &'static str>> = phf_map! {
	0u8 => phf_map! {
		1u8 => "un",
		2u8 => "duo",
		3u8 => "tre",
		4u8 => "quattuor",
		5u8 => "quin",
		6u8 => "se",
		7u8 => "septe",
		8u8 => "octo",
		9u8 => "nove"
	},
	1u8 => phf_map! {
		1u8 => "dec",
		2u8 => "vigint",
		3u8 => "trigint",
		4u8 => "quadragint",
		5u8 => "quinquagint",
		6u8 => "sexagint",
		7u8 => "septuagint",
		8u8 => "octogint",
		9u8 => "nonagint"
	},
	2u8 => phf_map! {
		1u8 => "centi",
		2u8 => "ducenti",
		3u8 => "trecenti",
		4u8 => "quadringenti",
		5u8 => "quingenti",
		6u8 => "sescenti",
		7u8 => "septingenti",
		8u8 => "octingenti",
		9u8 => "nongenti"
	},
};

pub fn ones_word(digit: char) -> String {
	ONES.get(&digit)
		.unwrap_or_else(|| panic!("\"{digit}\" is not a valid digit"))
		.to_string()
}

pub fn teens_word(digit: char) -> String {
	TEENS
		.get(&digit)
		.unwrap_or_else(|| panic!("\"{digit}\" is not a valid digit"))
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
		.unwrap_or_else(|| panic!("\"{tens_digit}\" is not a valid digit"))
		.to_string();

	if ones_digit == '0' {
		return tens_word;
	}
	format!("{tens_word}-{}", ones_word(ones_digit))
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

	format!("{result}{joiner} {}", tens_word((tens_digit, ones_digit)))
}

pub fn thousands_word(
	digits: (char, char, char),
	options: &LanguageOptions,
) -> String {
	format!("{} thousand", hundreds_word(digits, options))
}

/*
	Gets the parts of an illion from the illion's number.
	Million is 1.
	Billion is 2.
	Nonillion is 9.
	And so on...
*/
pub fn illion_part_numbers(illion: usize) -> Vec<Vec<char>> {
	let mut buffer = itoa::Buffer::new();
	let printed = buffer.format(illion).to_string();

	let mut chunks = chunk_number(printed, 3);
	for chunk in chunks.iter_mut() {
		chunk.reverse();
	}

	chunks
}

pub fn illion_parts(part_numbers: &Vec<Vec<char>>) -> Vec<Vec<&'static str>> {
	let mut parts: Vec<Vec<&str>> = vec![];

	for numbers in part_numbers.iter() {
		let mut part: Vec<&str> = vec![];

		let mut digit_pos = 0;
		for digit_char in numbers.iter() {
			let digit: u8 = digit_char
				.to_digit(10)
				.expect(&format!("\"{digit_char}\" should be a number"))
				as u8;

			if digit != 0 {
				let part_string = ILLION_PARTS
					.get(&(digit_pos % 3))
					.unwrap()
					.get(&(digit))
					.unwrap();

				part.push(part_string);
			} else {
				part.push("");
			}

			digit_pos += 1;
		}

		parts.push(part);
	}

	parts
}

lazy_static! {
	pub static ref ILLION_COMBINER_REGEX: Vec<Regex> = vec![
		Regex::new(r"^[36]([2-5]|0[345])").unwrap(),
		Regex::new(r"^6(8|0[18])").unwrap(),
		Regex::new(r"^[79]([28]|08)").unwrap(),
		Regex::new(r"^[79]([134567]|0[1-7])").unwrap(),
	];
}

lazy_static! {
	pub static ref ILLION_COMBINER_CHARS: Vec<char> =
		vec!['s', 'x', 'm', 'n'];
	pub static ref ONLY_ONES_ILLIONS_REGEX: Regex =
		Regex::new(r"^[1-9]00$").unwrap();
}

pub fn combine_illion_parts(
	illion_number_parts: &Vec<Vec<char>>,
	illion_words: &Vec<Vec<&'static str>>,
) -> String {
	let mut combined = String::new();

	let mut i = 0;
	for illion_numbers in illion_number_parts {
		// Make sure to add the illi between every chunk.
		if i > 0 {
			combined.push_str("lli");
		}

		let ones_word = illion_words[i][0];
		let tens_word = illion_words[i][1];
		let hundreds_word = illion_words[i][2];
		let ones_number = illion_numbers[0];
		let tens_number = illion_numbers[1];
		let hundreds_number = illion_numbers[2];

		let illion_chunk_numbers =
			illion_numbers.iter().cloned().collect::<String>();

		// If there is nothing, it's a nillion.
		if ones_number == '0' && tens_number == '0' && hundreds_number == '0'
		{
			// Handle just nillion because why not.
			if combined == "" {
				combined.push_str("ni");
				continue;
			}
			if combined.ends_with("lli") {
				combined.push_str("ni");
			}
		}

		// Grab the correct combiner for after the ones.
		let mut illions_combiner = String::new();
		let mut j = 0;
		for regex in ILLION_COMBINER_REGEX.iter() {
			if regex.is_match(&illion_chunk_numbers) {
				illions_combiner.push(ILLION_COMBINER_CHARS[j]);
			}
			j += 1;
		}

		// If there is only a ones, add the correct special ones (million, billion, ...).
		if ONLY_ONES_ILLIONS_REGEX.is_match(&illion_chunk_numbers) {
			combined.push_str(ONES_ILLION_PARTS[ones_word]);
		}
		// Otherwise add the ones and the combiner that was found for it.
		else if ones_number != '0' {
			combined.push_str(&format!("{ones_word}{illions_combiner}"));
		}
		// Add the tens.
		if tens_number != '0' {
			combined.push_str(tens_word);
			match tens_number {
				// Deci and viginti are always `i`.
				'1' => {
					combined.push('i');
				}
				'2' => {
					combined.push('i');
				}
				_ => {
					combined.push(if hundreds_number != '0' {
						'a'
					} else {
						'i'
					});
				}
			}
		}
		// Add the hundreds.
		if hundreds_number != '0' {
			combined.push_str(hundreds_word);
		}

		i += 1;
	}

	// Add the ending.
	combined.push_str("llion");

	combined
}

pub fn illion_name(illion_number: usize) -> String {
	let illion_part_numbers = illion_part_numbers(illion_number.abs_diff(0));

	format!(
		"{}",
		combine_illion_parts(
			&illion_part_numbers,
			&illion_parts(&illion_part_numbers),
		)
	)
}

pub fn illions_word(
	digits: (char, char, char),
	illion: usize,
	options: &LanguageOptions,
) -> String {
	format!("{} {}", hundreds_word(digits, options), illion_name(illion))
}

pub fn to_words(number: &str, options: &LanguageOptions) -> ToWordsReturn {
	let mut words = String::new();

	match number {
		"0" => return Ok(ones_word('0')),
		"-0" => return Ok(format!("negative {}", ones_word('0'))),
		_ => {}
	}

	// Split off the decimals from the actual number.
	let mut decimals = String::new();
	let whole;
	if let Some(decimal_point_index) = number.find('.') {
		decimals = number[decimal_point_index + 1..].to_string();
		whole = number[..decimal_point_index].to_string();
	} else {
		whole = number.to_string();
	}

	let chunks = chunk_number(whole, 3);

	let mut i = 0;
	for chunk in chunks.iter().rev() {
		let chunk = (chunk[0], chunk[1], chunk[2]);

		// Skip empty chunks.
		if chunk == ('0', '0', '0') {
			i += 1;
			continue;
		}

		let word = match i {
			// If it's the first iteration, handle the hundreds.
			0 => hundreds_word(chunk, options),
			1 => thousands_word(chunk, options),
			_ => illions_word(chunk, i - 1, options),
		};

		words.insert_str(
			0,
			&format!(
				"{}{} ",
				&word,
				// Join with a comma if that option is enabled.
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

		i += 1;
	}

	// Remove the extra space and comma.
	words.truncate(words.len() - 2);

	// TODO: 0.8009 => zero point eight thousand nine ten-thousandths
	if !decimals.is_empty() {
		words.push_str(" point");
		for decimal in decimals.chars() {
			words.push(' ');
			words.push_str(&ones_word(decimal));
		}
	}

	Ok(words)
}
