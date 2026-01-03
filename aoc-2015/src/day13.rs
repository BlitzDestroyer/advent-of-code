use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn solve_day13_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day13.txt")?;
    let lines = input.lines().collect::<Vec<&str>>();
    let mut people = HashSet::new();
    let mut happiness_map = HashMap::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let person1 = parts[0];
        let gain_loss = parts[2];
        let happiness_value: i32 = parts[3].parse().unwrap();
        let person2 = parts[10].trim_end_matches('.');
        people.insert(person1);
        people.insert(person2);
        let value = if gain_loss == "gain" {
            happiness_value
        } else {
            -happiness_value
        };

        happiness_map.insert((person1, person2), value);
    }

    let people_vec = people.into_iter().collect::<Vec<&str>>();
    let mut max_happiness = i32::MIN;
    for perm in &mut people_vec.iter().permutations(people_vec.len()) {
        let mut total_happiness = 0;
        for i in 0..perm.len() {
            let person1 = perm[i];
            let person2 = perm[(i + 1) % perm.len()];
            total_happiness += happiness_map.get(&(person1, person2)).unwrap_or(&0);
            total_happiness += happiness_map.get(&(person2, person1)).unwrap_or(&0);
        }

        if total_happiness > max_happiness {
            max_happiness = total_happiness;
        }
    }

    println!("Maximum happiness: {}", max_happiness);
    
    Ok(())
}

pub fn solve_day13_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day13.txt")?;
    let lines = input.lines().collect::<Vec<&str>>();
    let mut people = HashSet::new();
    people.insert("Me");
    let mut happiness_map = HashMap::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let person1 = parts[0];
        let gain_loss = parts[2];
        let happiness_value: i32 = parts[3].parse().unwrap();
        let person2 = parts[10].trim_end_matches('.');
        people.insert(person1);
        people.insert(person2);
        let value = if gain_loss == "gain" {
            happiness_value
        } else {
            -happiness_value
        };

        happiness_map.insert((person1, person2), value);
    }

    let people_vec = people.into_iter().collect::<Vec<&str>>();
    let mut max_happiness = i32::MIN;
    for perm in &mut people_vec.iter().permutations(people_vec.len()) {
        let mut total_happiness = 0;
        for i in 0..perm.len() {
            let person1 = perm[i];
            let person2 = perm[(i + 1) % perm.len()];
            total_happiness += happiness_map.get(&(person1, person2)).unwrap_or(&0);
            total_happiness += happiness_map.get(&(person2, person1)).unwrap_or(&0);
        }

        if total_happiness > max_happiness {
            max_happiness = total_happiness;
        }
    }

    println!("Maximum happiness: {}", max_happiness);
    
    Ok(())
}

#[test]
fn test_solve_day13_puzzle_part1() {
    assert!(solve_day13_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day13_puzzle_part2() {
    assert!(solve_day13_puzzle_part2().is_ok());
}