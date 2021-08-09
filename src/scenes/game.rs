use crate::{
    Scene, SceneManager,
    generate_scene,
    data, theme,
};

use conrod_core::{
    widget, position::{Place, Align},
    Borderable, Colorable, Labelable, Positionable, Sizeable, Ui, Widget,
};

use rand::Rng;

widget_ids! {
    pub struct Ids {
        root,

        player_container,
        enemy_container,

        button_flee,
        
        // Player 
        player_act_attack,
        player_act_focus,
        player_act_heal,

        player_healthbar_background,
        player_healthbar_fill,
        player_healthbar_text,

        player_focus_image,

        player_text_name,
        player_text_status,
        player_image,

        player_stat_vitality_container,
        player_stat_vitality_image,
        player_stat_vitality_text,

        player_stat_attack_container,
        player_stat_attack_image,
        player_stat_attack_text,

        player_stat_defense_container,
        player_stat_defense_image,
        player_stat_defense_text,

        player_stat_stamina_container,
        player_stat_stamina_image,
        player_stat_stamina_text,

        // Enemy
        enemy_healthbar_background,
        enemy_healthbar_fill,
        enemy_healthbar_text,

        enemy_text_name,
        enemy_text_status,
        enemy_image,

        enemy_stat_vitality_container,
        enemy_stat_vitality_image,
        enemy_stat_vitality_text,

        enemy_stat_attack_container,
        enemy_stat_attack_image,
        enemy_stat_attack_text,

        enemy_stat_defense_container,
        enemy_stat_defense_image,
        enemy_stat_defense_text,

        enemy_stat_stamina_container,
        enemy_stat_stamina_image,
        enemy_stat_stamina_text,
    }
}

generate_scene!(Game -> Ids);

impl Scene for Game {
    fn build(
		&mut self, 
		ui: &mut conrod_core::UiCell, 
		images: &std::collections::HashMap<String, conrod_core::image::Id>, 
		fonts: &std::collections::HashMap<&str, conrod_core::text::font::Id>, 
		scene_manager: &SceneManager,
		theme: &theme::Theme,
		data_store: &mut data::DataStore,
	) {
        let ids = &self.ids;

        widget::Canvas::new()
            .color(theme.background)
            .border(0.0)
            .w_h(ui.win_w, ui.win_h)
            .set(ids.root, ui);

    }

    fn get_scene_switch_index(&self) -> Option<usize> {
        self.next_scene_index
    }

    fn reset_switch_request(&mut self) {
        self.next_scene_index = None;
    }
}