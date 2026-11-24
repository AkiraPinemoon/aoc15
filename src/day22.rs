use std::{fs::File, io::Read, println, vec};

pub fn part_one() {
    let mut file = File::open("input_22").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut values = contents
        .lines()
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|str| str.parse::<usize>().unwrap());

    let boss_hp = values.next().unwrap();
    let boss_damage = values.next().unwrap();

    println!("boss hp: {}, damage: {}", boss_hp, boss_damage);

    let player_hp = 50;
    let player_mana = 500;

    let state = GameState {
        total_mana_spent: 0,
        player_hp,
        player_mana,
        boss_hp,
        boss_damage,
        timer_shield: None,
        timer_poison: None,
        timer_recharge: None,
    };

    let mut best = None;

    find_optimal_game(state, &mut best, Vec::new());

    println!("least mana used to win: {:?}", best);
}

pub fn part_two() {
    let mut file = File::open("input_22").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut values = contents
        .lines()
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|str| str.parse::<usize>().unwrap());

    let boss_hp = values.next().unwrap();
    let boss_damage = values.next().unwrap();

    println!("boss hp: {}, damage: {}", boss_hp, boss_damage);

    let player_hp = 50;
    let player_mana = 500;

    let state = GameState {
        total_mana_spent: 0,
        player_hp,
        player_mana,
        boss_hp,
        boss_damage,
        timer_shield: None,
        timer_poison: None,
        timer_recharge: None,
    };

    let mut best = None;

    find_optimal_hard_game(state, &mut best, Vec::new());

    println!("least mana used to win: {:?}", best);
}

pub fn find_optimal_game(mut state: GameState, best: &mut Option<usize>, history: Vec<Spell>) {
    // early return if already lost
    if state.has_lost() { return }

    // early return if worse than existing best
    if let Some(value) = best {
        if state.total_mana_spent >= *value { return; }
    }

    // apply effects before casting spell
    state.process_effects();

    // early return if already won
    if state.has_won() {
        println!("won with {} mana used: {:?}", state.total_mana_spent, history);
        if let Some(value) = best {
            if state.total_mana_spent < *value { *best = Some(state.total_mana_spent); }
        } else {
            *best = Some(state.total_mana_spent);
        }
    }

    // recursively try each spell
    for spell in Spell::all() {
        if state.can_cast(&spell) {
            let mut new_state = state.clone();
            new_state.cast(&spell);

            let mut new_history = history.clone();
            new_history.push(spell);

            // apply effects before boss turn
            new_state.process_effects();

            // return if already won
            if new_state.has_won() {
                println!("won with {} mana used: {:?}", new_state.total_mana_spent, new_history);
                if let Some(value) = best {
                    if new_state.total_mana_spent < *value { *best = Some(new_state.total_mana_spent); }
                } else {
                    *best = Some(new_state.total_mana_spent);
                }
            }

            new_state.boss_turn();

            find_optimal_game(new_state, best, new_history);
        }
    }
}

pub fn find_optimal_hard_game(mut state: GameState, best: &mut Option<usize>, history: Vec<Spell>) {
    state.player_hp = state.player_hp.saturating_sub(1);

    // early return if already lost
    if state.has_lost() { return }

    // early return if worse than existing best
    if let Some(value) = best {
        if state.total_mana_spent >= *value { return; }
    }

    // apply effects before casting spell
    state.process_effects();

    // early return if already won
    if state.has_won() {
        println!("won with {} mana used: {:?}", state.total_mana_spent, history);
        if let Some(value) = best {
            if state.total_mana_spent < *value { *best = Some(state.total_mana_spent); }
        } else {
            *best = Some(state.total_mana_spent);
        }
    }

    // recursively try each spell
    for spell in Spell::all() {
        if state.can_cast(&spell) {
            let mut new_state = state.clone();
            new_state.cast(&spell);

            let mut new_history = history.clone();
            new_history.push(spell);

            // apply effects before boss turn
            new_state.process_effects();

            // return if already won
            if new_state.has_won() {
                println!("won with {} mana used: {:?}", new_state.total_mana_spent, new_history);
                if let Some(value) = best {
                    if new_state.total_mana_spent < *value { *best = Some(new_state.total_mana_spent); }
                } else {
                    *best = Some(new_state.total_mana_spent);
                }
            }

            new_state.boss_turn();

            find_optimal_hard_game(new_state, best, new_history);
        }
    }
}

