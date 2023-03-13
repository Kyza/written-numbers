use lazy_static::lazy_static;
use phf::phf_map;
use pomsky_macro::pomsky;
use regex::Regex;

use crate::{util::map_has_value, LanguageOptions, ToWordsError};

pub type ToWordsReturn = Result<String, ToWordsError>;

pub static WORDS: phf::Map<
	&'static str,
	phf::Map<
		&'static str,
		phf::Map<&'static str, phf::Map<char, &'static str>>,
	>,
> = phf_map! {
	"cardinal" => phf_map!{
		"masculine" => phf_map!{
			"ones" => phf_map!{
				'0' => "",
				'1' => "ūnus",
				'2' => "duo",
				'3' => "trēs",
				'4' => "quattuor",
				'5' => "quīnque",
				'6' => "sex",
				'7' => "septem",
				'8' => "octō",
				'9' => "novem",
			},
			"teens" => phf_map! {
					'0' => "decem",
					'1' => "ūndecim",
					'2' => "duodecim",
					'3' => "tredecim",
					'4' => "quattuordecim",
					'5' => "quīndecim",
					'6' => "sēdecim",
					'7' => "septendecim",
					'8' => "duodēvīgintī",
					'9' => "ūndēvīgintī",
			},
			"tens" => phf_map! {
					'0' => "",
					'1' => "decem",
					'2' => "vīgintī",
					'3' => "trīgintā",
					'4' => "quadrāgintā",
					'5' => "quīnquāgintā",
					'6' => "sexāgintā",
					'7' => "septuāgintā",
					'8' => "octōgintā",
					'9' => "nōnāgintā",
			},
			"hundreds" => phf_map! {
					'0' => "",
					'1' => "centum",
					'2' => "ducentī",
					'3' => "trecentī",
					'4' => "quadringentī",
					'5' => "quīngentī",
					'6' => "sescentī",
					'7' => "septingentī",
					'8' => "octingentī",
					'9' => "nōngentī",
			},
			"thousands" => phf_map! {
					'0' => "",
					'1' => "mīlle",
					'2' => "mīlīa",
					'3' => "mīlīa",
					'4' => "mīlīa",
					'5' => "mīlīa",
					'6' => "mīlīa",
					'7' => "mīlīa",
					'8' => "mīlīa",
					'9' => "mīlīa",
			},
		},
		"feminine" => phf_map!{
			"ones" => phf_map!{
				'0' => "",
				'1' => "ūna",
				'2' => "duae",
				'3' => "tria",
				'4' => "quattuor",
				'5' => "quīnque",
				'6' => "sex",
				'7' => "septem",
				'8' => "octō",
				'9' => "novem",
			},
			"teens" => phf_map! {
					'0' => "decem",
					'1' => "ūndecim",
					'2' => "duodecim",
					'3' => "tredecim",
					'4' => "quattuordecim",
					'5' => "quīndecim",
					'6' => "sēdecim",
					'7' => "septendecim",
					'8' => "duodēvīgintī",
					'9' => "ūndēvīgintī",
			},
			"tens" => phf_map! {
					'0' => "",
					'1' => "decem",
					'2' => "vīgintī",
					'3' => "trīgintā",
					'4' => "quadrāgintā",
					'5' => "quīnquāgintā",
					'6' => "sexāgintā",
					'7' => "septuāgintā",
					'8' => "octōgintā",
					'9' => "nōnāgintā",
			},
			"hundreds" => phf_map! {
					'0' => "",
					'1' => "centum",
					'2' => "ducentae",
					'3' => "trecentae",
					'4' => "quadringentae",
					'5' => "quīngentae",
					'6' => "sescentae",
					'7' => "septingentae",
					'8' => "octingentae",
					'9' => "nōngentae",
			},
			"thousands" => phf_map! {
					'0' => "",
					'1' => "mīlle",
					'2' => "mīlīa",
					'3' => "mīlīa",
					'4' => "mīlīa",
					'5' => "mīlīa",
					'6' => "mīlīa",
					'7' => "mīlīa",
					'8' => "mīlīa",
					'9' => "mīlīa",
			},
		},
		"neuter" => phf_map!{
			"ones" => phf_map!{
				'0' => "",
				'1' => "ūnum",
				'2' => "duo",
				'3' => "trēs",
				'4' => "quattuor",
				'5' => "quīnque",
				'6' => "sex",
				'7' => "septem",
				'8' => "octō",
				'9' => "novem",
			},
			"teens" => phf_map! {
					'0' => "decem",
					'1' => "ūndecim",
					'2' => "duodecim",
					'3' => "tredecim",
					'4' => "quattuordecim",
					'5' => "quīndecim",
					'6' => "sēdecim",
					'7' => "septendecim",
					'8' => "duodēvīgintī",
					'9' => "ūndēvīgintī",
			},
			"tens" => phf_map! {
					'0' => "",
					'1' => "decem",
					'2' => "vīgintī",
					'3' => "trīgintā",
					'4' => "quadrāgintā",
					'5' => "quīnquāgintā",
					'6' => "sexāgintā",
					'7' => "septuāgintā",
					'8' => "octōgintā",
					'9' => "nōnāgintā",
			},
			"hundreds" => phf_map! {
					'0' => "",
					'1' => "centum",
					'2' => "ducenta",
					'3' => "trecenta",
					'4' => "quadringenta",
					'5' => "quīngenta",
					'6' => "sescenta",
					'7' => "septingenta",
					'8' => "octingenta",
					'9' => "nōngenta",
			},
			"thousands" => phf_map! {
					'0' => "",
					'1' => "mīlle",
					'2' => "mīlīa",
					'3' => "mīlīa",
					'4' => "mīlīa",
					'5' => "mīlīa",
					'6' => "mīlīa",
					'7' => "mīlīa",
					'8' => "mīlīa",
					'9' => "mīlīa",
			},
		}
	}
};

