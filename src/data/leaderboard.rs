use super::Difficulty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Leaderboard {
    pub capacity: usize,
    pub entries: Vec<LeaderboardEntry>,
    #[serde(skip)]
    pub is_empty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub name: String,
    pub score: u32,
    pub difficulty: Difficulty,
}

impl Leaderboard {
    pub fn make(capacity: Option<usize>) -> Self {
        Leaderboard::read_from_file(capacity)
    }

    pub fn get(&self, index: usize) -> Option<&LeaderboardEntry> {
        self.entries.get(index)
    }

    pub fn add_entry(&mut self, entry: LeaderboardEntry) {
        self.entries.push(entry);
        
        self.sort();
        self.remove_extras();

        self.write_to_file();
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
            is_empty: true,
        }
    }

    #[allow(unused_must_use)]
    fn read_from_file(capacity: Option<usize>) -> Self {
        let capacity = capacity.unwrap_or(10);
        let path = super::ASSETS_FOLDER.join(format!("data/runtime/leaderboard.yaml"));
        
        let parent = path.parent().unwrap();
        if !parent.exists() {
            std::fs::create_dir_all(parent);
        }

        let file = std::fs::File::open(path);
        match file {
            Err(_) => { 
                let leaderboard = Leaderboard::new(capacity);
                leaderboard.write_to_file();
                leaderboard
            }
            Ok(file) => { 
                let mut leaderboard: Leaderboard = serde_yaml::from_reader(file).unwrap();
                leaderboard.is_empty = leaderboard.entries.len() == 0;
                leaderboard
            }
        }
    }
    
    #[allow(unused_must_use)]
    fn write_to_file(&self) {
        let path = super::ASSETS_FOLDER.join(format!("data/runtime/leaderboard.yaml"));
        
        let parent = path.parent().unwrap();
        if !parent.exists() {
            std::fs::create_dir_all(parent);
        }

        let file = std::fs::File::create(path).unwrap();
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