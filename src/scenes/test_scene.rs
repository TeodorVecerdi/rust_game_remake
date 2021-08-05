#[macro_use] use crate::{SceneManager, Scene};
use conrod_core::{
	color, widget, Colorable, Labelable, Positionable, Sizeable, Borderable, Ui, UiCell, Widget,
};

use crate::theme;

widget_ids! {
	pub struct Ids {
		root,
		
		title,
		
		easy,
		normal,
		hard,
		
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
	fn build(
		&mut self, 
		ui: &mut UiCell, 
		fonts: &std::collections::HashMap<&str, conrod_core::text::font::Id>, 
		scene_manager: &SceneManager,
		theme: &theme::Theme
	) {
		let ids = &self.ids;

		widget::Canvas::new()
			.color(theme.background)
			.border(0.0)
			.w_h(ui.win_w, ui.win_h)
			.set(ids.root, ui);

		widget::Text::new("Select difficulty")
			.font_id(*fonts.get("lato").unwrap())
			.color(theme.primary_text)
			.font_size(32)
			.middle_of(ids.root)
			.set(ids.title, ui);

		if widget::Button::new()
			.color(theme.button_normal)
			.hover_color(theme.button_hover)
			.press_color(theme.button_pressed)
			.label_color(theme.secondary_text)
			.border(0.0)
			.label("Back")
			.label_font_id(*fonts.get("lato").unwrap())
			.label_font_size(24)
			.parent(ids.root)
			.mid_bottom_with_margin(24.0)
			.w_h(320.0, 48.0)
			.set(ids.back, ui)
			.was_clicked()
		{
			println!("Back");
			self.next_scene_index = Some(SceneManager::MAIN_MENU);
		}
	}
	fn reset_switch_request(&mut self) { self.next_scene_index = None; }
	fn get_scene_switch_index(&self) -> std::option::Option<usize> { self.next_scene_index }
}