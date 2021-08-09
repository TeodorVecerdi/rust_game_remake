use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub static ALL_DIFFICULTY_SETTINGS: &[&str] = &["easy", "normal", "hard"];

lazy_static! {
    static ref DIFFICULTY_SETTINGS: std::collections::HashMap<&'static str, DifficultySettings> = ALL_DIFFICULTY_SETTINGS.iter().map(|&difficulty| {
		println!("Loading difficulty settings for {}", difficulty);
		let path = super::ASSETS_FOLDER.join(format!("data/difficulty_settings/{}.yaml", difficulty));
		if !path.exists() {
			init_difficulty_settings();
		}
		let settings = serde_yaml::from_reader(std::fs::File::open(path).unwrap()).unwrap();
		(difficulty, settings)
	}).collect();
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DifficultySettings {
	pub enemy_base_attribute_points: i32,
	pub enemy_attack_chance: i32,
	pub enemy_heal_chance: i32,
	pub enemy_do_nothing_chance: i32,
	pub enemy_evade_chance: i32,

	pub player_base_attribute_points: i32,
	pub player_focus_chance: i32,
	pub player_evade_chance: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum Difficulty {
	Easy,
	Normal,
	Hard,
}

impl Difficulty {
	pub const fn as_str(self) -> &'static str {
		match self {
			Difficulty::Easy => "easy",
			Difficulty::Normal => "normal",
			Difficulty::Hard => "hard",
		}
	}
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let display_name = match self {
			Difficulty::Easy => "Easy",
			Difficulty::Normal => "Normal",
			Difficulty::Hard => "Hard",
		};
        write!(f, "Difficulty[{}]", display_name)
    }
}

fn init_difficulty_settings () {
	use std::io::Write;

	fn create_and_save_as_yaml(name: &str, enemy_base_attribute_points: i32, enemy_attack_chance: i32, enemy_heal_chance: i32, enemy_do_nothing_chance: i32, enemy_evade_chance: i32, player_base_attribute_points: i32, player_focus_chance: i32, player_evade_chance: i32) {
		let difficulty_settings = DifficultySettings::new(enemy_base_attribute_points, enemy_attack_chance, enemy_heal_chance, enemy_do_nothing_chance, enemy_evade_chance, player_base_attribute_points, player_focus_chance, player_evade_chance);
		let yaml = serde_yaml::to_string(&difficulty_settings).unwrap();
		// create directories if they don't exist
		std::fs::create_dir_all(ASSETS_FOLDER.join("data/difficulty_settings")).unwrap();
		let mut file = std::fs::File::create(ASSETS_FOLDER.join(format!("data/difficulty_settings/{}.yaml", name))).unwrap();
		file.write_all(yaml.as_bytes()).unwrap();
	}

	create_and_save_as_yaml("easy", 3, 30, 30, 40, 5, 10, 70, 10);
	create_and_save_as_yaml("normal", 5, 45, 30, 25, 7, 7, 50, 7);
	create_and_save_as_yaml("hard", 8, 60, 32, 8, 10, 5, 20, 5);
}

impl DifficultySettings {
	pub fn new(
		enemy_base_attribute_points: i32, 
		enemy_attack_chance: i32, 
		enemy_heal_chance: i32, 
		enemy_do_nothing_chance: i32, 
		enemy_evade_chance: i32, 
		player_base_attribute_points: i32, 
		player_focus_chance: i32, 
		player_evade_chance: i32
	) -> DifficultySettings 
	{
		DifficultySettings {
			enemy_base_attribute_points,
			enemy_attack_chance,
			enemy_heal_chance,
			enemy_do_nothing_chance,
			enemy_evade_chance,
			player_base_attribute_points,
			player_focus_chance,
			player_evade_chance,
		}
	}

	pub fn difficulty_settings() -> &'static std::collections::HashMap<&'static str, DifficultySettings> {
		&DIFFICULTY_SETTINGS
	}
}
