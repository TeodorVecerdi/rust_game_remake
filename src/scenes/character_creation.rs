use crate::{
    Scene, SceneManager, 
    generate_scene,
    data, theme
};

use conrod_core::position::Place;
use conrod_core::{
	position::{Align, Relative}, 
	widget,
	Borderable, Colorable, Labelable, Positionable, Sizeable, Ui, UiCell, Widget, 
};
use rand::Rng;

widget_ids! {
	pub struct Ids {
		root,

        left_col,
        right_col,
		
		title,

		button_create,
		button_back,

        image,
		
        text_name,
        text_character_type,
        text_remaining_points,
        text_vitality,
        text_attack,
        text_defense,
        text_stamina,

        button_randomize_name,
        button_randomize_stats,

        button_previous_character_type,
        button_next_character_type,

        button_increment_vitality,
        button_decrement_vitality,
        button_increment_attack,
        button_decrement_attack,
        button_increment_defense,
        button_decrement_defense,
        button_increment_stamina,
        button_decrement_stamina,
	}
}

generate_scene!(CharacterCreation);

impl Scene for CharacterCreation {
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
        let mut rng = rand::thread_rng();

        if !data_store.has("create_character_settings") {
            let character_name: usize = rng.gen_range(0..data::CHARACTER_NAME_COUNT);
            let character_type: usize = rng.gen_range(0..data::CHARACTER_TYPE_COUNT);
            data_store.set("create_character_settings", CreateCharacterSettings::new(character_name, character_type));
        }

        let difficulty_settings: data::DifficultySettings;
        let mut create_character_settings: &mut CreateCharacterSettings;
        let base_character_stats: data::CharacterStats;
        {
            difficulty_settings = *data::DifficultySettings::difficulty_settings().get(data_store.get_t::<data::Difficulty>("difficulty").unwrap().as_str()).unwrap();
            create_character_settings = *data_store.get_mut_t::<CreateCharacterSettings>("create_character_settings").unwrap();
            base_character_stats = *data::CharacterStats::base_character_stats().get(data::ALL_CHARACTER_TYPES[create_character_settings.character_type]).unwrap();
        }
         
        
        let total_assigned_points = create_character_settings.assigned_stats.vitality + 
                                        create_character_settings.assigned_stats.attack + 
                                        create_character_settings.assigned_stats.defense + 
                                        create_character_settings.assigned_stats.stamina;

        let points_remaining = difficulty_settings.player_base_attribute_points - total_assigned_points;


        const TITLE_HEIGHT: f64 = 96.0;
        const BUTTON_HEIGHT: f64 = 48.0;

        widget::Canvas::new()
            .color(theme.background)
            .border(0.0)
            .w_h(ui.win_w, ui.win_h)
            .set(ids.root, ui);

        widget::Text::new("Create a Character")
            .color(theme.primary_text)
            .font_size(64)
            .font_id(*fonts.get("lato").unwrap())
            .x_align(Align::Middle)
            .y_place_on(ids.root, Place::End(Some(10.0)))
            .set(ids.title, ui);

        widget::Canvas::new()
            .color(conrod_core::color::TRANSPARENT)
            .border(0.0)
            .x_align_to(ids.root, Align::Start)
            .y_place_on(ids.root, Place::Start(None))
            .w_h(ui.win_w / 2.0, ui.win_h-TITLE_HEIGHT)
            .set(ids.left_col, ui);

        widget::Canvas::new()
            .color(conrod_core::color::TRANSPARENT)
            .border(0.0)
            .x_align_to(ids.root, Align::End)
            .y_place_on(ids.root, Place::Start(None))
            .w_h(ui.win_w / 2.0, ui.win_h-TITLE_HEIGHT)
            .set(ids.right_col, ui);

        widget::Text::new(&(String::from("Name: ") + data::ALL_CHARACTER_NAMES[create_character_settings.name]))
            .color(theme.primary_text)
            .font_size(32)
            .font_id(*fonts.get("lato").unwrap())
            .x_place_on(ids.left_col, Place::Start(Some(32.0)))
            .y_place_on(ids.left_col, Place::End(None))
            .set(ids.text_name, ui);

