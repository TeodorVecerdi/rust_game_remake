mod actions;

use crate::{Scene, SceneManager, data, generate_scene, math, theme};
use actions::{
    GameData, Character, PlayerAction, 
};

use conrod_core::{
    Borderable, Colorable, Labelable, Positionable, Sizeable, Ui, Widget, 
    position::{Place, Align}, 
    widget
};

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
		image_map: &conrod_core::image::Map::<glium::texture::SrgbTexture2d>, 
		fonts: &std::collections::HashMap<&str, conrod_core::text::font::Id>, 
		scene_manager: &SceneManager,
		theme: &theme::Theme,
		data_store: &mut data::DataStore,
	) {
        let ids = &self.ids;

        const PLAYER_HEALTH_TOTAL: f64 = 15.0;
        const ENEMY_HEALTH_TOTAL: f64 = 32.0;

        if !data_store.has("player_health") {
            data_store.set("player_health", PLAYER_HEALTH_TOTAL);
            data_store.set("player_health_current", PLAYER_HEALTH_TOTAL);
        }

        if !data_store.has("enemy_health") {
            data_store.set("enemy_health", ENEMY_HEALTH_TOTAL);
            data_store.set("enemy_health_current", ENEMY_HEALTH_TOTAL);
        }

        let mut player_health = (*(*data_store.get_t::<f64>("player_health").unwrap())).clone();
        let mut player_health_current = (*(*data_store.get_t::<f64>("player_health_current").unwrap())).clone();
        if math::abs(player_health - player_health_current) > 0.05 {
            player_health_current = math::lerp(player_health_current,player_health, 0.05);
            data_store.set("player_health_current", player_health_current);
            scene_manager.wake_up_events_loop().unwrap_or_else(|e| eprintln!("Failed to wake up events loop: {}", e));
        } else {
            player_health_current = player_health;
            data_store.set("player_health_current", player_health_current);
        }

        let mut enemy_health = (*(*data_store.get_t::<f64>("enemy_health").unwrap())).clone();
        let mut enemy_health_current = (*(*data_store.get_t::<f64>("enemy_health_current").unwrap())).clone();
        if math::abs(enemy_health - enemy_health_current) > 0.05 {
            enemy_health_current = math::lerp(enemy_health_current,enemy_health, 0.05);
            data_store.set("enemy_health_current", enemy_health_current);
            scene_manager.wake_up_events_loop().unwrap_or_else(|e| eprintln!("Failed to wake up events loop: {}", e));
        } else {
            enemy_health_current = enemy_health;
            data_store.set("enemy_health_current", enemy_health_current);
        }

        const HEALTHBAR_HEIGHT: f64 = 48.0;
        const STAT_HEIGHT: f64 = 96.0;
        const STAT_WIDTH: f64 = 64.0;
        
        const PANEL_SPACING: f64 = 64.0;
        const PANEL_MARGIN: f64 = 32.0;
        const PANEL_ELEMENT_MARGIN: f64 = 8.0;

        let vitality_id = images.get("stat_vitality").unwrap();
        let attack_id = images.get("stat_attack").unwrap();
        let defense_id = images.get("stat_defense").unwrap();
        let stamina_id = images.get("stat_stamina").unwrap();

        let panel_title_height = 64.0;
        let panel_height = ui.win_h / 2.0 - PANEL_SPACING * 2.0;
        let panel_width = ui.win_w / 2.0 - PANEL_MARGIN * 2.0 - PANEL_SPACING / 2.0;
        let image_size = panel_height - panel_title_height - PANEL_ELEMENT_MARGIN * 2.0;
        let button_height = (panel_height - panel_title_height - HEALTHBAR_HEIGHT - STAT_HEIGHT - PANEL_ELEMENT_MARGIN * 5.0) / 3.0;
        
        let player_image_id = images.get("player_idle").unwrap();
        let (player_image_w, player_image_h) = image_map.get(player_image_id).unwrap().dimensions();
        let player_image_ratio = player_image_w as f64 / player_image_h as f64;
        let player_image_width = image_size * player_image_ratio;
        let player_right_column_width = panel_width - player_image_width - PANEL_ELEMENT_MARGIN * 3.0;

        widget::Canvas::new()
            .color(theme.background)
            .border(0.0)
            .w_h(ui.win_w, ui.win_h)
            .set(ids.root, ui);

        // Player
        widget::Canvas::new()
            .color(theme.panel_dark)
            .border(0.0)
            .w_h(panel_width, panel_height - panel_title_height)
            .top_left_with_margins_on(ids.root, ui.win_h / 2.0 + panel_title_height - panel_height / 2.0, PANEL_MARGIN)
            .set(ids.player_container, ui);

        widget::Text::new("Player Name")
            .color(theme.text_primary)
            .font_size(32)
            .font_id(*fonts.get("lato").unwrap())
            .x_align_to(ids.player_container, Align::Start)
            .y_place_on(ids.player_container, Place::End(Some(-44.0)))
            .set(ids.player_text_name, ui);

        widget::Text::new("Thinking...")
            .color(theme.text_primary)
            .font_size(32)
            .font_id(*fonts.get("lato").unwrap())
            .x_align_to(ids.player_container, Align::End)
            .y_place_on(ids.player_container, Place::End(Some(-44.0)))
            .set(ids.player_text_status, ui);

        widget::Image::new(*player_image_id)
            .w_h(player_image_width, image_size)
            .x_place_on(ids.player_container, Place::Start(Some(PANEL_ELEMENT_MARGIN)))
            .y_place_on(ids.player_container, Place::Start(Some(PANEL_ELEMENT_MARGIN)))
            .set(ids.player_image, ui);

        healthbar (
            player_health, 
            player_health_current, 
            PLAYER_HEALTH_TOTAL, 
            player_right_column_width, 
            HEALTHBAR_HEIGHT, 
            ids.player_healthbar_background, 
            ids.player_healthbar_fill, 
            ids.player_healthbar_text, 
            ui, 
            theme, fonts
        )
        .y_place_on(ids.player_container, Place::End(Some(PANEL_ELEMENT_MARGIN)))
        .x_place_on(ids.player_container, Place::Start(Some(player_image_width + PANEL_ELEMENT_MARGIN * 2.0)))
        .set(ids.player_healthbar_background, ui);

        stat (
            1,
            *vitality_id,
            STAT_WIDTH,
            STAT_HEIGHT,
            PANEL_ELEMENT_MARGIN / 2.0,
            ids.player_stat_vitality_container,
            ids.player_stat_vitality_image,
            ids.player_stat_vitality_text,
            ui,
            theme,
            fonts
        )
        .y_place_on(ids.player_container, Place::End(Some(HEALTHBAR_HEIGHT + PANEL_ELEMENT_MARGIN * 2.0)))
        .x_place_on(ids.player_container, Place::Start(Some(player_image_width + PANEL_ELEMENT_MARGIN * 2.0)))
        .set(ids.player_stat_vitality_container, ui);

        stat (
            2,
            *attack_id,
            STAT_WIDTH,
            STAT_HEIGHT,
            PANEL_ELEMENT_MARGIN / 2.0,
            ids.player_stat_attack_container,
            ids.player_stat_attack_image,
            ids.player_stat_attack_text,
            ui,
            theme,
            fonts
        )
        .y_place_on(ids.player_container, Place::End(Some(HEALTHBAR_HEIGHT + PANEL_ELEMENT_MARGIN * 2.0)))
        .x_place_on(ids.player_container, Place::Start(Some(player_image_width + STAT_WIDTH + PANEL_ELEMENT_MARGIN * 2.5)))
        .set(ids.player_stat_attack_container, ui);

        stat (
            2,
            *defense_id,
            STAT_WIDTH,
            STAT_HEIGHT,
            PANEL_ELEMENT_MARGIN / 2.0,
            ids.player_stat_defense_container,
            ids.player_stat_defense_image,
            ids.player_stat_defense_text,
            ui,
            theme,
            fonts
        )
        .y_place_on(ids.player_container, Place::End(Some(HEALTHBAR_HEIGHT + PANEL_ELEMENT_MARGIN * 2.0)))
        .x_place_on(ids.player_container, Place::Start(Some(player_image_width + STAT_WIDTH * 2.0 + PANEL_ELEMENT_MARGIN * 3.0)))
        .set(ids.player_stat_defense_container, ui);

        stat (
            1,
            *stamina_id,
            STAT_WIDTH,
            STAT_HEIGHT,
            PANEL_ELEMENT_MARGIN / 2.0,
            ids.player_stat_stamina_container,
            ids.player_stat_stamina_image,
            ids.player_stat_stamina_text,
            ui,
            theme,
            fonts
        )
        .y_place_on(ids.player_container, Place::End(Some(HEALTHBAR_HEIGHT + PANEL_ELEMENT_MARGIN * 2.0)))
        .x_place_on(ids.player_container, Place::Start(Some(player_image_width + STAT_WIDTH * 3.0 + PANEL_ELEMENT_MARGIN * 3.5)))
        .set(ids.player_stat_stamina_container, ui);

        let base_button = widget::Button::new()
            .color(theme.button_normal)
            .hover_color(theme.button_hover)
            .press_color(theme.button_press)
            .h(button_height)
            .w(player_right_column_width)
            .border(0.0)
            .label_font_size(24)
            .label_font_id(*fonts.get("lato").unwrap())
            .label_color(theme.text_secondary);

        if base_button.clone()
            .label("ATTACK")
            .x_place_on(ids.player_container, Place::Start(Some(player_image_width + PANEL_ELEMENT_MARGIN * 2.0)))
            .y_place_on(ids.player_container, Place::End(Some(HEALTHBAR_HEIGHT + STAT_HEIGHT + PANEL_ELEMENT_MARGIN * 3.0)))
            .set(ids.player_act_attack, ui)
            .was_clicked()
        {
            println!("Attack");
        }

        if base_button.clone()
            .label("FOCUS")
            .x_place_on(ids.player_container, Place::Start(Some(player_image_width + PANEL_ELEMENT_MARGIN * 2.0)))
            .y_place_on(ids.player_container, Place::End(Some(HEALTHBAR_HEIGHT + STAT_HEIGHT + button_height + PANEL_ELEMENT_MARGIN * 3.5)))
            .set(ids.player_act_focus, ui)
            .was_clicked()
        {
            println!("Focus");
        }

        if base_button.clone()
            .label("HEAL")
            .x_place_on(ids.player_container, Place::Start(Some(player_image_width + PANEL_ELEMENT_MARGIN * 2.0)))
            .y_place_on(ids.player_container, Place::End(Some(HEALTHBAR_HEIGHT + STAT_HEIGHT + button_height * 2.0 + PANEL_ELEMENT_MARGIN * 4.0)))
            .set(ids.player_act_heal, ui)
            .was_clicked()
        {
            println!("Heal");
        }


        
        let enemy_image_id = images.get("soldier_idle").unwrap();
        let (enemy_image_w, enemy_image_h) = image_map.get(enemy_image_id).unwrap().dimensions();
        let enemy_image_ratio = enemy_image_w as f64 / enemy_image_h as f64;
        let enemy_image_width = image_size * enemy_image_ratio;
        let enemy_right_column_width = panel_width - enemy_image_width - PANEL_ELEMENT_MARGIN * 3.0;

        // Enemy
        widget::Canvas::new()
            .color(theme.panel_dark)
            .border(0.0)
            .w_h(panel_width, panel_height - panel_title_height)
            .top_right_with_margins_on(ids.root, ui.win_h / 2.0 + panel_title_height - panel_height / 2.0, PANEL_MARGIN)
            .set(ids.enemy_container, ui);

        widget::Text::new("Thinking...")
            .color(theme.text_primary)
            .font_size(32)
            .font_id(*fonts.get("lato").unwrap())
            .x_align_to(ids.enemy_container, Align::Start)
            .y_place_on(ids.enemy_container, Place::End(Some(-44.0)))
            .set(ids.enemy_text_status, ui);

        widget::Text::new("Enemy Name")
            .color(theme.text_primary)
            .font_size(32)
            .font_id(*fonts.get("lato").unwrap())
            .x_align_to(ids.enemy_container, Align::End)
            .y_place_on(ids.enemy_container, Place::End(Some(-44.0)))
            .set(ids.enemy_text_name, ui);

        widget::Image::new(*enemy_image_id)
            .w_h(enemy_image_width, image_size)
            .x_place_on(ids.enemy_container, Place::Start(Some(PANEL_ELEMENT_MARGIN)))
            .y_place_on(ids.enemy_container, Place::Start(Some(PANEL_ELEMENT_MARGIN)))
            .set(ids.enemy_image, ui);

        healthbar (
            enemy_health, 
            enemy_health_current, 
            ENEMY_HEALTH_TOTAL, 
            enemy_right_column_width, 
            HEALTHBAR_HEIGHT, 
            ids.enemy_healthbar_background, 
            ids.enemy_healthbar_fill, 
            ids.enemy_healthbar_text, 
            ui, 
            theme, fonts
        )
        .y_place_on(ids.enemy_container, Place::End(Some(PANEL_ELEMENT_MARGIN)))
        .x_place_on(ids.enemy_container, Place::Start(Some(enemy_image_width + PANEL_ELEMENT_MARGIN * 2.0)))
        .set(ids.enemy_healthbar_background, ui);

        stat (
            1,
            *vitality_id,
            STAT_WIDTH,
            STAT_HEIGHT,
            PANEL_ELEMENT_MARGIN / 2.0,
            ids.enemy_stat_vitality_container,
            ids.enemy_stat_vitality_image,
            ids.enemy_stat_vitality_text,
            ui,
            theme,
            fonts
        )
        .y_place_on(ids.enemy_container, Place::End(Some(HEALTHBAR_HEIGHT + PANEL_ELEMENT_MARGIN * 2.0)))
        .x_place_on(ids.enemy_container, Place::Start(Some(enemy_image_width + PANEL_ELEMENT_MARGIN * 2.0)))
        .set(ids.enemy_stat_vitality_container, ui);

        stat (
            2,
            *attack_id,
            STAT_WIDTH,
            STAT_HEIGHT,
            PANEL_ELEMENT_MARGIN / 2.0,
            ids.enemy_stat_attack_container,
            ids.enemy_stat_attack_image,
            ids.enemy_stat_attack_text,
            ui,
            theme,
            fonts
        )
        .y_place_on(ids.enemy_container, Place::End(Some(HEALTHBAR_HEIGHT + PANEL_ELEMENT_MARGIN * 2.0)))
        .x_place_on(ids.enemy_container, Place::Start(Some(enemy_image_width + STAT_WIDTH + PANEL_ELEMENT_MARGIN * 2.5)))
        .set(ids.enemy_stat_attack_container, ui);

        stat (
            2,
            *defense_id,
            STAT_WIDTH,
            STAT_HEIGHT,
            PANEL_ELEMENT_MARGIN / 2.0,
            ids.enemy_stat_defense_container,
            ids.enemy_stat_defense_image,
            ids.enemy_stat_defense_text,
            ui,
            theme,
            fonts
        )
        .y_place_on(ids.enemy_container, Place::End(Some(HEALTHBAR_HEIGHT + PANEL_ELEMENT_MARGIN * 2.0)))
        .x_place_on(ids.enemy_container, Place::Start(Some(enemy_image_width + STAT_WIDTH * 2.0 + PANEL_ELEMENT_MARGIN * 3.0)))
        .set(ids.enemy_stat_defense_container, ui);

        stat (
            1,
            *stamina_id,
            STAT_WIDTH,
            STAT_HEIGHT,
            PANEL_ELEMENT_MARGIN / 2.0,
            ids.enemy_stat_stamina_container,
            ids.enemy_stat_stamina_image,
            ids.enemy_stat_stamina_text,
            ui,
            theme,
            fonts
        )
        .y_place_on(ids.enemy_container, Place::End(Some(HEALTHBAR_HEIGHT + PANEL_ELEMENT_MARGIN * 2.0)))
        .x_place_on(ids.enemy_container, Place::Start(Some(enemy_image_width + STAT_WIDTH * 3.0 + PANEL_ELEMENT_MARGIN * 3.5)))
        .set(ids.enemy_stat_stamina_container, ui);
    
        if base_button.clone()
            .label("Flee Battle")
            .w_h(256.0, 48.0)
            .x_align_to(ids.player_container, Align::Start)
            .y_place_on(ids.player_container, Place::Start(Some(-PANEL_ELEMENT_MARGIN - 48.0)))
            .set(ids.button_flee, ui)
            .was_clicked()
        {
            println!("Flee Battle");
        }

        /* 

        if widget::Button::new()
            .color(theme.button_normal)
            .hover_color(theme.button_hover)
            .press_color(theme.button_pressed)
            .h(48.0)
            .w(160.0)
            .border(0.0)
            .y_align_to(ids.root, Align::Start)
            .x_align_to(ids.root, Align::Start)
            .label("Increase")
            .label_font_size(24)
            .label_font_id(*fonts.get("lato").unwrap())
            .label_color(theme.secondary_text)
            .set(ids.player_act_attack, ui) 
            .was_clicked() 
        {
            player_health += 1.0;
            if player_health > PLAYER_HEALTH_TOTAL {
                player_health = PLAYER_HEALTH_TOTAL;
            }
            data_store.set("player_health", player_health);
        }

        if widget::Button::new()
            .color(theme.button_normal)
            .hover_color(theme.button_hover)
            .press_color(theme.button_pressed)
            .h(48.0)
            .w(160.0)
            .border(0.0)
            .y_align_to(ids.root, Align::Start)
            .x_align_to(ids.root, Align::End)
            .label("Decrease")
            .label_font_size(24)
            .label_font_id(*fonts.get("lato").unwrap())
            .label_color(theme.secondary_text)
            .set(ids.player_act_focus, ui) 
            .was_clicked() 
        {
            player_health -= 1.0;
            if player_health < 0.0 {
                player_health = 0.0;
            }
            data_store.set("player_health", player_health);
        } */
    }

    fn get_scene_switch_index(&self) -> Option<usize> {
        self.next_scene_index
    }

    fn reset_switch_request(&mut self) {
        self.next_scene_index = None;
    }
}

