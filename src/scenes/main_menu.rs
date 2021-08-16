use std::time::{Duration, Instant};

use crate::scenes::game::actions::{CharacterState, GameData, Turn};
use crate::{
	Scene, SceneManager,
	generate_scene,
	data, theme,
	};

use conrod_core::position::{Align, Place};
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
		button_continue,
		button_erase_data,
		button_quit,

		button_change_theme,
		text_change_theme,

		leaderboard_container,
		leaderboard_title,
		leaderboard_text_0,
		leaderboard_text_1,
		leaderboard_text_2,
		leaderboard_text_3,
		leaderboard_text_4,
		leaderboard_text_5,
		leaderboard_text_6,
		leaderboard_text_7,
		leaderboard_text_8,
		leaderboard_text_9,
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
		data_store: &data::DataStore,
	) {
		let ids = &self.ids;

		const BUTTON_HEIGHT: f64 = 64.0;
		const BUTTON_WIDTH: f64 = 320.0;
		const BUTTON_SPACING: f64 = 8.0;
		const TOTAL_BUTTONS_HEIGHT: f64 = BUTTON_HEIGHT * 4.0 + BUTTON_SPACING * 3.0;
		const TITLE_HEIGHT: f64 = 56.0;

		const LEADERBOARD_SPACING_X: f64 = 48.0;
		const LEADERBOARD_SPACING_Y: f64 = 96.0;
		
		let win_height = ui.win_h;
		let remaining_height = win_height - TOTAL_BUTTONS_HEIGHT - 2.0 * TITLE_HEIGHT;
		let vertical_spacing = remaining_height / 2.0;

		let leaderboard_width = ui.win_w / 2.0 - BUTTON_WIDTH / 2.0 - LEADERBOARD_SPACING_X * 2.0;
		let leaderboard_height = ui.win_h - LEADERBOARD_SPACING_Y * 2.0;

		let leaderboard;
		{
			leaderboard = *data_store.get_t::<data::Leaderboard>("leaderboard").unwrap();
		}

		let has_save_data = crate::ASSETS_FOLDER.join("data/runtime/current_game.yaml").exists();
		let has_leaderboard = !leaderboard.is_empty;
		
		widget::Canvas::new()
			.color(theme.background)
			.border(0.0)
			.w_h(ui.win_w, ui.win_h)
			.set(ids.root, ui);


		widget::Text::new("Main Menu")
			.color(theme.text_primary)
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
			.press_color(theme.button_press)
			.label_font_size(28)
			.label_color(theme.text_secondary)
			.label_font_id(*fonts.get("lato").unwrap());

		if base_button.clone() 
			.label("Play")
			.mid_top_with_margin_on(ids.title, TITLE_HEIGHT + BUTTON_SPACING * 2.0)
			.set(ids.button_play, ui)
			.was_clicked() {
				println!("Play");
				self.next_scene_index = Some(SceneManager::DIFFICULTY_SELECTION);
			}

		let mut continue_button 
			= base_button.clone()
			.label("Continue")
			.mid_bottom_of(ids.button_play)
			.down(BUTTON_SPACING);

		if !has_save_data {
			continue_button = continue_button
			.color(theme.button_disabled)
			.hover_color(theme.button_disabled)
			.press_color(theme.button_disabled);
		}

		if continue_button
			.set(ids.button_continue, ui)
			.was_clicked() {
				if has_save_data {
					let file = std::fs::File::open(crate::ASSETS_FOLDER.join("data/runtime/current_game.yaml")).unwrap();
					let game_data: GameData = serde_yaml::from_reader(file).unwrap();
					game_data.player.borrow_mut().state = CharacterState::Idle;
					game_data.enemy.borrow_mut().state = CharacterState::Idle;

					data_store.set("game_data", game_data);

					self.next_scene_index = Some(SceneManager::GAME);
				}
			}

		let mut erase_data_button 
			= base_button.clone()
			.label("Erase Data")
			.mid_bottom_of(ids.button_continue)
			.down(BUTTON_SPACING);

		if !has_save_data && !has_leaderboard {
			erase_data_button = erase_data_button
			.color(theme.button_disabled)
			.hover_color(theme.button_disabled)
			.press_color(theme.button_disabled);
		}	

			
		if erase_data_button
			.set(ids.button_erase_data, ui)
			.was_clicked() {
				#[allow(unused_must_use)]
				if has_save_data || has_leaderboard {
					if has_save_data {
						std::fs::remove_file(crate::ASSETS_FOLDER.join("data/runtime/current_game.yaml"));
					}

					if has_leaderboard {
						std::fs::remove_file(crate::ASSETS_FOLDER.join("data/runtime/leaderboard.yaml"));
						data_store.remove("leaderboard");
						data_store.set("leaderboard", data::Leaderboard::make(Some(10)));
					}

					scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
				}
			}

		if base_button.clone()
			.label("Quit")
			.mid_bottom_of(ids.button_erase_data)
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
			.label_color(theme.text_primary)
			.label_font_id(*fonts.get("lato").unwrap())
			.label_x(Relative::Place(Place::Start(Some(64.0 + 8.0))))
			.w_h(64.0, 64.0)
			.bottom_left_with_margins_on(ids.root, 8.0, 16.0)
			.border(0.0)
			.set(ids.button_change_theme, ui)
			.was_clicked()
		{
			{
				data_store.set("should_toggle_theme", ());
			}
			scene_manager.wake_up_events_loop().unwrap_or_else(|e| eprintln!("wakeup error: {}", e));
		}

		widget::Rectangle::fill_with([leaderboard_width, leaderboard_height], theme.panel_dark)
			.y_align_to(ids.root, Align::Middle)
			.x_place_on(ids.root, Place::Start(Some(LEADERBOARD_SPACING_X)))
			.set(ids.leaderboard_container, ui);

		widget::Text::new("Leaderboard")
			.font_size(48)
			.font_id(*fonts.get("lato").unwrap())
			.color(theme.text_primary)
			.x_align_to(ids.leaderboard_container, Align::Middle)
			.y_place_on(ids.leaderboard_container, Place::End(Some(8.0)))
			.set(ids.leaderboard_title, ui);

		let text_ids = [
			ids.leaderboard_text_0, ids.leaderboard_text_1, ids.leaderboard_text_2, ids.leaderboard_text_3,
			ids.leaderboard_text_4, ids.leaderboard_text_5, ids.leaderboard_text_6, ids.leaderboard_text_7,
			ids.leaderboard_text_8, ids.leaderboard_text_9,
		];

		let leaderboard_text_height = leaderboard_height - 16.0 * 2.0 /* Top+bottom spacing */ - 48.0 - 16.0 /* Title spacing */;
		let leaderboard_text_spacing = leaderboard_text_height / leaderboard.capacity as f64;

		let entry0_text: String = match leaderboard.get(0) {
			None => String::from(" 1. "),
			Some(entry) => format!("{:>2}. [{}] {} - {}", 1, entry.difficulty.as_str(), entry.name, entry.score),
		};
		widget::Text::new(&entry0_text)
			.font_size(24)
			.font_id(*fonts.get("lato").unwrap())
			.color(theme.text_primary)
			.x_place_on(ids.leaderboard_container, Place::Start(Some(16.0)))
			.y_place_on(ids.leaderboard_container, Place::End(Some(64.0 + 16.0)))
			.h(0.0)
			.set(text_ids[0], ui);

		for i in 1..leaderboard.capacity {
			let entry_text: String = match leaderboard.get(i) {
				None => format!("{:>2}. ", i + 1),
				Some(entry) => format!("{:>2}. [{}] {} - {}", i + 1, entry.difficulty.as_str(), entry.name, entry.score),
			};
			widget::Text::new(&entry_text)
				.font_size(24)
				.font_id(*fonts.get("lato").unwrap())
				.color(theme.text_primary)
				.x_place_on(ids.leaderboard_container, Place::Start(Some(16.0)))
				.y_place_on(ids.leaderboard_container, Place::End(Some(i as f64 * leaderboard_text_spacing + 64.0 + 16.0)))
				.h(0.0)
				.set(text_ids[i], ui);
		}
	}

	fn reset_switch_request(&mut self) { self.next_scene_index = None; }
	fn get_scene_switch_index(&self) -> std::option::Option<usize> { self.next_scene_index }
}
