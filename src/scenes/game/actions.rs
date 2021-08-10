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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character<'a> {
    pub name: &'a str,
    pub character_type: &'a str,
    pub state: CharacterState,
    
    pub health: i32,
    pub stats: CharacterStats,
    pub difficulty_settings: DifficultySettings,
    is_player: bool,
    
    #[serde(skip)]
    rng: RefCell<rand::rngs::ThreadRng>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData<'a, 'b> {
    pub turn: RefCell<Turn>,
    
    pub enemies_killed: u32,
    pub is_player_focused: RefCell<bool>,

    pub waiting_for_player: RefCell<bool>,
    pub waiting_for_enemy: RefCell<bool>,

    pub player_status_text: RefCell<&'a str>,
    pub enemy_status_text: RefCell<&'a str>,

    difficulty_settings: DifficultySettings,

    #[serde(skip, default = "default_instant")]
    pub wait_for_enemy_timer: RefCell<Instant>,
    #[serde(skip, default = "default_instant")]
    pub player_status_timer: RefCell<Instant>,
    #[serde(skip, default = "default_instant")]
    pub enemy_status_timer: RefCell<Instant>,

    #[serde(borrow)]
    pub player: RefCell<Box<Character<'a>>>,
    #[serde(borrow)]
    pub enemy: RefCell<Box<Character<'b>>>,

    #[serde(skip)]
    rng: RefCell<rand::rngs::ThreadRng>,
}

impl<'a> Character<'a> {
    pub fn new(name: &'a str, character_type: &'a str, stats: CharacterStats, difficulty_settings: DifficultySettings) -> Character<'a> {
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
            rng: self.rng.clone(),
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

impl<'a: 'b, 'b> GameData<'a, 'b> {
    pub fn new(player: Character<'a>, difficulty_settings: DifficultySettings) -> GameData<'a, 'b> {
        let mut rng = rand::thread_rng();
        let turn = match rng.gen_bool(0.5) {
            true => Turn::Enemy,
            false => Turn::Player,
        };

        let data = GameData {
            turn: RefCell::new(turn.clone()),
            enemies_killed: 0,
            is_player_focused: RefCell::new(false),

            player_status_text: RefCell::new(match turn { Turn::Player => "Thinking...", Turn::Enemy => "" }),
            enemy_status_text: RefCell::new(match turn { Turn::Enemy => "Thinking...", Turn::Player => "" }),

            waiting_for_player: RefCell::new(false),
            waiting_for_enemy: RefCell::new(false),

            difficulty_settings,

            wait_for_enemy_timer: RefCell::new(Instant::now()),
            player_status_timer: RefCell::new(Instant::now()),
            enemy_status_timer: RefCell::new(Instant::now()),

            enemy: RefCell::new(Box::new(player.clone())),
            player: RefCell::new(Box::new(player)),
            rng: RefCell::new(rng),
        };

        data.make_enemy();

        data
    }

    pub fn update(&self) {
        let turn = self.turn.borrow_mut().clone();
        
        let is_waiting_for_player = self.waiting_for_player.borrow().clone();
        let is_waiting_for_enemy = self.waiting_for_enemy.borrow().clone();
        let is_player_status_timer_done = self.player_status_timer.borrow().saturating_duration_since(Instant::now()).is_zero();
        let is_enemy_status_timer_done = self.enemy_status_timer.borrow().saturating_duration_since(Instant::now()).is_zero();
        let is_wait_for_enemy_timer_done = self.wait_for_enemy_timer.borrow().saturating_duration_since(Instant::now()).is_zero();
        
        if turn == Turn::Player && !is_waiting_for_player {
            *self.waiting_for_player.borrow_mut() = true;
            if is_player_status_timer_done {
                *self.player_status_text.borrow_mut() = "Thinking...";
            }
        } else if turn == Turn::Enemy && !is_waiting_for_enemy {
            *self.waiting_for_enemy.borrow_mut() = true;
            if is_enemy_status_timer_done {
                *self.enemy_status_text.borrow_mut() = "...Thinking";
            }

            *self.wait_for_enemy_timer.borrow_mut() = Instant::now() + Duration::from_secs_f64(self.rng.borrow_mut().gen_range(1.0..4.0));
        }


        if is_player_status_timer_done { 
            if is_waiting_for_enemy {
                *self.player_status_text.borrow_mut() = "";
            } else {
                *self.player_status_text.borrow_mut() = "Thinking...";
            }
        }

        if is_enemy_status_timer_done {
            if is_waiting_for_player {
                *self.enemy_status_text.borrow_mut() = "";
            } else {
                *self.enemy_status_text.borrow_mut() = "...Thinking";
            }
        }

        if is_waiting_for_enemy && is_wait_for_enemy_timer_done {
            *self.waiting_for_enemy.borrow_mut() = false;
            self.enemy_act();
            self.next_turn();
        }

        if self.enemy.borrow().health <= 0 {
            println!("Enemy is dead!");
            self.make_enemy();
        }

        if self.player.borrow().health <= 0 {
            println!("Player is dead!");
        }
    }
    
