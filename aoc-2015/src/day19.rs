use std::collections::{HashMap, HashSet};

use common::error::PuzzleError;

pub fn solve_day19_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day19.txt")?;
    let mut lines = input.lines();
    let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split(" => ").collect();
        let from = parts[0];
        let to = parts[1];
        if let Some(entry) = replacements.get_mut(from) {
            entry.push(to);
        }
        else {
            replacements.insert(from, vec![to]);
        }
    }

    let base_molecule = lines.next().unwrap();
    let mut distinct_molecules = HashSet::new();
    for (i, _) in base_molecule.char_indices() {
        let single = &base_molecule[i..i + 1];
        if let Some(repls) = replacements.get(single) {
            for repl in repls {
                let new_molecule = format!("{}{}{}", &base_molecule[0..i], repl, &base_molecule[i + 1..]);
                distinct_molecules.insert(new_molecule);
            }
        }

        let double = if i + 2 <= base_molecule.len() {
            &base_molecule[i..i + 2]
        }
        else {
            continue;
        };

        if let Some(repls) = replacements.get(double) {
            for repl in repls {
                let new_molecule = format!("{}{}{}", &base_molecule[0..i], repl, &base_molecule[i + 2..]);
                distinct_molecules.insert(new_molecule);
            }
        }
    }

    println!("Number of distinct molecules: {}", distinct_molecules.len());

    Ok(())
}

pub fn solve_day19_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day19.txt")?;
    let mut lines = input.lines();
    let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split(" => ").collect();
        let from = parts[0];
        let to = parts[1];
        if let Some(entry) = replacements.get_mut(from) {
            entry.push(to);
        }
        else {
            replacements.insert(from, vec![to]);
        }
    }

    let reverse_replacements: HashMap<&str, Vec<&str>> = replacements.iter()
        .flat_map(|(from, tos)| tos.iter().map(move |to| (*to, *from)))
        .fold(HashMap::new(), |mut acc, (to, from)| {
            acc.entry(to).or_insert_with(Vec::new).push(from);
            acc
        });
    let target_molecule = lines.next().unwrap();
    let starting_molecule = "e";
    let mut min_steps: Option<usize> = None;
    solve_day19_part2_helper(target_molecule, starting_molecule, &reverse_replacements, 0, &mut min_steps);

    println!("Minimum number of steps: {}", min_steps.unwrap());

    Ok(())
}

fn solve_day19_part2_helper(current_molecule: &str, target_molecule: &str, replacements: &HashMap<&str, Vec<&str>>, steps: usize, min_steps: &mut Option<usize>) {
    if let Some(min) = min_steps {
        if steps >= *min {
            return;
        }
    }

    // if target_molecule.len() < current_molecule.len() {
    //     return;
    // }

    if current_molecule == target_molecule {
        if let Some(min) = min_steps {
            if steps < *min {
                *min_steps = Some(steps);
            }
        }
        else {
            *min_steps = Some(steps);
        }
        return;
    }

    for (from, tos) in replacements {
        //println!("Current molecule: {}, looking for: {}, replacements: {:?}", current_molecule, from, tos);
        for to in tos {
            //println!("Trying to replace: {} with: {}", to, from);
            let mut start_index = 0;
            //println!("Haystack: {:?}, {:?}", search, search.find(from));
            while let Some(index) = current_molecule[start_index..].find(from) {
                //println!("At molecule: {}, steps: {}", current_molecule, steps);
                let absolute_index = start_index + index;
                let end_bound = absolute_index + from.len();
                let start_str = &current_molecule[0..absolute_index];
                let end_str = if end_bound < current_molecule.len() {
                    &current_molecule[end_bound..]
                }
                else {
                    ""
                };

                let new_molecule = format!("{}{}{}", start_str, to, end_str);
                
                solve_day19_part2_helper(&new_molecule, target_molecule, replacements, steps + 1, min_steps);
                start_index = absolute_index + 1;
            }
        }
    }
}

pub fn solve_day19_puzzle_part2_v2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day19.txt")?;
    let mut lines = input.lines();
    let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split(" => ").collect();
        let from = parts[0];
        let to = parts[1];
        if let Some(entry) = replacements.get_mut(from) {
            entry.push(to);
        }
        else {
            replacements.insert(from, vec![to]);
        }
    }

    let target_molecule = lines.next().unwrap();
    let rn_count = target_molecule.matches("Rn").count();
    let ar_count = target_molecule.matches("Ar").count();
    let y_count = target_molecule.matches("Y").count();
    let element_count = target_molecule
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .filter(|window| window[0].is_uppercase())
        .count() + if target_molecule.chars().last().unwrap().is_uppercase() { 1 } else { 0 };
    let min_steps = element_count - rn_count - ar_count - (2 * y_count) - 1;

    println!("Minimum number of steps: {}", min_steps);

    Ok(())
}

#[test]
fn test_day19_part1() {
    let result = solve_day19_puzzle_part1();
    assert!(result.is_ok());
}

#[test]
fn test_day19_part2() {
    let result = solve_day19_puzzle_part2();
    assert!(result.is_ok());
}

#[test]
fn test_day19_part2_v2() {
    let result = solve_day19_puzzle_part2_v2();
    assert!(result.is_ok());
}