#[derive(Debug, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    pub fn all() -> Vec<Self> {
        vec![Self::MagicMissile, Self::Drain, Self::Shield, Self::Poison, Self::Recharge]
    }
}

#[derive(Debug, Clone)]
struct GameState {
    pub total_mana_spent: usize,
    pub player_hp: usize,
    pub player_mana: usize,
    pub boss_hp: usize,
    pub boss_damage: usize,
    pub timer_shield: Option<usize>,
    pub timer_poison: Option<usize>,
    pub timer_recharge: Option<usize>,
}

impl GameState {
    pub fn can_cast(&self, spell: &Spell) -> bool {
        match spell {
            Spell::MagicMissile => self.player_mana >= 53,
            Spell::Drain => self.player_mana >= 73,
            Spell::Shield => self.player_mana >= 113 && self.timer_shield.is_none(),
            Spell::Poison => self.player_mana >= 173 && self.timer_poison.is_none(),
            Spell::Recharge => self.player_mana >= 229 && self.timer_recharge.is_none(),
        }
    }

    pub fn cast(&mut self, spell: &Spell) {
        if !self.can_cast(spell) { panic!("tried to cast an invalid spell") }

        match spell {
            Spell::MagicMissile => {
                self.player_mana -= 53;
                self.total_mana_spent += 53;
                self.boss_hp = self.boss_hp.saturating_sub(4)
            },
            Spell::Drain => {
                self.player_mana -= 73;
                self.total_mana_spent += 73;
                self.boss_hp = self.boss_hp.saturating_sub(2);
                self.player_hp += 2;
            },
            Spell::Shield => {
                self.player_mana -= 113;
                self.total_mana_spent += 113;
                if self.timer_shield.is_some() { panic!("shield timer still active") }
                self.timer_shield = Some(6)
            },
            Spell::Poison => {
                self.player_mana -= 173;
                self.total_mana_spent += 173;
                if self.timer_poison.is_some() { panic!("poison timer still active") }
                self.timer_poison = Some(6)
            },
            Spell::Recharge => {
                self.player_mana -= 229;
                self.total_mana_spent += 229;
                if self.timer_recharge.is_some() { panic!("recharge timer still active") }
                self.timer_recharge = Some(5)
            },
        };
    }

    pub fn process_effects(&mut self) {
        if let Some(timer) = self.timer_shield {
            match timer {
                1 => self.timer_shield = None,
                _ => self.timer_shield = Some(timer - 1),
            }
        }

        if let Some(timer) = self.timer_poison {
            match timer {
                1 => self.timer_poison = None,
                _ => self.timer_poison = Some(timer - 1),
            }

            self.boss_hp = self.boss_hp.saturating_sub(3)
        }

        if let Some(timer) = self.timer_recharge {
            match timer {
                1 => self.timer_recharge = None,
                _ => self.timer_recharge = Some(timer - 1),
            }

            self.player_mana += 101;
        }
    }

    pub fn boss_turn(&mut self) {
        let defense = self.timer_shield.map_or(0, |_| 7);
        let damage = (self.boss_damage - defense).max(1);
        self.player_hp = self.player_hp.saturating_sub(damage);
    }

    pub fn has_won(&self) -> bool {
        self.boss_hp == 0
    }

    pub fn has_lost(&self) -> bool {
        self.player_hp == 0
    }
}
