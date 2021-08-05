extern crate paste;

mod main_menu;
mod test_scene;
#[macro_use] mod macros;


use std::cell::RefCell;
use std::panic;
use paste::paste;
use crate::theme;

pub use main_menu::MainMenu;
pub use test_scene::TestScene;

pub trait Scene {
	fn get_scene_switch_index(&self) -> Option<usize>;
	fn reset_switch_request(&mut self);
	fn build(
		&mut self, 
		ui: &mut conrod_core::UiCell, 
		fonts: &std::collections::HashMap<&str, conrod_core::text::font::Id>, 
		scene_manager: &SceneManager,
		theme: &theme::Theme
	);
}

pub struct SceneManager {
	scenes: Vec<RefCell<Box<dyn Scene>>>,
	current_scene: usize,
}

generate_scene_collection!(MainMenu, TestScene);

impl SceneManager {
	pub fn set_starting_scene(&mut self, starting_scene: usize) {
		self.current_scene = starting_scene;
	}

	// switch scene
	pub fn switch_scene(&mut self, scene: usize) {
		self.scenes.iter().for_each(|scene| scene.borrow_mut().reset_switch_request());
		if scene < self.scenes.len() {
			self.current_scene = scene;
			return;
		}
		
		eprintln!("Scene at index {} does not exist!", scene);
	}

	// build scene
	pub fn build(&mut self, 
		ui: &mut conrod_core::UiCell, 
		fonts: &std::collections::HashMap<&str, conrod_core::text::font::Id>,
		theme: &theme::ThemeManager,
		events_loop: &glium::glutin::EventsLoopProxy
	) {
		self.scenes[self.current_scene].borrow_mut().build(ui, fonts, self, &theme.active_theme);
		
		let mut switch_index: Option<usize> = None;
		if let Some(scene) = self.scenes[self.current_scene].borrow().get_scene_switch_index() {
			switch_index = Some(scene);
		}
		if let Some(scene) = switch_index {
			self.switch_scene(scene);
			events_loop.wakeup().unwrap_or_else(|e| eprintln!("wakeup error: {}", e));
		}
	}

	// add scene
	pub fn add_scene(&mut self, scene: Box<dyn Scene>) -> &mut Self {
		self.scenes.push(RefCell::new(scene));
		self
	}

	// remove scene 
	pub fn remove_scene(&mut self, scene: usize, replacement_scene: usize) {
		if scene < self.scenes.len() {
			// remove from scenes at index scene
			self.scenes.remove(scene);
		}
		
		self.current_scene = match replacement_scene {
			r if r < scene => replacement_scene,
			r if r > scene => replacement_scene - 1,
			_ => {
				if replacement_scene < self.scenes.len() {
					panic!("SceneManager: No scenes left to replace removed scene with!");
				}
				0
			}
		};
	}
}		