        let base_button = widget::Button::new()
            .color(theme.button_normal)
            .hover_color(theme.button_hover)
            .press_color(theme.button_pressed)
            .h(BUTTON_HEIGHT)
            .border(0.0)
            .label_font_size(24)
            .label_font_id(*fonts.get("lato").unwrap())
            .label_color(theme.secondary_text);

        if base_button.clone()
            .label("Randomize")
            .w(160.0)
            .y_place_on(ids.left_col, Place::End(None))
            .x_place_on(ids.left_col, Place::End(Some(24.0)))
            .label_y(Relative::Place(Place::Start(Some(16.0))))
            .set(ids.button_randomize_name, ui)
            .was_clicked()
        {
            let character_name: usize = rng.gen_range(0..data::CHARACTER_NAME_COUNT);
            create_character_settings.name = character_name;
            scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
        }

        let character_type = data::ALL_CHARACTER_TYPES[create_character_settings.character_type];
        let image_id = images.get(&(String::from(character_type) + "_idle")).unwrap();
        let image_size: f64 = {
            let v1 = ui.win_w / 2.0 - 512.0 + 64.0;
            let v2 = ui.win_h / 2.0 - 256.0;
            let v3 = ui.win_w / 2.0 - 64.0;
            if v1 < 0.0 {
                if v2 < 0.0 {
                    v3
                } else {
                    v2
                }
            } else {
                v1
            }
        };
        widget::Image::new(*image_id)
            .w_h(image_size, image_size)
            .y_place_on(ids.text_name, Place::End(Some(64.0)))
            .x_place_on(ids.left_col, Place::Middle)
            .set(ids.image, ui);

        widget::Text::new(&character_type.to_uppercase())
            .color(theme.primary_text)
            .font_size(32)
            .font_id(*fonts.get("lato").unwrap())
            .x_place_on(ids.left_col, Place::Middle)
            .y_place_on(ids.image, Place::Start(Some(-64.0)))
            .set(ids.text_character_type, ui);

        if base_button.clone()
            .label("<")
            .w_h(36.0, 36.0)
            .label_y(Relative::Place(Place::Start(Some(8.0))))
            .y_place_on(ids.text_character_type, Place::Middle)
            .x_place(Place::Start(Some(32.0)))
            .set(ids.button_previous_character_type, ui)
            .was_clicked()
        {
            if create_character_settings.character_type == 0 {
                create_character_settings.character_type = data::CHARACTER_TYPE_COUNT - 1;
            } else {
                create_character_settings.character_type = create_character_settings.character_type - 1;
            }
            scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
        }

        if base_button.clone()
            .label(">")
            .w_h(36.0, 36.0)
            .label_y(Relative::Place(Place::Start(Some(8.0))))
            .y_place_on(ids.text_character_type, Place::Middle)
            .x_place(Place::End(Some(32.0)))
            .set(ids.button_next_character_type, ui)
            .was_clicked()
        {
            create_character_settings.character_type = (create_character_settings.character_type + 1) % data::CHARACTER_TYPE_COUNT;
            scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
        }


        widget::Text::new(&format!("Remaining points: {}", points_remaining))
            .color(theme.primary_text)
            .font_size(32)
            .font_id(*fonts.get("lato").unwrap())
            .y_place_on(ids.right_col, Place::End(None))
            .x_place_on(ids.right_col, Place::Start(Some(24.0)))
            .set(ids.text_remaining_points, ui);
        
        widget::Text::new(&format!("Vitality: {}", create_character_settings.assigned_stats.vitality + base_character_stats.vitality))
            .color(theme.primary_text)
            .font_size(32)
            .font_id(*fonts.get("lato").unwrap())
            .y_place_on(ids.text_remaining_points, Place::End(Some(48.0)))
            .x_place_on(ids.right_col, Place::Start(Some(24.0)))
            .set(ids.text_vitality, ui);

