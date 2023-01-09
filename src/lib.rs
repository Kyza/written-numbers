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

pub struct WrittenNumbers<'a> {
	languages: LanguageMap<'a>,
}

pub struct LanguageImplementation {
	to_words: Box<ToWordsClosure>,
	to_ordinal: Box<ToOrdinalClosure>,
}

impl WrittenNumbers<'_> {
	pub fn new() -> Self {
		let mut languages: LanguageMap = HashMap::new();

		languages.insert(
			"en",
			LanguageImplementation {
				to_words: Box::new(en::to_words),
				to_ordinal: Box::new(en::to_ordinal),
			},
		);

		Self { languages }
	}

	pub fn register_language(
		&mut self,
		name: &'static str,
		implementation: LanguageImplementation,
	) {
		self.languages.insert(name, implementation);
	}

	pub fn unregister_language(&mut self, name: &'static str) {
		self.languages.remove(name);
	}

	pub fn has_language(&self, name: &'static str) -> bool {
		self.languages.get(&name).is_some()
	}

	pub fn to_words(
		&self,
		number: &str,
		options: &ToWordsOptions,
		language_options: &LanguageOptions,
	) -> ToWordsReturn {
		if !IS_NUMBER_REGEX.is_match(number) {
			return Err(ToWordsError::NotANumber);
		}

		match self.languages.get(options.language) {
			Some(language) => (language.to_words)(number, language_options),
			None => Err(ToWordsError::UnimplementedLanguage),
		}
	}

	pub fn to_ordinal(
		&self,
		words: &str,
		options: &ToOrdinalOptions,
	) -> ToOrdinalReturn {
		match self.languages.get(options.language) {
			Some(language) => (language.to_ordinal)(words),
			None => Err(ToOrdinalError::UnimplementedLanguage),
		}
	}
}

impl Default for WrittenNumbers<'_> {
	fn default() -> Self {
		Self::new()
	}
}
