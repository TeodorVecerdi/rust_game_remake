use super::Difficulty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Leaderboard {
    pub capacity: usize,
    pub entries: Vec<LeaderboardEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub name: String,
    pub score: u32,
    pub difficulty: Difficulty,
}

impl Leaderboard {
    pub fn get(capacity: Option<usize>) -> Self {
        Leaderboard::read_from_file(capacity)
    }

    pub fn add_entry(&mut self, name: &str, score: u32, difficulty: Difficulty) {
        let entry = LeaderboardEntry {
            name: String::from(name),
            score,
            difficulty
        };
        self.entries.push(entry);
        
        self.sort();
        self.remove_extras();
    }

    fn sort(&mut self) {
        self.entries.sort_by(|a, b| b.cmp(a));
    }

    fn remove_extras(&mut self) {
        while self.entries.len() > self.capacity {
            self.entries.pop();
        }
    }

    fn new(capacity: usize) -> Self {
        Leaderboard {
            capacity,
            entries: Vec::new(),
        }
    }

    fn read_from_file(capacity: Option<usize>) -> Self {
        let capacity = capacity.unwrap_or(10);
        let path = super::ASSETS_FOLDER.join(format!("data/runtime/leaderboard.yaml"));
        
        let parent = path.parent().unwrap();
        if !parent.exists() {
            std::fs::create_dir_all(parent);
        }

        let file = std::fs::File::open(path);
        match file {
            Err(err) => { 
                let leaderboard = Leaderboard::new(capacity);
                leaderboard.write_to_file();
                leaderboard
            }
            Ok(file) => { 
                serde_yaml::from_reader(file).unwrap()
            }
        }
    }

    fn write_to_file(&self) {
        let path = super::ASSETS_FOLDER.join(format!("data/runtime/leaderboard.yaml"));
        
        let parent = path.parent().unwrap();
        if !parent.exists() {
            std::fs::create_dir_all(parent);
        }

        let file = std::fs::File::open(path).unwrap();
        serde_yaml::to_writer(file, self);
    }
}

impl LeaderboardEntry {
    pub fn new(name: String, score: u32, difficulty: Difficulty) -> LeaderboardEntry {
        LeaderboardEntry { name, score, difficulty }
    }

    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let difficulty_cmp = self.difficulty.cmp(&other.difficulty);
        if difficulty_cmp != std::cmp::Ordering::Equal {
            return difficulty_cmp;
        }

        std::cmp::Ord::cmp(&self.score, &other.score)
    }
}