    pub fn player_act(&self, action: PlayerAction) {
        match action {
            PlayerAction::Attack |
            PlayerAction::Heal => {
                match action {
                    PlayerAction::Attack => { 
                        self.player_act_attack();
                        *self.player_status_text.borrow_mut() = "Attacking!";
                    },
                    _ => { 
                        self.player_act_heal();
                        *self.player_status_text.borrow_mut() = "Healing!";
                    }
                };
                
                *self.player_status_timer.borrow_mut() = Instant::now() + STATUS_EFFECT_TIME;
            },
            PlayerAction::Focus => {
                if self.player_act_focus() {
                    *self.is_player_focused.borrow_mut() = true;
                    *self.player_status_text.borrow_mut() = "Focusing!";
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

    fn player_act_attack(&self) {
        self.player.borrow_mut().state = CharacterState::Attack;
        self.enemy.borrow_mut().state = CharacterState::Hurt;

        let is_player_focused = self.is_player_focused.borrow().clone();
        let mut attack_power = self.player.borrow().get_attack_power();
        if is_player_focused {
            attack_power += self.rng.borrow_mut().gen_range(0 ..= attack_power / 2);
        }

        let evaded = self.enemy.borrow_mut().take_damage(attack_power);
        if evaded && is_player_focused {
            self.enemy.borrow_mut().take_damage(attack_power);
        }

        if is_player_focused {
            *self.is_player_focused.borrow_mut() = false;
        }
    }

    fn player_act_heal(&self) {
        let mut heal_power = self.player.borrow().get_heal_power();
        let is_player_focused = self.is_player_focused.borrow().clone();
        if is_player_focused {
            heal_power += self.rng.borrow_mut().gen_range(0 ..= heal_power / 2);
            *self.is_player_focused.borrow_mut() = false;
        }

        self.player.borrow_mut().heal(heal_power);
    }

    fn player_act_focus(&self) -> bool {
        if !self.rng.borrow_mut().gen_bool(self.difficulty_settings.player_focus_chance) {
            return false;
        }

        *self.is_player_focused.borrow_mut() = true;
        true
    }

    fn enemy_act(&self) {
        let chance: f64 = self.rng.borrow_mut().gen_range(0.0 .. 1.0);
        if chance < self.difficulty_settings.enemy_attack_chance {
            self.enemy.borrow_mut().state = CharacterState::Attack;
            self.player.borrow_mut().state = CharacterState::Hurt;

            let attack_power = self.enemy.borrow().get_attack_power();
            self.player.borrow_mut().take_damage(attack_power);

            *self.enemy_status_text.borrow_mut() = "Attacking!";
            *self.enemy_status_timer.borrow_mut() = Instant::now() + STATUS_EFFECT_TIME;
        } else if chance < self.difficulty_settings.enemy_attack_chance + self.difficulty_settings.enemy_heal_chance {
            let heal_power = self.enemy.borrow().get_heal_power();
            self.enemy.borrow_mut().heal(heal_power);

            *self.enemy_status_text.borrow_mut() = "Healing!";
            *self.enemy_status_timer.borrow_mut() = Instant::now() + STATUS_EFFECT_TIME;
        } else {
            *self.enemy_status_text.borrow_mut() = "Trembling in fear!";
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

    fn make_enemy(&self) {
        let enemy_assigned_stats = data::CharacterStats::random(&mut *self.rng.borrow_mut(), self.difficulty_settings.enemy_base_attribute_points);
        let enemy_type_idx = self.rng.borrow_mut().gen_range(0..=data::CHARACTER_TYPE_COUNT);
        let (enemy_type, enemy_name) = match enemy_type_idx {
            0 => ("adventurer", "Enemy adventurer"),
            1 => ("female", "Enemy female"),
            2 => ("player", "Enemy player"),
            3 => ("soldier", "Enemy soldier"),
            _ => ("zombie", "Enemy zombie"),
        };
        
        let enemy_stats = data::CharacterStats::base_character_stats()[enemy_type] + enemy_assigned_stats;
        
        *self.enemy.borrow_mut() = Box::new(Character::new(enemy_name, enemy_type, enemy_stats, self.difficulty_settings.clone()));
    }
}

fn default_instant() -> RefCell<Instant> {
    RefCell::new(Instant::now())
}