        widget::Text::new(&format!("Attack: {}", create_character_settings.assigned_stats.attack + base_character_stats.attack))
            .color(theme.primary_text)
            .font_size(32)
            .font_id(*fonts.get("lato").unwrap())
            .y_place_on(ids.text_vitality, Place::End(Some(40.0)))
            .x_place_on(ids.right_col, Place::Start(Some(24.0)))
            .set(ids.text_attack, ui);

        widget::Text::new(&format!("Defense: {}", create_character_settings.assigned_stats.defense + base_character_stats.defense))
            .color(theme.primary_text)
            .font_size(32)
            .font_id(*fonts.get("lato").unwrap())
            .y_place_on(ids.text_attack, Place::End(Some(40.0)))
            .x_place_on(ids.right_col, Place::Start(Some(24.0)))
            .set(ids.text_defense, ui);

        widget::Text::new(&format!("Stamina: {}", create_character_settings.assigned_stats.stamina + base_character_stats.stamina))
            .color(theme.primary_text)
            .font_size(32)
            .font_id(*fonts.get("lato").unwrap())
            .y_place_on(ids.text_defense, Place::End(Some(40.0)))
            .x_place_on(ids.right_col, Place::Start(Some(24.0)))
            .set(ids.text_stamina, ui);

        if base_button.clone()
            .label("+")
            .w_h(36.0, 36.0)
            .label_y(Relative::Place(Place::Start(Some(8.0))))
            .y_place_on(ids.text_vitality, Place::Middle)
            .x_place_on(ids.right_col, Place::End(Some(32.0)))
            .set(ids.button_increment_vitality, ui)
            .was_clicked()
        {
            if points_remaining > 0 {
                create_character_settings.assigned_stats.vitality = create_character_settings.assigned_stats.vitality + 1;
                scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
            }
        }

        if base_button.clone()
            .label("-")
            .w_h(36.0, 36.0)
            .label_y(Relative::Place(Place::Start(Some(8.0))))
            .y_place_on(ids.text_vitality, Place::Middle)
            .x_place_on(ids.button_increment_vitality, Place::End(Some(44.0)))
            .set(ids.button_decrement_vitality, ui)
            .was_clicked()
        {
            if create_character_settings.assigned_stats.vitality > 0 {
                create_character_settings.assigned_stats.vitality = create_character_settings.assigned_stats.vitality - 1;
                scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
            }
        }

        if base_button.clone()
            .label("+")
            .w_h(36.0, 36.0)
            .label_y(Relative::Place(Place::Start(Some(8.0))))
            .y_place_on(ids.text_attack, Place::Middle)
            .x_place_on(ids.right_col, Place::End(Some(32.0)))
            .set(ids.button_increment_attack, ui)
            .was_clicked()
        {
            if points_remaining > 0 {
                create_character_settings.assigned_stats.attack = create_character_settings.assigned_stats.attack + 1;
                scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
            }
        }

        if base_button.clone()
            .label("-")
            .w_h(36.0, 36.0)
            .label_y(Relative::Place(Place::Start(Some(8.0))))
            .y_place_on(ids.text_attack, Place::Middle)
            .x_place_on(ids.button_increment_attack, Place::End(Some(44.0)))
            .set(ids.button_decrement_attack, ui)
            .was_clicked()
        {
            if create_character_settings.assigned_stats.attack > 0 {
                create_character_settings.assigned_stats.attack = create_character_settings.assigned_stats.attack - 1;
                scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
            }
        }

        if base_button.clone()
            .label("+")
            .w_h(36.0, 36.0)
            .label_y(Relative::Place(Place::Start(Some(8.0))))
            .y_place_on(ids.text_defense, Place::Middle)
            .x_place_on(ids.right_col, Place::End(Some(32.0)))
            .set(ids.button_increment_defense, ui)
            .was_clicked()
        {
            if points_remaining > 0 {
                create_character_settings.assigned_stats.defense = create_character_settings.assigned_stats.defense + 1;
                scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
            }
        }

