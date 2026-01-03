use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
struct Ingrediant {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingrediant {
    fn new(capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self {
        Self {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

pub fn solve_day15_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day15.txt")?;
    let lines = input.lines();
    let mut ingrediants = Vec::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let capacity = parts[2].trim_end_matches(',').parse::<i32>().unwrap();
        let durability = parts[4].trim_end_matches(',').parse::<i32>().unwrap();
        let flavor = parts[6].trim_end_matches(',').parse::<i32>().unwrap();
        let texture = parts[8].trim_end_matches(',').parse::<i32>().unwrap();
        let calories = parts[10].parse::<i32>().unwrap();
        ingrediants.push(Ingrediant::new(capacity, durability, flavor, texture, calories));
    }

    let num_ingredients = ingrediants.len();
    let mut amounts = vec![0; num_ingredients];
    let mut max_score = 0;
    solve_day15_helper(&ingrediants, &mut amounts, 0, 100, &mut max_score, None);

    println!("Max score: {}", max_score);

    Ok(())
}

pub fn solve_day15_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day15.txt")?;
    let lines = input.lines();
    let mut ingrediants = Vec::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let capacity = parts[2].trim_end_matches(',').parse::<i32>().unwrap();
        let durability = parts[4].trim_end_matches(',').parse::<i32>().unwrap();
        let flavor = parts[6].trim_end_matches(',').parse::<i32>().unwrap();
        let texture = parts[8].trim_end_matches(',').parse::<i32>().unwrap();
        let calories = parts[10].parse::<i32>().unwrap();
        ingrediants.push(Ingrediant::new(capacity, durability, flavor, texture, calories));
    }

    let num_ingredients = ingrediants.len();
    let mut amounts = vec![0; num_ingredients];
    let mut max_score = 0;
    solve_day15_helper(&ingrediants, &mut amounts, 0, 100, &mut max_score, Some(500));

    println!("Max score: {}", max_score);

    Ok(())
}

fn solve_day15_helper(
    ingrediants: &Vec<Ingrediant>,
    amounts: &mut Vec<i32>,
    index: usize,
    remaining: i32,
    max_score: &mut i32,
    calorie_target: Option<i32>,
) {
    if index == ingrediants.len() - 1 {
        amounts[index] = remaining;
        let score = calculate_score(ingrediants, amounts, calorie_target);
        if score > *max_score {
            *max_score = score;
        }
        return;
    }

    for amount in 0..=remaining {
        amounts[index] = amount;
        solve_day15_helper(ingrediants, amounts, index + 1, remaining - amount, max_score, calorie_target);
    }
}

fn calculate_score(ingrediants: &Vec<Ingrediant>, amounts: &Vec<i32>, calorie_target: Option<i32>) -> i32 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;

    for (i, ingrediant) in ingrediants.iter().enumerate() {
        capacity += ingrediant.capacity * amounts[i];
        durability += ingrediant.durability * amounts[i];
        flavor += ingrediant.flavor * amounts[i];
        texture += ingrediant.texture * amounts[i];
        calories += ingrediant.calories * amounts[i];
    }

    capacity = capacity.max(0);
    durability = durability.max(0);
    flavor = flavor.max(0);
    texture = texture.max(0);
    calories = if let Some(target) = calorie_target {
        if calories == target {
            1
        }
        else {
            0
        }
    }
    else {
        1
    };

    capacity * durability * flavor * texture * calories
}

#[test]
fn test_day15_part1() {
    assert!(solve_day15_puzzle_part1().is_ok());
}

#[test]
fn test_day15_part2() {
    assert!(solve_day15_puzzle_part2().is_ok());
}