lazy_static! {
	static ref IS_VALID_LATIN_NUMBER_REGEX: Regex =
		Regex::new(pomsky!(^ "-"? [d]+ $)).unwrap();
}

pub fn to_words<'a>(
	number: &str,
	options: &LanguageOptions,
) -> ToWordsReturn {
	return Err(ToWordsError::UnimplementedLanguage);

	if !IS_VALID_LATIN_NUMBER_REGEX.is_match(number) {
		return Err(ToWordsError::NotANumber);
	}

	match number {
		"0" => return Ok("nihil".to_string()),
		"-0" => return Ok("minus nihil".to_string()),
		_ => {}
	}

	let chars = number.chars().into_iter().rev();
	let mut words_vec = Vec::with_capacity(number.len());

	let binding = "cardinal".to_string();
	let form = options.get("form").unwrap_or(&binding).as_str();
	let binding = "neuter".to_string();
	let gender = options.get("gender").unwrap_or(&binding).as_str();

	let ones = WORDS
		.get(form)
		.unwrap()
		.get(gender)
		.unwrap()
		.get("ones")
		.unwrap();
	let tens = WORDS
		.get(form)
		.unwrap()
		.get(gender)
		.unwrap()
		.get("tens")
		.unwrap();
	let hundreds = WORDS
		.get(form)
		.unwrap()
		.get(gender)
		.unwrap()
		.get("hundreds")
		.unwrap();
	let thousands = WORDS
		.get(form)
		.unwrap()
		.get(gender)
		.unwrap()
		.get("thousands")
		.unwrap();

	let mut i = 0;
	for char in chars {
		let result = match i {
			0 => ones.get(&char).unwrap(),
			1 => tens.get(&char).unwrap(),
			2 => hundreds.get(&char).unwrap(),
			3 => thousands.get(&char).unwrap(),
			_ => "above",
		};

		if result.len() > 0 {
			words_vec.push(result);
		}

		i += 1;
	}

	words_vec.reverse();

	Ok(words_vec.join(" ").to_string())
}
