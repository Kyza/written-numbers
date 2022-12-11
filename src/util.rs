use std::collections::HashMap;

use core::hash::Hash;

pub fn map_has_value<A, B>(map: &HashMap<A, B>, key: &A, value: &B) -> bool
where
	A: Eq,
	A: Hash,
	B: PartialEq,
{
	match map.get(key) {
		Some(found_value) => found_value == value,
		None => false,
	}
}

pub fn chunk_number(str: String, chunk_size: usize) -> Vec<Vec<char>> {
	let mut chunks = vec![];

	let chars = str.chars().collect::<Vec<_>>();

	let mut index = str.len();
	while index > 0 {
		let mut chunk = vec![];

		let mut i = 1;
		while i <= chunk_size {
			chunk.insert(
				0,
				if index > i - 1 { chars[index - i] } else { '0' },
			);
			i += 1;
		}

		chunks.insert(0, chunk);

		index = index.saturating_sub(chunk_size);
	}

	chunks
}
