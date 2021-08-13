use std::{cell::RefCell, time::{Duration, Instant}};
use rand::Rng;
use serde::{Serialize, Deserialize};
use crate::data::{self, CharacterStats, DifficultySettings};

pub const HEALTH_MULTIPLIER: i32 = 5;
pub const STATUS_EFFECT_TIME: Duration = Duration::from_secs_f64(1.5);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Turn {
    Player,
    Enemy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CharacterState {
    Idle,
    Hurt,
    Attack,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerAction {
    Attack,
    Heal,
    Focus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameUpdateResult {
    EnemyKilled,
    PlayerKilled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub character_type: String,
    pub state: CharacterState,
    
    pub health: i32,
    pub stats: CharacterStats,
    pub difficulty_settings: DifficultySettings,
    is_player: bool,
    
    #[serde(skip)]
    rng: RefCell<rand::rngs::ThreadRng>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub turn: RefCell<Turn>,
    
    pub enemies_killed: RefCell<u32>,
    pub is_player_focused: RefCell<bool>,

    pub waiting_for_player: RefCell<bool>,
    pub waiting_for_enemy: RefCell<bool>,

    pub player_status_text: RefCell<String>,
    pub enemy_status_text: RefCell<String>,

    pub difficulty_settings: DifficultySettings,

    info_text: RefCell<Vec<String>>,

    #[serde(skip, default = "default_instant")]
    pub wait_for_enemy_timer: RefCell<Instant>,
    #[serde(skip, default = "default_instant")]
    pub player_status_timer: RefCell<Instant>,
    #[serde(skip, default = "default_instant")]
    pub enemy_status_timer: RefCell<Instant>,
    #[serde(skip, default = "default_instant")]
    pub player_state_timer: RefCell<Instant>,
    #[serde(skip, default = "default_instant")]
    pub enemy_state_timer: RefCell<Instant>,


    pub player: RefCell<Box<Character>>,
    pub enemy: RefCell<Box<Character>>,

    #[serde(skip)]
    rng: RefCell<rand::rngs::ThreadRng>,
}

impl CharacterState {
    pub const fn image_id(&self) -> &str {
        match self {
            CharacterState::Idle => "idle",
            CharacterState::Hurt => "hurt",
            CharacterState::Attack => "attack",
        }
    }
}

impl Character {
    pub fn new(name: String, character_type: String, stats: CharacterStats, difficulty_settings: DifficultySettings) -> Character {
        Character {
            name,
            character_type,
            state: CharacterState::Idle,
            health: stats.vitality * HEALTH_MULTIPLIER,
            stats,
            difficulty_settings,
            is_player: true,
            rng: RefCell::new(rand::thread_rng()),
        }
    }

    pub fn as_enemy(self) -> Self {
        Self {
            name: self.name,
            character_type: self.character_type,
            state: CharacterState::Idle,
            health: self.stats.vitality * HEALTH_MULTIPLIER,
            stats: self.stats,
            difficulty_settings: self.difficulty_settings,
            is_player: false,
            rng: self.rng,
        }
    }

    pub fn get_max_health(&self) -> i32 {
        self.stats.vitality * HEALTH_MULTIPLIER
    }

    pub fn get_attack_power(&self) -> i32 {
        let mut power = self.rng.borrow_mut().gen_range(self.stats.attack-2..=self.stats.attack+2);
        if power <= 0 {
            power = 1;
        }
        power
    }

    pub fn get_defense_power(&self) -> i32 {
        let mut power = self.rng.borrow_mut().gen_range(self.stats.defense-2..=self.stats.defense+2);
        if power <= 0 {
            power = 1;
        }
        power
    }

    pub fn get_heal_power(&self) -> i32 {
        let mut power = self.rng.borrow_mut().gen_range(self.stats.stamina-5..=self.stats.stamina+5);
        if power <= 0 {
            power = 1;
        }
        power
    }

    pub fn can_evade(&self) -> bool {
        let chance = match self.is_player {
            true => self.difficulty_settings.player_evade_chance,
            false => self.difficulty_settings.enemy_evade_chance,
        };

        let mut evaded = false;
        for _ in 0..self.get_defense_power() {
            evaded |= self.rng.borrow_mut().gen_bool(chance);
            if evaded {
                break;
            }
        }
        evaded
    }

    pub fn take_damage(&mut self, damage: i32) -> bool {
        if self.can_evade() {
            println!("{} evaded the attack!", self.name);
            return true;
        }

        self.health -= damage;
        println!("{} took {} damage", self.name, damage);
        if self.health <= 0 {
            self.health = 0;
        }

        false
    }

    pub fn heal(&mut self, heal_amount: i32) {
        self.health += heal_amount;
        let max_health = self.get_max_health();
        if self.health > max_health {
            self.health = max_health;
        }
    }
}

impl GameData {
    pub fn new(player: Character, difficulty_settings: DifficultySettings) -> GameData {
        let mut rng = rand::thread_rng();
        let turn = match rng.gen_bool(0.5) {
            true => Turn::Enemy,
            false => Turn::Player,
        };

        let enemy = GameData::make_enemy(difficulty_settings);

        let data = GameData {
            turn: RefCell::new(turn.clone()),
            enemies_killed: RefCell::new(0),
            is_player_focused: RefCell::new(false),

            player_status_text: RefCell::new(match turn { Turn::Player => "Thinking...", Turn::Enemy => "" }.to_string()),
            enemy_status_text: RefCell::new(match turn { Turn::Enemy => "Thinking...", Turn::Player => "" }.to_string()),

            waiting_for_player: RefCell::new(false),
            waiting_for_enemy: RefCell::new(false),

            difficulty_settings,

            info_text: RefCell::new(Vec::new()),

            wait_for_enemy_timer: RefCell::new(Instant::now()),
            player_status_timer: RefCell::new(Instant::now()),
            enemy_status_timer: RefCell::new(Instant::now()),
            player_state_timer: RefCell::new(Instant::now()),
            enemy_state_timer: RefCell::new(Instant::now()),

            enemy: RefCell::new(Box::new(enemy)),
            player: RefCell::new(Box::new(player)),
            rng: RefCell::new(rng),
        };
        
        data.add_info_text(format!("~===== A wild {} appeared! =====~", data.enemy.borrow().name));

        data
    }

    pub fn update(&self) -> Option<GameUpdateResult> {
        let turn = self.turn.borrow_mut().clone();
        
        let is_waiting_for_player = self.waiting_for_player.borrow().clone();
        let is_waiting_for_enemy = self.waiting_for_enemy.borrow().clone();
        let is_player_status_timer_done = self.player_status_timer.borrow().saturating_duration_since(Instant::now()).is_zero();
        let is_enemy_status_timer_done = self.enemy_status_timer.borrow().saturating_duration_since(Instant::now()).is_zero();
        let is_wait_for_enemy_timer_done = self.wait_for_enemy_timer.borrow().saturating_duration_since(Instant::now()).is_zero();
        let is_player_state_timer_done = self.player_state_timer.borrow().saturating_duration_since(Instant::now()).is_zero();
        let is_enemy_state_timer_done = self.enemy_state_timer.borrow().saturating_duration_since(Instant::now()).is_zero();
        
        if turn == Turn::Player && !is_waiting_for_player {
            *self.waiting_for_player.borrow_mut() = true;
            if is_player_status_timer_done {
                *self.player_status_text.borrow_mut() = "Thinking...".to_string();
            }
        } else if turn == Turn::Enemy && !is_waiting_for_enemy {
            *self.waiting_for_enemy.borrow_mut() = true;
            if is_enemy_status_timer_done {
                *self.enemy_status_text.borrow_mut() = "...Thinking".to_string();
            }

            *self.wait_for_enemy_timer.borrow_mut() = Instant::now() + Duration::from_secs_f64(self.rng.borrow_mut().gen_range(1.0..4.0));
        }


        if is_player_status_timer_done { 
            if is_waiting_for_enemy {
                *self.player_status_text.borrow_mut() = "".to_string();
            } else {
                *self.player_status_text.borrow_mut() = "Thinking...".to_string();
            }
        }

        if is_enemy_status_timer_done {
            if is_waiting_for_player {
                *self.enemy_status_text.borrow_mut() = "".to_string();
            } else {
                *self.enemy_status_text.borrow_mut() = "...Thinking".to_string();
            }
        }

        if is_player_state_timer_done {
            self.player.borrow_mut().state = CharacterState::Idle;
        }

        if is_enemy_state_timer_done {
            self.enemy.borrow_mut().state = CharacterState::Idle;
        }

        if is_waiting_for_enemy && is_wait_for_enemy_timer_done {
            *self.waiting_for_enemy.borrow_mut() = false;
            self.enemy_act();
            self.next_turn();
        }

        if self.enemy.borrow().health <= 0 {
            println!("Enemy is dead!");
            let kills = self.enemies_killed.borrow().clone();
            *self.enemies_killed.borrow_mut() = kills + 1;
            *self.enemy.borrow_mut() = Box::new(GameData::make_enemy(self.difficulty_settings));
            self.add_info_text(format!("~===== A wild {} appeared! =====~", self.enemy.borrow().name));
            return Some(GameUpdateResult::EnemyKilled);
        }

        if self.player.borrow().health <= 0 {
            println!("Player is dead!");
            return Some(GameUpdateResult::PlayerKilled);
        }

        return None;
    }
    
    pub fn player_act(&self, action: PlayerAction) {
        match action {
            PlayerAction::Attack => {
                self.player_act_attack();
                *self.player_status_timer.borrow_mut() = Instant::now() + STATUS_EFFECT_TIME;
            }
            PlayerAction::Heal => {
                self.player_act_heal();
                *self.player_status_text.borrow_mut() = "Healing!".to_string();    
                *self.player_status_timer.borrow_mut() = Instant::now() + STATUS_EFFECT_TIME;
            },
            PlayerAction::Focus => {
                if self.player_act_focus() {
                    *self.is_player_focused.borrow_mut() = true;
                    *self.player_status_text.borrow_mut() = "Focusing!".to_string();
                    *self.player_status_timer.borrow_mut() = Instant::now() + STATUS_EFFECT_TIME;
                }
            },
        };

        *self.waiting_for_player.borrow_mut() = false;
        self.next_turn()
    }

    pub fn should_disable_player_controls(&self) -> bool {
        *self.turn.borrow() == Turn::Enemy
    }

    pub fn get_info_text(&self) -> &Vec<String> {
        unsafe {&*self.info_text.as_ptr()}
    }

    fn player_act_attack(&self) {
        self.player.borrow_mut().state = CharacterState::Attack;
        *self.player_state_timer.borrow_mut() = Instant::now() + Duration::from_secs_f64(self.rng.borrow_mut().gen_range(1.0..2.5));


        let is_player_focused = self.is_player_focused.borrow().clone();
        let mut attack_power = self.player.borrow().get_attack_power();
        if is_player_focused {
            attack_power += self.rng.borrow_mut().gen_range(0 ..= attack_power / 2);
        }

        let mut evaded = self.enemy.borrow_mut().take_damage(attack_power);
        if evaded && is_player_focused {
            evaded = self.enemy.borrow_mut().take_damage(attack_power);
            if evaded {
                *self.player_status_text.borrow_mut() = "Missed!".to_string();
                self.add_info_text(format!("{} (you) tried to attack {}, but missed!", self.player.borrow().name, self.enemy.borrow().name));
            }
        } else if evaded {
            *self.player_status_text.borrow_mut() = "Missed!".to_string();
            self.add_info_text(format!("{} (you) tried to attack {}, but missed!", self.player.borrow().name, self.enemy.borrow().name));
        } else {
            *self.player_status_text.borrow_mut() = "Attacking!".to_string();
            self.enemy.borrow_mut().state = CharacterState::Hurt;
            *self.enemy_state_timer.borrow_mut() = Instant::now() + Duration::from_secs_f64(self.rng.borrow_mut().gen_range(1.0..2.5));
            self.add_info_text(format!("{} (you) attacked {} for {} damage!", self.player.borrow().name, self.enemy.borrow().name, attack_power));
        }

        if is_player_focused {
            *self.is_player_focused.borrow_mut() = false;
            self.add_info_text(format!("{} (you) is no longer focused.", self.player.borrow().name));
        }
    }

    fn player_act_heal(&self) {
        let mut heal_power = self.player.borrow().get_heal_power();
        let is_player_focused = self.is_player_focused.borrow().clone();
        if is_player_focused {
            heal_power += self.rng.borrow_mut().gen_range(0 ..= heal_power / 2);
            *self.is_player_focused.borrow_mut() = false;
            self.add_info_text(format!("{} (you) is no longer focused.", self.player.borrow().name));
        }

        self.player.borrow_mut().heal(heal_power);
        self.add_info_text(format!("{} (you) healed for {}!", self.player.borrow().name, heal_power));
    }

    fn player_act_focus(&self) -> bool {
        if !self.rng.borrow_mut().gen_bool(self.difficulty_settings.player_focus_chance) {
            self.add_info_text(format!("{} (you) tried to focus, but failed.", self.player.borrow().name));
            return false;
        }

        *self.is_player_focused.borrow_mut() = true;
        self.add_info_text(format!("{} (you) successfully focused. Their next action will be twice as powerful.", self.player.borrow().name));
        true
    }

    fn enemy_act(&self) {
        let chance: f64 = self.rng.borrow_mut().gen_range(0.0 .. 1.0);
        if chance < self.difficulty_settings.enemy_attack_chance {
            self.enemy.borrow_mut().state = CharacterState::Attack;
            *self.enemy_state_timer.borrow_mut() = Instant::now() + Duration::from_secs_f64(self.rng.borrow_mut().gen_range(1.0..2.5));

            let attack_power = self.enemy.borrow().get_attack_power();
            let evaded = self.player.borrow_mut().take_damage(attack_power);
            if evaded {
                *self.enemy_status_text.borrow_mut() = "Missed!".to_string();
                self.add_info_text(format!("{} tried to attack {} (you), but missed!", self.enemy.borrow().name, self.player.borrow().name));
            } else {
                self.player.borrow_mut().state = CharacterState::Hurt;
                *self.player_state_timer.borrow_mut() = Instant::now() + Duration::from_secs_f64(self.rng.borrow_mut().gen_range(1.0..2.5));
                *self.enemy_status_text.borrow_mut() = "Attacking!".to_string();
                self.add_info_text(format!("{} attacked {} (you) for {} damage!", self.enemy.borrow().name, self.player.borrow().name, attack_power));
            }
            *self.enemy_status_timer.borrow_mut() = Instant::now() + STATUS_EFFECT_TIME;
        } else if chance < self.difficulty_settings.enemy_attack_chance + self.difficulty_settings.enemy_heal_chance {
            let heal_power = self.enemy.borrow().get_heal_power();
            self.enemy.borrow_mut().heal(heal_power);

            *self.enemy_status_text.borrow_mut() = "Healing!".to_string();
            *self.enemy_status_timer.borrow_mut() = Instant::now() + STATUS_EFFECT_TIME;
            self.add_info_text(format!("{} healed for {}!", self.enemy.borrow().name, heal_power));
        } else {
            *self.enemy_status_text.borrow_mut() = "Trembling in fear!".to_string();
            *self.enemy_status_timer.borrow_mut() = Instant::now() + STATUS_EFFECT_TIME;
        }
    }

    fn next_turn(&self) {
        let current_turn = self.turn.borrow().clone();
        *self.turn.borrow_mut() = match current_turn {
            Turn::Player => Turn::Enemy,
            Turn::Enemy => Turn::Player,
        };
    }

    fn add_info_text(&self, text: String) {
        if self.info_text.borrow().len() == 10 {
            self.info_text.borrow_mut().remove(0);
        }

        self.info_text.borrow_mut().push(text);
    }

    fn make_enemy(difficulty_settings: DifficultySettings) -> Character {
        let mut rng = rand::thread_rng();
        let enemy_assigned_stats = data::CharacterStats::random(&mut rng, difficulty_settings.enemy_base_attribute_points);
        let enemy_type_idx = rng.gen_range(0..=data::CHARACTER_TYPE_COUNT);
        let (enemy_type, enemy_name) = match enemy_type_idx {
            0 => ("adventurer", "Enemy adventurer"),
            1 => ("female", "Enemy female"),
            2 => ("player", "Enemy player"),
            3 => ("soldier", "Enemy soldier"),
            _ => ("zombie", "Enemy zombie"),
        };
        
        let enemy_stats = data::CharacterStats::base_character_stats()[enemy_type] + enemy_assigned_stats;
        
        Character::new(enemy_name.to_string(), enemy_type.to_string(), enemy_stats, difficulty_settings.clone()).as_enemy()
    }
}

fn default_instant() -> RefCell<Instant> {
    RefCell::new(Instant::now())
}