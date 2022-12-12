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

// TODO: Don't use insert.
pub fn chunk_number(str: String, chunk_size: usize) -> Vec<Vec<char>> {
	let mut chunks = vec![];
	let mut chunk = vec![];

	let chars = str.chars().into_iter().rev();

	for char in chars {
		chunk.push(char);

		if chunk.len() == chunk_size {
			chunk.reverse();
			chunks.push(chunk.clone());
			chunk.clear();
		}
	}

	if !chunk.is_empty() && chunk.len() < 3 {
		while chunk.len() < chunk_size {
			chunk.push('0');
		}
		chunk.reverse();
		chunks.push(chunk);
	}

	chunks.reverse();

	chunks
}