fn healthbar(
    current_value: f64, current_fill_value: f64, max_value: f64, width: f64, height: f64, 
    background_id: widget::Id, 
    fill_id: widget::Id, 
    text_id: widget::Id, 
    ui: &mut conrod_core::UiCell, 
    theme: &theme::Theme, 
    fonts: &std::collections::HashMap<&str, conrod_core::text::font::Id>
) -> widget::Rectangle 
{
    let fill_amount = current_fill_value / max_value;
    let root_widget = widget::Rectangle::fill_with([width, height], theme.panel_dark);
    
    widget::Rectangle::fill_with([fill_amount * width, height], theme.accent_color)
        .y_align_to(background_id, Align::Middle)
        .x_align_to(background_id, Align::Start)
        .parent(background_id)
        .set(fill_id, ui);

    widget::Text::new(&format!("{}/{}", current_value as i32, max_value as i32))
        .font_size(24)
        .font_id(*fonts.get("lato").unwrap())
        .color(theme.text_light)
        .y_align_to(background_id, Align::Middle)
        .x_place_on(background_id, Place::End(Some(16.0)))
        .parent(fill_id)
        .set(text_id, ui);

    root_widget
}

fn stat (
    stat_value: u32,
    stat_image_id: conrod_core::image::Id,
    stat_width: f64,
    stat_height: f64,
    stat_margin: f64,
    background_id: widget::Id,
    image_id: widget::Id,
    text_id: widget::Id,
    ui: &mut conrod_core::UiCell, 
    theme: &theme::Theme, 
    fonts: &std::collections::HashMap<&str, conrod_core::text::font::Id>
) -> widget::Rectangle
{
    let stat_widget = widget::Rectangle::fill_with([stat_width, stat_height], theme.accend_color_secondary);

    widget::Image::new(stat_image_id)
        .w_h(stat_width - stat_margin * 2.0, stat_width - stat_margin * 2.0)
        .y_place_on(background_id, Place::End(Some(stat_margin)))
        .x_align_to(background_id, Align::Middle)
        .parent(background_id)
        .set(image_id, ui);

    widget::Text::new(&format!("{}", stat_value))
        .font_size(24)
        .font_id(*fonts.get("lato").unwrap())
        .color(theme.text_light)
        .y_place_on(background_id, Place::Start(Some(stat_margin)))
        .x_align_to(background_id, Align::Middle)
        .parent(background_id)
        .set(text_id, ui);

    stat_widget
}