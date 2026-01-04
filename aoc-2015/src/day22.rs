use std::collections::HashMap;

use common::error::PuzzleError;

#[derive(Debug)]
struct Effect {
    duration: usize,
    armor: u64,
    damage_per_turn: u64,
    mana_per_turn: u64,
    buff: bool,
}

impl Effect {
    fn new(duration: usize, armor: u64, damage_per_turn: u64, mana_per_turn: u64, buff: bool) -> Self {
        Effect {
            duration,
            armor,
            damage_per_turn,
            mana_per_turn,
            buff,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct AppliedEffect {
    _effect: usize,
    remaining_duration: usize,
}

impl AppliedEffect {
    fn new(effect: usize, remaining_duration: usize) -> Self {
        AppliedEffect { _effect: effect, remaining_duration }
    }
}

#[derive(Debug)]
struct Spell {
    cost: u64,
    damage: u64,
    healing: u64,
    effect: Option<usize>,
}

impl Spell {
    fn new(cost: u64, damage: u64, healing: u64, effect: Option<usize>) -> Self {
        Spell { cost, damage, healing, effect }
    }
}

#[derive(Debug, Clone)]
struct Entity {
    hit_points: u64,
    armor: u64,
    mana: u64,
    effects: HashMap<usize, Option<AppliedEffect>>,
}

impl Entity {
    fn new(hit_points: u64, armor: u64, mana: u64) -> Self {
        Entity {
            hit_points,
            armor,
            mana,
            effects: HashMap::new(),
        }
    }

    fn is_alive(&self) -> bool {
        self.hit_points > 0
    }

    fn apply_effects(&mut self, effects: &Vec<Effect>) {
        self.armor = 0;
        let mut expired_effects = vec![];
        for (effect_index, applied_effect_option) in self.effects.iter_mut() {
            if let Some(applied_effect) = applied_effect_option {
                let effect = &effects[*effect_index];
                if effect.buff {
                    self.armor += effect.armor;
                    self.mana += effect.mana_per_turn;
                }
                else{
                    self.hit_points = self.hit_points.saturating_sub(effect.damage_per_turn);
                }

                applied_effect.remaining_duration -= 1;
                if applied_effect.remaining_duration == 0 {
                    expired_effects.push(*effect_index);
                }
            }
        }

        for effect_index in expired_effects {
            self.effects.insert(effect_index, None);
        }
    }

    fn can_cast_spell(&self, spell: &Spell, target: &Entity, effects: &Vec<Effect>) -> bool {
        if self.mana < spell.cost {
            return false;
        }

        if let Some(effect_index) = spell.effect {
            let effect = &effects[effect_index];
            let effects_lookup = if effect.buff {
                &self.effects
            }
            else{
                &target.effects
            };

            if let Some(applied_effect_option) = effects_lookup.get(&effect_index) {
                if let Some(applied_effect) = applied_effect_option {
                    if applied_effect.remaining_duration > 0 {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn cast_spell(&mut self, spell: &Spell, target: &mut Entity, effects: &Vec<Effect>) {
        self.mana -= spell.cost;
        if spell.damage > 0 {
            target.hit_points = target.hit_points.saturating_sub(spell.damage);
        }

        self.hit_points += spell.healing;

        if let Some(effect_index) = spell.effect {
            let effect = &effects[effect_index];
            let applied_effect = AppliedEffect::new(effect_index, effect.duration);
            if effect.buff {
                self.effects.insert(effect_index, Some(applied_effect));
            }
            else {
                target.effects.insert(effect_index, Some(applied_effect));
            }
        }
    }
}

pub fn solve_day22_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day22.txt")?;
    let mut lines = input.lines();
    let boss_hit_points = lines.next().unwrap().split(": ").nth(1).unwrap().parse::<u64>().unwrap();
    let boss_damage = lines.next().unwrap().split(": ").nth(1).unwrap().parse::<u64>().unwrap();
    let effects = vec![
        Effect::new(6, 7, 0, 0, true),   // Shield
        Effect::new(6, 0, 3, 0, false),  // Poison
        Effect::new(5, 0, 0, 101, true), // Recharge
    ];

    let spells = vec![
        Spell::new(53, 4, 0, None),               // Magic Missile
        Spell::new(73, 2, 2, None),               // Drain
        Spell::new(113, 0, 0, Some(0)),           // Shield
        Spell::new(173, 0, 0, Some(1)),           // Poison
        Spell::new(229, 0, 0, Some(2)),           // Recharge
    ];

    let boss_spell = Spell::new(0, boss_damage, 0, None);
    let player = Entity::new(50, 0, 500);
    let boss = Entity::new(boss_hit_points, 0, 0);
    let mut min_mana_spent = u64::MAX;
    solve_day22_puzzle_helper(player, boss, &effects, &spells, &boss_spell, 0, &mut min_mana_spent, false);
    println!("Minimum mana spent to win is {}", min_mana_spent);

    Ok(())
}

pub fn solve_day22_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day22.txt")?;
    let mut lines = input.lines();
    let boss_hit_points = lines.next().unwrap().split(": ").nth(1).unwrap().parse::<u64>().unwrap();
    let boss_damage = lines.next().unwrap().split(": ").nth(1).unwrap().parse::<u64>().unwrap();
    let effects = vec![
        Effect::new(6, 7, 0, 0, true),   // Shield
        Effect::new(6, 0, 3, 0, false),  // Poison
        Effect::new(5, 0, 0, 101, true), // Recharge
    ];

    let spells = vec![
        Spell::new(53, 4, 0, None),               // Magic Missile
        Spell::new(73, 2, 2, None),               // Drain
        Spell::new(113, 0, 0, Some(0)),           // Shield
        Spell::new(173, 0, 0, Some(1)),           // Poison
        Spell::new(229, 0, 0, Some(2)),           // Recharge
    ];

    let boss_spell = Spell::new(0, boss_damage, 0, None);
    let player = Entity::new(50, 0, 500);
    let boss = Entity::new(boss_hit_points, 0, 0);
    let mut min_mana_spent = u64::MAX;
    solve_day22_puzzle_helper(player, boss, &effects, &spells, &boss_spell, 0, &mut min_mana_spent, true);
    println!("Minimum mana spent to win is {}", min_mana_spent);

    Ok(())
}


fn solve_day22_puzzle_helper(player: Entity, boss: Entity, effects: &Vec<Effect>, spells: &Vec<Spell>, boss_spell: &Spell, mana_spent: u64, min_mana_spent: &mut u64, part2: bool) {
    for (spell_index, spell) in spells.iter().enumerate() {
        let current_mana_spent = mana_spent + spell.cost;

        if current_mana_spent >= *min_mana_spent {
            continue;
        }

        if player.mana < spell.cost {
            continue;
        }

        let mut player_clone = player.clone();
        let mut boss_clone = boss.clone();

        if !matches!(boss_clone.effects.get(&0), None | Some(None)) {
            println!("Boss should not have shield effect active");
        }

        // if !player_clone.can_cast_spell(spell, &boss_clone, effects) {
        //     continue;
        // }

        match execute_turn(&mut player_clone, &mut boss_clone, effects, spells, spell_index, boss_spell, part2) {
            Some(true) => {
                if current_mana_spent < *min_mana_spent {
                    *min_mana_spent = current_mana_spent;
                }
            },
            Some(false) => {
                continue;
            },
            None => {
                solve_day22_puzzle_helper(player_clone, boss_clone, effects, spells, boss_spell, current_mana_spent, min_mana_spent, part2);
            }
        }
    }
}

fn execute_turn(player: &mut Entity, boss: &mut Entity, effects: &Vec<Effect>, spells: &Vec<Spell>, spell_index: usize, boss_spell: &Spell, part2: bool) -> Option<bool> {
    // Player's turn

    if part2 {
        player.hit_points = player.hit_points.saturating_sub(1);
        if !player.is_alive() {
            return Some(false);
        }
    }

    // Apply effects
    player.apply_effects(effects);
    boss.apply_effects(effects);
    
    if !boss.is_alive() {
        return Some(true);
    }

    if !player.is_alive() {
        return Some(false);
    }

    if player.mana == 0 {
        return Some(false);
    }

    let spell = &spells[spell_index];
    if !player.can_cast_spell(spell, boss, effects) {
        return Some(false);
    }

    player.cast_spell(spell, boss, effects);

    if !boss.is_alive() {
        return Some(true);
    }

    // Boss's turn

    // Apply effects
    player.apply_effects(effects);
    boss.apply_effects(effects);
    
    if !boss.is_alive() {
        return Some(true);
    }

    if !player.is_alive() {
        return Some(false);
    }
    
    let damage = boss_spell.damage.saturating_sub(player.armor).max(1);
    player.hit_points = player.hit_points.saturating_sub(damage);

    if !player.is_alive() {
        return Some(false);
    }

    None
}

#[test]
fn test_day22_solver_part1() {
    match solve_day22_puzzle_part1() {
        Ok(_) => (),
        Err(e) => panic!("Error solving day 22 part 1: {}", e),
    }
}

#[test]
fn test_day22_solver_part2() {
    match solve_day22_puzzle_part2() {
        Ok(_) => (),
        Err(e) => panic!("Error solving day 22 part 2: {}", e),
    }
}