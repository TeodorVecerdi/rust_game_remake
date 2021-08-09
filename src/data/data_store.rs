
pub struct DataStore {
	dict: std::collections::HashMap<&'static str, Box<dyn std::any::Any>>,
}

#[allow(dead_code)]
impl DataStore {
	pub fn new() -> DataStore {
		DataStore {
			dict: std::collections::HashMap::new(),
		}
	}

	pub fn has(&self, key: &str) -> bool {
		self.dict.get(key).is_some()
	}

	pub fn set_boxed<T: std::any::Any + std::fmt::Debug>(&mut self, key: &'static str, value: Box<T>) {
		self.dict.insert(key, value);
	}

	pub fn set<T: std::any::Any + std::fmt::Debug>(&mut self, key: &'static str, value: T) {
		self.dict.insert(key, Box::new(value));
	}

	pub fn get(&self, key: &'static str) -> Option<&Box<dyn std::any::Any>> {
		self.dict.get(key)
	}

	pub fn get_t<T: std::any::Any>(&self, key: &'static str) -> Option<Box<&T>> {
		match self.get(key).map(|value| value.downcast_ref::<T>()) {
			None => None,
			Some(value) => match value {
				None => None,
				Some(value) => Some(Box::new(value)),
			}
		}
	}

	pub fn get_mut(&mut self, key: &'static str) -> Option<&mut Box<dyn std::any::Any>> {
		self.dict.get_mut(key)
	}

	pub fn get_mut_t<T: std::any::Any>(&mut self, key: &'static str) -> Option<Box<&mut T>> {
		match self.get_mut(key).map(|value| value.downcast_mut::<T>()) {
			None => None,
			Some(value) => match value {
				None => None,
				Some(value) => Some(Box::new(value)),
			}
		}
	}

	pub fn remove(&mut self, key: &'static str) -> Option<Box<dyn std::any::Any>> {
		self.dict.remove(key)
	}
}