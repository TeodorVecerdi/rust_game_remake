mod main_menu;
mod test_scene;
use std::cell::RefCell;
pub use main_menu::MainMenu;
pub use test_scene::TestScene;

pub trait Scene {
	fn get_scene_switch_index(&self) -> Option<usize>;
	fn reset_switch_request(&mut self);
	fn build(&mut self, ui: &mut conrod_core::UiCell, fonts: &std::collections::HashMap<&str, conrod_core::text::font::Id>, scene_manager: &SceneManager);
}

pub struct SceneManager {
	scenes: Vec<RefCell<Box<dyn Scene>>>,
	current_scene: usize,
}


impl SceneManager {
	pub fn new(ui: &mut conrod_core::Ui) -> Self {
		Self {
			scenes: vec![
				RefCell::new(Box::new(MainMenu::new(ui))),
				RefCell::new(Box::new(TestScene::new(ui))),
				],
			current_scene: 0,
		}
	}

	// switch scene
	pub fn switch_scene(&mut self, ui: &conrod_core::UiCell, scene: usize) {
		self.scenes.iter().for_each(|scene| scene.borrow_mut().reset_switch_request());
		if scene < self.scenes.len() {
			self.current_scene = scene;
			return;
		}
		
		eprintln!("Scene at index {} does not exist!", scene);
	}

	// build scene
	pub fn build(&mut self, ui: &mut conrod_core::UiCell, fonts: &std::collections::HashMap<&str, conrod_core::text::font::Id>) {
		// immutable reference to self
		let mut switch_index: Option<usize> = None;
		match self.scenes[self.current_scene].borrow().get_scene_switch_index() {
			Some(scene) => {
				switch_index = Some(scene);
			},
			None => {},
		}
		if let Some(scene) = switch_index {
			self.switch_scene(ui, scene);
		}
		self.scenes[self.current_scene].borrow_mut().build(ui, fonts, &self);
	}

	// add scene
	pub fn add_scene(&mut self, scene: Box<dyn Scene>) {
		self.scenes.push(RefCell::new(scene));
	}

	// remove scene 
	pub fn remove_scene(&mut self, scene: usize, replacement_scene: usize) {
		if scene < self.scenes.len() {
			self.scenes.remove(scene);
		}
		
		if replacement_scene < scene {
			self.current_scene = replacement_scene;
		} else if replacement_scene > scene {
			self.current_scene = replacement_scene - 1;
		} else {
			if self.scenes.len() == 0 {
				panic!("SceneManager: No scenes left!");
			}
			self.current_scene = 0;
		}
	}
}		
