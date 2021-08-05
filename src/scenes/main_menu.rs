#[macro_use] use crate::{SceneManager, Scene};
use conrod_core::{
	color, widget, Colorable, Labelable, Positionable, Sizeable, Borderable, Ui, UiCell, Widget,
};
use crate::theme;

widget_ids! {
	pub struct Ids {
		root,
		
		header,
		content,
		content_play,
		content_options,
		content_quit,
		
		title,
		button_play,
		button_options,
		button_quit,
	}
}

pub struct MainMenu {
	ids: Ids,
	next_scene_index: Option<usize>,
}

impl MainMenu {
	pub fn new(ui: &mut Ui) -> Self {
		Self {
			ids: Ids::new(ui.widget_id_generator()),
			next_scene_index: None,
		}
	}
}

impl Scene for MainMenu {
	fn build(
		&mut self, 
		ui: &mut UiCell, 
		fonts: &std::collections::HashMap<&str, conrod_core::text::font::Id>, 
		scene_manager: &SceneManager,
		theme: &theme::Theme
	) {
		let ids = &self.ids;

		const BUTTON_HEIGHT: f64 = 64.0;
		const BUTTON_SPACING: f64 = 8.0;
		const TOTAL_BUTTONS_HEIGHT: f64 = BUTTON_HEIGHT * 3.0 + BUTTON_SPACING * 2.0;
		const TITLE_HEIGHT: f64 = 64.0;
		
		let win_height = ui.win_h;
		let remaining_height = win_height - TOTAL_BUTTONS_HEIGHT - 2.0 * TITLE_HEIGHT;
		let vertical_spacing = remaining_height / 2.0;
		
		widget::Canvas::new()
			.color(color::rgb(0.1, 0.1, 0.15))
			.border(0.0)
			.w_h(ui.win_w, ui.win_h)
			.set(ids.root, ui);


		widget::Text::new("Main Menu")
			.color(color::WHITE)
			.font_size(TITLE_HEIGHT as u32)
			.h(0.0)
			.font_id(*fonts.get("lato").unwrap())
			.mid_top_of(ids.root)
			.up(-vertical_spacing)
			.set(ids.title, ui);

		let base_button = widget::Button::new()
			.h(BUTTON_HEIGHT)
			.w(320.0)
			.color(color::WHITE)
			.label_color(color::BLACK)
			.label_font_size(28)
			.label_font_id(*fonts.get("lato").unwrap())
			.press_color(color::rgb(0.7, 0.7, 0.75))
			.hover_color(color::rgb(0.9, 0.9, 0.9));

		if base_button.clone() 
			.label("Play")
			.mid_top_with_margin_on(ids.title, TITLE_HEIGHT + BUTTON_SPACING * 2.0)
			.set(ids.button_play, ui)
			.was_clicked() {
				println!("Play");
				self.next_scene_index = Some(SceneManager::TEST_SCENE);
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
			println!("Exiting...");
			std::process::exit(0);
		}
	}
	fn reset_switch_request(&mut self) { self.next_scene_index = None; }
	fn get_scene_switch_index(&self) -> std::option::Option<usize> { self.next_scene_index }
}
