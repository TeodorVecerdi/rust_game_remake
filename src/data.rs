use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
	static ref ASSETS_FOLDER: std::path::PathBuf = find_folder::Search::ParentsThenKids(3, 5).for_folder("assets").unwrap();
}
pub const CHARACTER_TYPE_COUNT: usize = 5;
pub static ALL_CHARACTER_TYPES: &[&str] = &["adventurer", "female", "player", "soldier", "zombie"];
pub static ALL_CHARACTER_STATES: &[&str] = &["attack", "hurt", "idle"];
pub static ALL_DIFFICULTY_SETTINGS: &[&str] = &["easy", "normal", "hard"];
pub const CHARACTER_NAME_COUNT: usize = 60;
pub static ALL_CHARACTER_NAMES: [&str; CHARACTER_NAME_COUNT] = [
        "Sammie", "Regina", "Freddie", "Enrique", "Mignon", "Vanna", "Jaime", "Len", "Deloris", "Jodee", "Robby", "Mckenzie", "Rodrigo", "Emmett", "Cathryn", "Edmundo", "Darell",
        "Tyrell", "Hildegarde", "Julianne", "Marylou", "Andy", "Vilma", "Gala", "Linwood", "Riley", "Charlena", "Crissy", "Jeremy", "Ruby", "Williemae", "Ashlyn", "Elizabet",
        "Donte", "Gerry", "Rico", "Marinda", "Alfonso", "Shavon", "Solange", "Mayola", "Randy", "Richard", "Leonel", "Rufina", "Earnest", "Cortez", "Teodoro", "Rhett", "Ruthe",
        "Vicky", "Alice", "Yong", "Toya", "Machelle", "Jayne", "Zachariah", "Josie", "Steven", "Wilfredo"
];

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CharacterStats {
	pub vitality: i32,
	pub attack: i32,
	pub defense: i32,
	pub stamina: i32,
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


fn init_character_stats () {
	use std::io::Write;

	fn create_and_save_as_yaml(name: &str, vitality: i32, attack: i32, defense: i32, stamina: i32) {
		let stats = CharacterStats::new(vitality, attack, defense, stamina);
		let yaml = serde_yaml::to_string(&stats).unwrap();
		// create directories if they don't exist
		std::fs::create_dir_all(ASSETS_FOLDER.join("data/base_character_stats")).unwrap();
		let mut file = std::fs::File::create(ASSETS_FOLDER.join(format!("data/base_character_stats/{}.yaml", name))).unwrap();
		file.write_all(yaml.as_bytes()).unwrap();
	}

	create_and_save_as_yaml("adventurer", 1, 1, 2, 2);
	create_and_save_as_yaml("female", 2, 1, 1, 2);
	create_and_save_as_yaml("player", 1, 2, 1, 2);
	create_and_save_as_yaml("soldier", 1, 2, 2, 1);
	create_and_save_as_yaml("zombie", 1, 3, 1, 1);
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

lazy_static! {
	static ref BASE_STATS: std::collections::HashMap<&'static str, CharacterStats> = ALL_CHARACTER_TYPES.iter().map(|&name| {
		println!("Loading character stats for {}", name);
		let path = ASSETS_FOLDER.join(format!("data/base_character_stats/{}.yaml", name));
		if !path.exists() {
			init_character_stats();
		}
		let stats = serde_yaml::from_reader(std::fs::File::open(path).unwrap()).unwrap();
		(name, stats)
	}).collect();

	static ref DIFFICULTY_SETTINGS: std::collections::HashMap<&'static str, DifficultySettings> = ALL_DIFFICULTY_SETTINGS.iter().map(|&difficulty| {
		println!("Loading difficulty settings for {}", difficulty);
		let path = ASSETS_FOLDER.join(format!("data/difficulty_settings/{}.yaml", difficulty));
		if !path.exists() {
			init_difficulty_settings();
		}
		let settings = serde_yaml::from_reader(std::fs::File::open(path).unwrap()).unwrap();
		(difficulty, settings)
	}).collect();
}

impl CharacterStats {
	pub fn new(vitality: i32, attack: i32, defense: i32, stamina: i32) -> CharacterStats {
		CharacterStats {
			vitality,
			attack,
			defense,
			stamina,
		}
	}

	pub fn base_character_stats() -> &'static std::collections::HashMap<&'static str, CharacterStats> {
		&BASE_STATS
	}
}

impl DifficultySettings {
	pub fn new(enemy_base_attribute_points: i32, enemy_attack_chance: i32, enemy_heal_chance: i32, enemy_do_nothing_chance: i32, enemy_evade_chance: i32, player_base_attribute_points: i32, player_focus_chance: i32, player_evade_chance: i32) -> DifficultySettings {
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


pub struct DataStore {
	dict: std::collections::HashMap<&'static str, Box<dyn std::any::Any>>,
}

#[allow(dead_code)]
impl DataStore {
	pub fn new() -> DataStore {
		DataStore {
			dict: std::collections::HashMap::new(),
		}
	}

	pub fn has(&self, key: &str) -> bool {
		self.dict.get(key).is_some()
	}

	pub fn set_boxed<T: std::any::Any + std::fmt::Debug>(&mut self, key: &'static str, value: Box<T>) {
		self.dict.insert(key, value);
	}

	pub fn set<T: std::any::Any + std::fmt::Debug>(&mut self, key: &'static str, value: T) {
		self.dict.insert(key, Box::new(value));
	}

	pub fn get(&self, key: &'static str) -> Option<&Box<dyn std::any::Any>> {
		self.dict.get(key)
	}

	pub fn get_t<T: std::any::Any>(&self, key: &'static str) -> Option<Box<&T>> {
		match self.get(key).map(|value| value.downcast_ref::<T>()) {
			None => None,
			Some(value) => match value {
				None => None,
				Some(value) => Some(Box::new(value)),
			}
		}
	}

	pub fn get_mut(&mut self, key: &'static str) -> Option<&mut Box<dyn std::any::Any>> {
		self.dict.get_mut(key)
	}

	pub fn get_mut_t<T: std::any::Any>(&mut self, key: &'static str) -> Option<Box<&mut T>> {
		match self.get_mut(key).map(|value| value.downcast_mut::<T>()) {
			None => None,
			Some(value) => match value {
				None => None,
				Some(value) => Some(Box::new(value)),
			}
		}
	}

	pub fn remove(&mut self, key: &'static str) -> Option<Box<dyn std::any::Any>> {
		self.dict.remove(key)
	}
}