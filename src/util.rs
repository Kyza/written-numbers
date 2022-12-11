use std::collections::HashMap;

use core::hash::Hash;
pub fn map_has<A, B>(map: &HashMap<A, B>, key: &A) -> bool
where
	A: Eq,
	A: Hash,
{
	match map.get(key) {
		Some(_) => true,
		None => false,
	}
}
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
