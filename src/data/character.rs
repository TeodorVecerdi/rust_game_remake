use lazy_static::lazy_static;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub const CHARACTER_TYPE_COUNT: usize = 5;
pub static ALL_CHARACTER_TYPES: &[&str] = &["adventurer", "female", "player", "soldier", "zombie"];
pub static ALL_CHARACTER_STATES: &[&str] = &["attack", "hurt", "idle"];
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

fn init_character_stats () {
	use std::io::Write;

	fn create_and_save_as_yaml(name: &str, vitality: i32, attack: i32, defense: i32, stamina: i32) {
		let stats = CharacterStats::new(vitality, attack, defense, stamina);
		let yaml = serde_yaml::to_string(&stats).unwrap();
		// create directories if they don't exist
		std::fs::create_dir_all(super::ASSETS_FOLDER.join("data/base_character_stats")).unwrap();
		let mut file = std::fs::File::create(super::ASSETS_FOLDER.join(format!("data/base_character_stats/{}.yaml", name))).unwrap();
		file.write_all(yaml.as_bytes()).unwrap();
	}

	create_and_save_as_yaml("adventurer", 1, 1, 2, 2);
	create_and_save_as_yaml("female", 2, 1, 1, 2);
	create_and_save_as_yaml("player", 1, 2, 1, 2);
	create_and_save_as_yaml("soldier", 1, 2, 2, 1);
	create_and_save_as_yaml("zombie", 1, 3, 1, 1);
}

lazy_static! {
	static ref BASE_STATS: std::collections::HashMap<&'static str, CharacterStats> = ALL_CHARACTER_TYPES.iter().map(|&name| {
		println!("Loading character stats for {}", name);
		let path = super::ASSETS_FOLDER.join(format!("data/base_character_stats/{}.yaml", name));
		if !path.exists() {
			init_character_stats();
		}
		let stats = serde_yaml::from_reader(std::fs::File::open(path).unwrap()).unwrap();
		(name, stats)
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

	pub fn random(rng: &mut rand::prelude::ThreadRng, max_points: i32) -> CharacterStats {
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
	
		CharacterStats {
			vitality,
			attack,
			defense,
			stamina
		}
	}
}

impl std::ops::Add<CharacterStats> for CharacterStats {
    type Output = CharacterStats;

    fn add(self, rhs: CharacterStats) -> Self::Output {
        CharacterStats {
			vitality: self.vitality + rhs.vitality,
			attack: self.attack + rhs.attack,
			defense: self.defense + rhs.defense,
			stamina: self.stamina + rhs.stamina,
		}
    }
}