use phf::phf_map;

use crate::{util::map_has_value, LanguageOptions, ToWordsReturn};

static ONES: phf::Map<char, &'static str> = phf_map! {
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
static TEENS: phf::Map<char, &'static str> = phf_map! {
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
static TENS: phf::Map<char, &'static str> = phf_map! {
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
