use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub mod en;
pub mod util;

pub type LanguageOptions = HashMap<&'static str, &'static str>;
pub type ToWordsReturn = Result<String, ToWordsError>;
pub type ToWordsClosure = dyn Fn(&str, &LanguageOptions) -> ToWordsReturn;

pub type ToOrdinalReturn = Result<String, ToOrdinalError>;
pub type ToOrdinalClosure = dyn Fn(&str) -> ToOrdinalReturn;

pub type LanguageMap<'a> = HashMap<&'a str, LanguageImplementation>;

lazy_static! {
	static ref IS_NUMBER_REGEX: Regex =
		Regex::new(r"^-?[1-9]{1}\d*(\.\d+)?$").unwrap();
}

pub struct ToWordsOptions<'a> {
	pub language: &'a str,
}
pub struct ToOrdinalOptions<'a> {
	pub language: &'a str,
}

pub enum ToWordsError {
	NotANumber,
	UnimplementedLanguage,
}

pub enum ToOrdinalError {
	UnimplementedLanguage,
}

pub struct LanguageImplementation {
	to_words: Box<ToWordsClosure>,
	to_ordinal: Box<ToOrdinalClosure>,
}

fn add_default_languages(
	languages: &mut HashMap<&str, LanguageImplementation>,
) {
	if !languages.contains_key("en") {
		languages.insert(
			"en",
			LanguageImplementation {
				to_words: Box::new(en::to_words),
				to_ordinal: Box::new(en::to_ordinal),
			},
		);
	}
}

pub fn to_words(
	number: &str,
	options: &ToWordsOptions,
	language_options: &LanguageOptions,
	languages: &mut HashMap<&str, LanguageImplementation>,
) -> ToWordsReturn {
	if !IS_NUMBER_REGEX.is_match(number) {
		return Err(ToWordsError::NotANumber);
	}

	add_default_languages(languages);

	match languages.get(options.language) {
		Some(language) => (language.to_words)(number, language_options),
		None => Err(ToWordsError::UnimplementedLanguage),
	}
}

pub fn to_ordinal(
	words: &str,
	options: &ToOrdinalOptions,
	languages: &mut HashMap<&str, LanguageImplementation>,
) -> ToOrdinalReturn {
	add_default_languages(languages);

	match languages.get(options.language) {
		Some(language) => (language.to_ordinal)(words),
		None => Err(ToOrdinalError::UnimplementedLanguage),
	}
}
