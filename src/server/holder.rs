

use std::collections::HashMap;
use std::hash::Hash;

pub trait HolderId: Copy + Eq + Hash {
	fn next(&self) -> Self;
	fn initial() -> Self;
}


#[derive(Debug, Clone)]
pub struct Holder <K: HolderId, V> {
	storage: HashMap<K, V>,
	next_key: K
}

impl <K: HolderId, V> Holder<K, V> {
	pub fn new() -> Self {
		Self {
			next_key: K::initial(),
			storage: HashMap::new()
		}
	}
	
	pub fn insert(&mut self, value: V) -> K {
		let key = self.next_key;
		self.next_key = key.next();
		self.storage.insert(key, value);
		key
	}
	
	pub fn get(&self, key: &K) -> Option<&V> {
		self.storage.get(key)
	}
	
	pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
		self.storage.get_mut(key)
	}
	
	pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<K, V> {
		self.storage.iter_mut()
	}
	
	pub fn remove(&mut self, key: &K) -> Option<V> {
		self.storage.remove(key)
	}
}



