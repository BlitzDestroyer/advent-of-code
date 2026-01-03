use std::collections::HashMap;

use common::error::PuzzleError;

pub fn solve_day17_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day17.txt")?;
    let mut containers: Vec<u32> = input
        .lines()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect();

    containers.sort_unstable();
    containers.reverse();

    let target_volume = 150;
    let combinations = solve_day17_part1_helper(&containers, target_volume);
    println!("Number of combinations: {}", combinations);

    Ok(())
}

fn solve_day17_part1_helper(containers: &[u32], target_volume: i32) -> usize {
    if target_volume == 0 {
        return 1;
    }
    else {
        if target_volume < 0 || containers.is_empty() {
            return 0;
        }
        else {
            let head = containers[0] as i32;

            let with_current = solve_day17_part1_helper(&containers[1..], target_volume - head);
            let without_current = solve_day17_part1_helper(&containers[1..], target_volume);

            return with_current + without_current;
        }
    }
}

pub fn solve_day17_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day17.txt")?;
    let mut containers: Vec<u32> = input
        .lines()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect();

    containers.sort_unstable();
    containers.reverse();

    let target_volume = 150;
    let mut container_tracker = HashMap::new();
    solve_day17_part2_helper(&containers, target_volume,0, &mut container_tracker);
    let min = container_tracker.keys().min().unwrap();
    println!("Number of combinations: {}", container_tracker.get(min).unwrap());

    Ok(())
}

fn solve_day17_part2_helper(containers: &[u32], target_volume: i32, num_containers: u32, container_tracker: &mut HashMap<u32, usize>) {
    if target_volume == 0 {
        if let Some(count) = container_tracker.get(&num_containers) {
            container_tracker.insert(num_containers, count + 1);
        }
        else {
            container_tracker.insert(num_containers, 1);
        }
    }
    else {
        if target_volume < 0 || containers.is_empty() {
            return;
        }
        else {
            let head = containers[0] as i32;

            solve_day17_part2_helper(&containers[1..], target_volume - head, num_containers + 1, container_tracker);
            solve_day17_part2_helper(&containers[1..], target_volume, num_containers, container_tracker);
        }
    }
}

#[test]
fn test_solve_day17_puzzle_part1() {
    assert!(solve_day17_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day17_puzzle_part2() {
    assert!(solve_day17_puzzle_part2().is_ok());
}