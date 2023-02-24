extern crate wasm_bindgen;
use std::collections::HashMap;

use maplit::hashmap;
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use written_numbers::{LanguageOptions, ToWordsOptions};

#[wasm_bindgen]
pub fn to_words(
	number: &str,
	options: JsValue,
	language_options: JsValue,
) -> JsValue {
	let options: ToWordsOptions =
		serde_wasm_bindgen::from_value(options).unwrap();
	let language_options: LanguageOptions =
		serde_wasm_bindgen::from_value(language_options).unwrap();

	let result = written_numbers::to_words(
		&number.to_string(),
		&options,
		&language_options,
		&mut hashmap! {},
	);

	match result {
		Ok(result) => serde_wasm_bindgen::to_value(&result).unwrap(),
		Err(error) => serde_wasm_bindgen::to_value(&error).unwrap(),
	}
}
