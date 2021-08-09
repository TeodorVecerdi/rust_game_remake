use conrod_core::{
	widget, Colorable, Labelable, Positionable, Sizeable, Borderable, Ui, UiCell, Widget,
};

use crate::{
	theme, data, generate_scene, 
	scenes::{Scene, SceneManager}, 
};

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

generate_scene!(DifficultySelection -> Ids);

impl Scene for DifficultySelection {
	fn build(
		&mut self, 
		ui: &mut UiCell, 
		images: &std::collections::HashMap<String, conrod_core::image::Id>,
		image_map: &conrod_core::image::Map::<glium::texture::SrgbTexture2d>, 
		fonts: &std::collections::HashMap<&str, conrod_core::text::font::Id>, 
		scene_manager: &SceneManager,
		theme: &theme::Theme,
		data_store: &mut data::DataStore,
	) {
		let ids = &self.ids;

		const BUTTON_HEIGHT: f64 = 48.0;
		const BUTTON_WIDTH: f64 = 256.0;
		const BUTTON_SPACING: f64 = 8.0;
		const TOTAL_BUTTONS_HEIGHT: f64 = BUTTON_HEIGHT * 3.0 + BUTTON_SPACING * 2.0;
		const TITLE_HEIGHT: f64 = 64.0;
		
		let win_height = ui.win_h;
		let remaining_height = win_height - TOTAL_BUTTONS_HEIGHT - 2.0 * TITLE_HEIGHT;
		let vertical_spacing = remaining_height / 2.0;

		widget::Canvas::new()
			.color(theme.background)
			.border(0.0)
			.w_h(ui.win_w, ui.win_h)
			.set(ids.root, ui);

		widget::Text::new("Select difficulty")
			.font_id(*fonts.get("lato").unwrap())
			.color(theme.text_primary)
			.align_middle_y_of(ids.root)
			.align_middle_x()
			.up(-vertical_spacing)
			.h(0.0)
			.font_id(*fonts.get("lato").unwrap())
			.font_size((TITLE_HEIGHT / 2.0) as u32)
			// .mid_top_with_margin_on(ids.root, vertical_spacing)
			.set(ids.title, ui);

		let base_button = widget::Button::new()
			.w_h(BUTTON_WIDTH, BUTTON_HEIGHT)
			.color(theme.button_normal)
			.hover_color(theme.button_hover)
			.press_color(theme.button_press)
			.border(0.0)
			.label_font_size(24)
			.label_color(theme.text_secondary)
			.label_font_id(*fonts.get("lato").unwrap())
			;
		
		if base_button.clone()
			.label("Easy")
			.mid_top_with_margin_on(ids.title, BUTTON_HEIGHT)
			.set(ids.easy, ui)
			.was_clicked() {
				data_store.set("difficulty", data::Difficulty::Easy);
				self.next_scene_index = Some(SceneManager::CHARACTER_CREATION);
			}

		if base_button.clone()
			.label("Normal")
			.mid_bottom_with_margin_on(ids.easy, -(BUTTON_SPACING + BUTTON_HEIGHT))
			.set(ids.normal, ui)
			.was_clicked() {
				data_store.set("difficulty",data::Difficulty::Normal);
				self.next_scene_index = Some(SceneManager::CHARACTER_CREATION);
			}

		if base_button.clone()
			.label("Hard")
			.mid_bottom_with_margin_on(ids.normal, -(BUTTON_SPACING + BUTTON_HEIGHT))
			.set(ids.hard, ui)
			.was_clicked() {
				data_store.set("difficulty",data::Difficulty::Hard);
				self.next_scene_index = Some(SceneManager::CHARACTER_CREATION);
			}

		if widget::Button::new()
			.color(theme.button_normal)
			.hover_color(theme.button_hover)
			.press_color(theme.button_press)
			.label_color(theme.text_secondary)
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