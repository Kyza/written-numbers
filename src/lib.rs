use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub mod en;
pub mod util;

pub type LanguageOptions = HashMap<String, String>;
pub type ToWordsReturn = Result<String, ToWordsError>;
pub type LanguageClosure = dyn Fn(&str, &LanguageOptions) -> ToWordsReturn;
pub type LanguageMap<'a> = HashMap<&'a str, Box<LanguageClosure>>;

lazy_static! {
	static ref IS_NUMBER_REGEX: Regex =
		Regex::new(r"^-?\d+(\.\d+)?$").unwrap();
}

pub struct ToWordsOptions<'a> {
	pub language: &'a str,
}

pub enum ToWordsError {
	NotANumber,
	UnimplementedLanguage,
}

pub struct WrittenNumbers<'a> {
	languages: LanguageMap<'a>,
}

impl WrittenNumbers<'_> {
	pub fn new() -> Self {
		let mut languages: LanguageMap = HashMap::new();

		languages.insert("en", Box::new(en::to_words));

		Self { languages }
	}

	pub fn register_language(
		&mut self,
		name: &'static str,
		implementation: &'static LanguageClosure,
	) {
		self.languages.insert(name, Box::new(implementation));
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

		// Preparse the number.
		let is_negative = number.starts_with('-');
		// TODO: Find a way to do this without converting between &str and String repeatedly.
		let number = number.replace(',', "");
		let mut number = number.as_str();
		if is_negative {
			number = &number[1..];
		}
		number = number.trim_start_matches('0');
		number = number.trim_end_matches('0');
		if number.is_empty() {
			number = "0";
		}
		let mut number = number.to_string();
		if is_negative {
			number.insert(0, '-');
		}

		match self.languages.get(options.language) {
			Some(language) => language(number.as_str(), language_options),
			None => Err(ToWordsError::UnimplementedLanguage),
		}
	}
}

impl Default for WrittenNumbers<'_> {
	fn default() -> Self {
		Self::new()
	}
}
