use std::cell::RefCell;
use std::collections::HashMap;
use std::any::Any;

pub struct DataStore {
	dict: RefCell<Box<HashMap<&'static str, Box<dyn Any>>>>,
}

#[allow(dead_code)]
impl DataStore {
	pub fn new() -> DataStore {
		DataStore {
			dict: RefCell::new(Box::new(HashMap::new())),
		}
	}

	pub fn has(&self, key: &str) -> bool {
		self.dict.borrow().get(key).is_some()
	}

	pub fn set_boxed<T: Any + std::fmt::Debug>(&self, key: &'static str, value: Box<T>) {
		self.dict.borrow_mut().insert(key, value);
	}

	pub fn set<T: Any + std::fmt::Debug>(&self, key: &'static str, value: T) {
		self.dict.borrow_mut().insert(key, Box::new(value));
	}

	pub fn get(&self, key: &'static str) -> Option<&Box<dyn Any>> {
		unsafe {(*self.dict.as_ptr()).get(key)}
	}

	pub fn get_t<T: Any>(&self, key: &'static str) -> Option<Box<&T>> {
		match self.get(key).map(|value| value.downcast_ref::<T>()) {
			None => None,
			Some(value) => match value {
				None => None,
				Some(value) => Some(Box::new(value)),
			}
		}
	}

	pub fn get_mut(&self, key: &'static str) -> Option<&mut Box<dyn Any>> {
		unsafe {(*self.dict.as_ptr()).get_mut(key)}
	}

	pub fn get_mut_t<T: Any>(&self, key: &'static str) -> Option<Box<&mut T>> {
		match self.get_mut(key).map(|value| value.downcast_mut::<T>()) {
			None => None,
			Some(value) => match value {
				None => None,
				Some(value) => Some(Box::new(value)),
			}
		}
	}

	pub fn remove(&self, key: &'static str) -> Option<Box<dyn Any>> {
		self.dict.borrow_mut().remove(key)
	}
}