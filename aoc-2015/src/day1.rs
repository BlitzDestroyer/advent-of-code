use common::error::PuzzleError;

pub fn solve_day1_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day1.txt")?;
    let mut floor = 0;
    let chars = input.chars();
    for c in chars {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }

    println!("Final floor: {}", floor);
    
    Ok(())
}

pub fn solve_day1_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day1.txt")?;
    let mut floor = 0;
    let chars = input.chars();
    for (i, c) in chars.enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor == -1 {
            println!("Position entering basement: {}", i + 1);
            break;
        }
    }

    println!("Final floor: {}", floor);
    
    Ok(())
}

#[test]
fn test_day1_puzzle_part1() {
    assert!(solve_day1_puzzle_part1().is_ok());
}

#[test]
fn test_day1_puzzle_part2() {
    assert!(solve_day1_puzzle_part2().is_ok());
}