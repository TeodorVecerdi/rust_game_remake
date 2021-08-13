use conrod_core::{Borderable, Colorable, Labelable, Positionable, Sizeable, Ui, UiCell, Widget, position::{Align, Place, Relative}, widget};

use crate::{data::{self, Leaderboard, LeaderboardEntry}, generate_scene, scenes::{Scene, SceneManager}, theme};

widget_ids! {
	pub struct Ids {
		root,
		
		title,
        score,
		
		back
	}
}

generate_scene!(GameOver -> Ids);

impl Scene for GameOver {
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
        let leaderboard_entry = (*data_store.get_t::<LeaderboardEntry>("leaderboard_entry").unwrap()).clone();
        
        if !data_store.has("added_leaderboard_entry") {
            data_store.get_mut_t::<Leaderboard>("leaderboard").unwrap().add_entry(leaderboard_entry.clone());
            data_store.set("added_leaderboard_entry", true);
        }

        widget::Canvas::new()
			.color(theme.background)
			.border(0.0)
			.w_h(ui.win_w, ui.win_h)
			.set(ids.root, ui);

        widget::Text::new("YOU DIED!")
            .font_id(*fonts.get("lato").unwrap())
            .font_size(64)
            .color(theme.text_primary)
            .y(32.0)
            .x_align_to(ids.root, Align::Middle)
            .set(ids.title, ui);

        widget::Text::new(&format!("SCORE: {}", leaderboard_entry.score))
            .font_id(*fonts.get("lato").unwrap())
            .font_size(32)
            .color(theme.text_primary)
            .y_relative_to(ids.title, -64.0)
            .x_align_to(ids.root, Align::Middle)
            .set(ids.score, ui);

        if widget::Button::new()
            .color(theme.button_normal)
            .hover_color(theme.button_hover)
            .press_color(theme.button_press)
            .label_color(theme.text_secondary)
            .label("Back")
            .label_font_id(*fonts.get("lato").unwrap())
            .label_font_size(24)
            .mid_bottom_with_margin(24.0)
            .w_h(320.0, 48.0)
            .set(ids.back, ui)
            .was_clicked()
        {
            data_store.remove("leaderboard_entry");
            data_store.remove("added_leaderboard_entry");
            self.next_scene_index = Some(SceneManager::MAIN_MENU);
        }
    }

    fn reset_switch_request(&mut self) { self.next_scene_index = None; }
	fn get_scene_switch_index(&self) -> std::option::Option<usize> { self.next_scene_index }
}