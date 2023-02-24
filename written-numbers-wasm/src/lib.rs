extern crate wasm_bindgen;
use std::collections::HashMap;

use maplit::hashmap;
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use written_numbers::{LanguageOptions, ToWordsOptions};

// use written_numbers::{self, LanguageOptions, ToWordsOptions};

// pub type LanguageOptions = HashMap<&'static str, &'static str>;
// pub type ToWordsReturn = Result<String, ToWordsError>;
// pub type ToWordsClosure = dyn Fn(&str, &LanguageOptions) -> ToWordsReturn;

// pub type LanguagesMap<'a> = HashMap<&'a str, Box<ToWordsClosure>>;

#[derive(Serialize, Deserialize)]
pub struct Example {
	pub field1: HashMap<String, String>,
	pub field2: Vec<Vec<f32>>,
	pub field3: [f32; 4],
}

#[wasm_bindgen]
pub fn send_example_to_js() -> JsValue {
	let mut field1 = HashMap::new();
	field1.insert("lol".to_string(), String::from("ex"));
	let example = Example {
		field1,
		field2: vec![vec![1., 2.], vec![3., 4.]],
		field3: [1., 2., 3., 4.],
	};

	serde_wasm_bindgen::to_value(&example).unwrap()
}

#[wasm_bindgen]
pub fn to_words(
	number: String,
	options: JsValue,
	language_options: JsValue,
) -> JsValue {
	let options: ToWordsOptions =
		serde_wasm_bindgen::from_value(options).unwrap();
	let language_options: LanguageOptions =
		serde_wasm_bindgen::from_value(language_options).unwrap();

	let result = written_numbers::to_words(
		&number,
		&options,
		&language_options,
		&mut hashmap! {},
	);

	match result {
		Ok(result) => serde_wasm_bindgen::to_value(&result).unwrap(),
		Err(error) => serde_wasm_bindgen::to_value(&error).unwrap(),
	}
}
