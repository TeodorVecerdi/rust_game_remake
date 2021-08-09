use lazy_static::lazy_static;

mod difficulty;
pub use difficulty
:: {
	ALL_DIFFICULTY_SETTINGS,
	Difficulty,
	DifficultySettings,
};

mod character;
pub use character
:: {
	ALL_CHARACTER_NAMES, ALL_CHARACTER_STATES, ALL_CHARACTER_TYPES,
	CHARACTER_NAME_COUNT, CHARACTER_TYPE_COUNT,
	CharacterStats
};

mod data_store;
pub use data_store
:: {
	DataStore
};

pub use super::ASSETS_FOLDER;