use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub mod en;
pub mod util;

pub type LanguageOptions = HashMap<&'static str, &'static str>;
pub type ToWordsReturn = Result<String, ToWordsError>;
pub type ToWordsClosure = dyn Fn(&str, &LanguageOptions) -> ToWordsReturn;

pub type LanguageMap<'a> = HashMap<&'a str, Box<ToWordsClosure>>;

lazy_static! {
	static ref IS_NUMBER_REGEX: Regex =
		Regex::new(r"^(-?\d+)(\.\d+)?$").unwrap();
}

pub struct ToWordsOptions<'a> {
	pub language: &'a str,
}

pub enum ToWordsError {
	NotANumber,
	UnimplementedLanguage,
}

pub enum ToOrdinalError {
	UnimplementedLanguage,
}

fn add_default_languages(languages: &mut LanguageMap) {
	if !languages.contains_key("en") {
		languages.insert("en", Box::new(en::to_words));
	}
}

pub fn to_words(
	number: &str,
	options: &ToWordsOptions,
	language_options: &LanguageOptions,
	languages: &mut HashMap<&str, Box<ToWordsClosure>>,
) -> ToWordsReturn {
	if !IS_NUMBER_REGEX.is_match(number) {
		return Err(ToWordsError::NotANumber);
	}

	add_default_languages(languages);

	let mut parsed_number =
		number.trim_start_matches('-').trim_matches('0').to_string();
	if parsed_number.ends_with('.') {
		parsed_number.pop();
	}
	if parsed_number.starts_with('.') {
		parsed_number = format!("0{}", parsed_number);
	}
	if parsed_number.len() == 0 {
		parsed_number = "0".to_string();
	}
	if number.starts_with('-') {
		parsed_number = format!("-{}", parsed_number);
	}

	match languages.get(options.language) {
		Some(language) => (language)(&parsed_number, language_options),
		None => Err(ToWordsError::UnimplementedLanguage),
	}
}