        if base_button.clone()
            .label("-")
            .w_h(36.0, 36.0)
            .label_y(Relative::Place(Place::Start(Some(8.0))))
            .y_place_on(ids.text_defense, Place::Middle)
            .x_place_on(ids.button_increment_defense, Place::End(Some(44.0)))
            .set(ids.button_decrement_defense, ui)
            .was_clicked()
        {
            if create_character_settings.assigned_stats.defense > 0 {
                create_character_settings.assigned_stats.defense = create_character_settings.assigned_stats.defense - 1;
                scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
            }
        }

        if base_button.clone()
            .label("+")
            .w_h(36.0, 36.0)
            .label_y(Relative::Place(Place::Start(Some(8.0))))
            .y_place_on(ids.text_stamina, Place::Middle)
            .x_place_on(ids.right_col, Place::End(Some(32.0)))
            .set(ids.button_increment_stamina, ui)
            .was_clicked()
        {
            if points_remaining > 0 {
                create_character_settings.assigned_stats.stamina = create_character_settings.assigned_stats.stamina + 1;
                scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
            }
        }

        if base_button.clone()
            .label("-")
            .w_h(36.0, 36.0)
            .label_y(Relative::Place(Place::Start(Some(8.0))))
            .y_place_on(ids.text_stamina, Place::Middle)
            .x_place_on(ids.button_increment_stamina, Place::End(Some(44.0)))
            .set(ids.button_decrement_stamina, ui)
            .was_clicked()
        {
            if create_character_settings.assigned_stats.stamina > 0 {
                create_character_settings.assigned_stats.stamina = create_character_settings.assigned_stats.stamina - 1;
                scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
            }
        }

        if base_button.clone()
            .label("Randomize stats")
            .w(ui.win_w / 2.0 - 24.0 - 32.0)
            .x_place(Place::End(None))
            .y_place_on(ids.button_decrement_stamina, Place::End(Some(52.0)))
            .set(ids.button_randomize_stats, ui)
            .was_clicked()
        {
            create_character_settings.assigned_stats = randomize_stats(&mut rng, difficulty_settings.player_base_attribute_points);
            scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
        }

        if base_button.clone()
            .label("Back")
            .w(256.0)
            .x_place_on(ids.right_col, Place::End(Some(32.0)))
            .y_place(Place::Start(Some(32.0)))
            .set(ids.button_back, ui)
            .was_clicked()
        {
            data_store.remove("create_character_settings");
            self.next_scene_index = Some(SceneManager::DIFFICULTY_SELECTION);
            scene_manager.wake_up_events_loop().unwrap_or_else(|e|eprintln!("Failed to wake up events loop: {}", e));
        }

        if base_button.clone()
            .label("Create")
            .w(256.0)
            .x_place_on(ids.right_col, Place::End(Some(32.0)))
            .y_place_on(ids.button_back, Place::Start(Some(BUTTON_HEIGHT + 8.0)))
            .set(ids.button_create, ui)
            .was_clicked()
        {
            println!("Create character");
            todo!("Move to next scene")
        }

        

    }

    fn get_scene_switch_index(&self) -> Option<usize> {
        self.next_scene_index
    }

    fn reset_switch_request(&mut self) {
        self.next_scene_index = None;
    }
}

fn randomize_stats(rng: &mut rand::prelude::ThreadRng, max_points: i32) -> data::CharacterStats {
    let mut vitality: i32 = 0;
    let mut attack: i32 = 0;
    let mut defense: i32 = 0;
    let mut stamina: i32 = 0;

    for _ in 0..max_points {
        let stat = rng.gen_range(0..=3);
        match stat {
            0 => vitality += 1,
            1 => attack += 1,
            2 => defense += 1,
            _ => stamina += 1,
        };
    }

    data::CharacterStats {
        vitality,
        attack,
        defense,
        stamina
    }
}


#[derive(Debug, Clone, Copy)]
struct CreateCharacterSettings {
    pub name: usize,
    pub character_type: usize,
    pub assigned_stats: data::CharacterStats,
}

impl CreateCharacterSettings {
    fn new(name: usize, character_type: usize) -> CreateCharacterSettings {
        CreateCharacterSettings {
            name,
            character_type,
            assigned_stats: data::CharacterStats::new(0, 0, 0, 0),
        }
    }
}