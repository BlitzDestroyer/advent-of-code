use common::error::PuzzleError;

pub fn solve_day20_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day20.txt")?;
    let target_presents: usize = input.trim().parse().unwrap();
    let mut i = 0;
    loop {
        i += 1;
        let factors = get_factors(i);
        let total_presents: usize = factors.iter().sum::<usize>() * 10;
        if total_presents >= target_presents {
            break;
        }
    }

    println!("Lowest house number with at least {} presents: {}", target_presents, i);
    
    Ok(())
}

pub fn solve_day20_puzzle_part1_v2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day20.txt")?;
    let target: usize = input.trim().parse().unwrap();
    
    // Upper bound heuristic; usually target / 10 is enough
    let limit = target / 10;
    let mut houses = vec![0usize; limit + 1];

    for elf in 1..=limit {
        let presents = elf * 10;
        let mut house = elf;

        while house <= limit {
            houses[house] += presents;
            house += elf;
        }
    }

    let answer = houses
        .iter()
        .enumerate()
        .skip(1)
        .find(|&(_, &p)| p >= target)
        .map(|(i, _)| i)
        .unwrap();

    println!("Lowest house number with at least {} presents: {}", target, answer);
    Ok(())
}

pub fn solve_day20_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day20.txt")?;
    let target_presents: usize = input.trim().parse().unwrap();
    let mut i = 0;
    loop {
        i += 1;
        let factors = get_factors(i);
        let total_presents: usize = factors.iter().filter(|v| *v * 50 <= i).sum::<usize>() * 11;
        if total_presents >= target_presents {
            break;
        }
    }

    println!("Lowest house number with at least {} presents: {}", target_presents, i);
    
    Ok(())
}

pub fn solve_day20_puzzle_part2_v2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day20.txt")?;
    let target: usize = input.trim().parse().unwrap();

    let limit = target;
    let mut houses = vec![0usize; limit + 1];

    for elf in 1..=limit {
        let presents = elf * 11;
        let mut house = elf;

        for _ in 0..50 {
            if house > limit {
                break;
            }
            
            houses[house] += presents;
            house += elf;
        }
    }

    let answer = houses
        .iter()
        .enumerate()
        .skip(1)
        .find(|&(_, &p)| p >= target)
        .map(|(i, _)| i)
        .unwrap();

    println!("Lowest house number with at least {} presents: {}", target, answer);
    
    Ok(())
}

fn get_factors(n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    for i in 1..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            factors.push(i);
            if i != n / i {
                factors.push(n / i);
            }
        }
    }

    factors
}

#[test]
fn test_day20_part1() {
    let result = solve_day20_puzzle_part1();
    assert!(result.is_ok());
}

#[test]
fn test_day20_part1_v2() {
    let result = solve_day20_puzzle_part1_v2();
    assert!(result.is_ok());
}

#[test]
fn test_day20_part2() {
    let result = solve_day20_puzzle_part2();
    assert!(result.is_ok());
}

#[test]
fn test_day20_part2_v2() {
    let result = solve_day20_puzzle_part2_v2();
    assert!(result.is_ok());
}