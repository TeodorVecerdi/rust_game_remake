use crate::{
	Scene, SceneManager,
	generate_scene,
	data, theme,
	};

use conrod_core::position::Place;
use conrod_core::{
	position::Relative, 
	widget,
	Borderable, Colorable, Labelable, Positionable, Sizeable, Ui, UiCell, Widget, 
};

widget_ids! {
	pub struct Ids {
		root,
		
		title,
		button_play,
		button_options,
		button_quit,

		button_change_theme,
		text_change_theme,
	}
}

generate_scene!(MainMenu -> Ids);

impl Scene for MainMenu {
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

		if data_store.has("difficulty") {
			let difficulty = **data_store.get_t::<data::Difficulty>("difficulty").unwrap();
			println!("{}", difficulty);
			data_store.remove("difficulty");
		}

		const BUTTON_HEIGHT: f64 = 64.0;
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


		widget::Text::new("Main Menu")
			.color(theme.primary_text)
			.font_size(TITLE_HEIGHT as u32)
			.h(0.0)
			.font_id(*fonts.get("lato").unwrap())
			.mid_top_of(ids.root)
			.up(-vertical_spacing)
			.set(ids.title, ui);

		let base_button = widget::Button::new()
			.h(BUTTON_HEIGHT)
			.w(320.0)
			.border(0.0)
			.color(theme.button_normal)
			.hover_color(theme.button_hover)
			.press_color(theme.button_pressed)
			.label_font_size(28)
			.label_color(theme.secondary_text)
			.label_font_id(*fonts.get("lato").unwrap())
			;

		if base_button.clone() 
			.label("Play")
			.mid_top_with_margin_on(ids.title, TITLE_HEIGHT + BUTTON_SPACING * 2.0)
			.set(ids.button_play, ui)
			.was_clicked() {
				println!("Play");
				self.next_scene_index = Some(SceneManager::DIFFICULTY_SELECTION);
			}

		if base_button.clone()
			.label("Settings")
			.mid_bottom_of(ids.button_play)
			.down(BUTTON_SPACING)
			.set(ids.button_options, ui)
			.was_clicked() {
				println!("Settings");
			}

		if base_button.clone()
			.label("Quit")
			.mid_bottom_of(ids.button_options)
			.down(BUTTON_SPACING)
			.set(ids.button_quit, ui)
			.was_clicked()
		{
			std::process::exit(0);
		}

		let is_light_theme = match data_store.get_t::<bool>("is_light_theme") {
			Some(is_light_theme) => **is_light_theme,
			None => {
				println!("No light mode setting found, defaulting to light mode.");
				false
			}
		};
		
		let theme_image_id = *images.get(match is_light_theme { true => "light_mode_icon", false => "dark_mode_icon" }).unwrap();
		if widget::Button::image(theme_image_id)
			.label(match is_light_theme { true => "Switch to dark mode", false => "Switch to light mode"})
			.label_font_size(24)
			.label_color(theme.primary_text)
			.label_font_id(*fonts.get("lato").unwrap())
			.label_x(Relative::Place(Place::Start(Some(64.0 + 8.0))))
			.w_h(64.0, 64.0)
			.bottom_left_with_margins_on(ids.root, 8.0, 16.0)
			.border(0.0)
			.set(ids.button_change_theme, ui)
			.was_clicked()
		{
			data_store.set("should_toggle_theme", ());
			scene_manager.wake_up_events_loop().unwrap_or_else(|e| eprintln!("wakeup error: {}", e));
		}
	}

	fn reset_switch_request(&mut self) { self.next_scene_index = None; }
	fn get_scene_switch_index(&self) -> std::option::Option<usize> { self.next_scene_index }
}
