use common::error::PuzzleError;

#[derive(Debug)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    fn new(cost: u32, damage: u32, armor: u32) -> Self {
        Item { cost, damage, armor }
    }

    fn empty() -> Self {
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        }
    }
}

#[derive(Debug)]
struct Entity {
    hit_points: u32,
    damage: u32,
    armor: u32,
}

impl Entity {
    fn new(hit_points: u32, damage: u32, armor: u32) -> Self {
        Entity {
            hit_points,
            damage,
            armor,
        }
    }

    fn is_alive(&self) -> bool {
        self.hit_points > 0
    }

    fn attack(&mut self, other: &mut Entity) {
        let damage_dealt = self.damage.saturating_sub(other.armor).max(1);
        other.hit_points = other.hit_points.saturating_sub(damage_dealt);
    }

    fn equip_items(&mut self, items: &[&Item]) {
        self.damage = 0;
        self.armor = 0;
        for item in items {
            self.damage += item.damage;
            self.armor += item.armor;
        }
    }
}

pub fn solve_day21_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day21.txt")?;
    let mut lines = input.lines();
    let hp = lines.next().unwrap().split(": ").nth(1).unwrap().parse::<u32>().unwrap();
    let damage_points = lines.next().unwrap().split(": ").nth(1).unwrap().parse::<u32>().unwrap();
    let armor_points = lines.next().unwrap().split(": ").nth(1).unwrap().parse::<u32>().unwrap();

    let weapons = vec![
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0),
    ];

    let armors = vec![
        Item::empty(),
        Item::new(13, 0, 1),
        Item::new(31, 0, 2),
        Item::new(53, 0, 3),
        Item::new(75, 0, 4),
        Item::new(102, 0, 5),
    ];

    let rings = vec![
        Item::empty(),
        Item::new(25, 1, 0),
        Item::new(50, 2, 0),
        Item::new(100, 3, 0),
        Item::new(20, 0, 1),
        Item::new(40, 0, 2),
        Item::new(80, 0, 3),
    ];

    let mut min_cost = u32::MAX;
    for weapon in &weapons {
        for armor in &armors {
            for (i, ring1) in rings.iter().enumerate() {
                for ring2 in rings.iter().skip(i + 1) {
                    let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                    if cost >= min_cost {
                        continue;
                    }

                    let mut player = Entity::new(100, 0, 0);
                    let boss = Entity::new(hp, damage_points, armor_points);
                    player.equip_items(&[weapon, armor, ring1, ring2]);
                    if fight(player, boss) {
                        if cost < min_cost {
                            min_cost = cost;
                        }
                    }
                }
            }
        }
    }

    println!("Minimum cost to win: {}", min_cost);

    Ok(())
}

pub fn solve_day21_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day21.txt")?;
    let mut lines = input.lines();
    let hp = lines.next().unwrap().split(": ").nth(1).unwrap().parse::<u32>().unwrap();
    let damage_points = lines.next().unwrap().split(": ").nth(1).unwrap().parse::<u32>().unwrap();
    let armor_points = lines.next().unwrap().split(": ").nth(1).unwrap().parse::<u32>().unwrap();

    let weapons = vec![
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0),
    ];

    let armors = vec![
        Item::empty(),
        Item::new(13, 0, 1),
        Item::new(31, 0, 2),
        Item::new(53, 0, 3),
        Item::new(75, 0, 4),
        Item::new(102, 0, 5),
    ];

    let rings = vec![
        Item::empty(),
        Item::new(25, 1, 0),
        Item::new(50, 2, 0),
        Item::new(100, 3, 0),
        Item::new(20, 0, 1),
        Item::new(40, 0, 2),
        Item::new(80, 0, 3),
    ];

    let mut max_cost = 0;
    for weapon in &weapons {
        for armor in &armors {
            for (i, ring1) in rings.iter().enumerate() {
                for ring2 in rings.iter().skip(i + 1) {
                    let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                    if cost <= max_cost {
                        continue;
                    }

                    let mut player = Entity::new(100, 0, 0);
                    let boss = Entity::new(hp, damage_points, armor_points);
                    player.equip_items(&[weapon, armor, ring1, ring2]);
                    if !fight(player, boss) {
                        if cost > max_cost {
                            max_cost = cost;
                        }
                    }
                }
            }
        }
    }

    println!("Minimum cost to win: {}", max_cost);

    Ok(())
}

fn fight(mut player: Entity, mut boss: Entity) -> bool {
    loop {
        player.attack(&mut boss);
        if !boss.is_alive() {
            return true;
        }

        boss.attack(&mut player);
        if !player.is_alive() {
            return false;
        }
    }
}

#[test]
fn test_day21_part1() {
    let result = solve_day21_puzzle_part1();
    assert!(result.is_ok());
}

#[test]
fn test_day21_part2() {
    let result = solve_day21_puzzle_part2();
    assert!(result.is_ok());
}