#[macro_use] use crate::{SceneManager, Scene};
use conrod_core::{
	color, widget, Colorable, Labelable, Positionable, Sizeable, Borderable, Ui, UiCell, Widget,
};

widget_ids! {
	pub struct Ids {
		root,
		title,
		back
	}
}

pub struct TestScene {
	ids: Ids,
	next_scene_index: Option<usize>,
}

impl TestScene {
	pub fn new(ui: &mut Ui) -> Self {
		Self {
			ids: Ids::new(ui.widget_id_generator()),
			next_scene_index: None,
		}
	}
}

impl Scene for TestScene {
	fn build(&mut self, ui: &mut UiCell, fonts: &std::collections::HashMap<&str, conrod_core::text::font::Id>, scene_manager: &SceneManager) {
		let ids = &self.ids;

		widget::Canvas::new()
			.color(color::DARK_CHARCOAL)
			.border(0.0)
			.w_h(ui.win_w, ui.win_h)
			.set(ids.root, ui);

		widget::Text::new("Test")
			.font_id(*fonts.get("lato").unwrap())
			.color(color::WHITE)
			.font_size(64)
			.middle_of(ids.root)
			.set(ids.title, ui);

		if widget::Button::new()
			.label("Back")
			.color(color::WHITE)
			.label_color(color::BLACK)
			.label_font_id(*fonts.get("lato").unwrap())
			.label_font_size(32)
			.mid_bottom_of(ids.root)
			.w_h(320.0, 64.0)
			.set(ids.back, ui)
			.was_clicked()
		{
			// switch scene to main menu (index 0)
			println!("Back");
			self.next_scene_index = Some(0);
		}
	}
	fn reset_switch_request(&mut self) { self.next_scene_index = None; }
	fn get_scene_switch_index(&self) -> std::option::Option<usize> { self.next_scene_index }
}