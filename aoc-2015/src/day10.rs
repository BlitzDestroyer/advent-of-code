use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn solve_day10_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day10.txt")?;
    let mut sequence = input.trim().to_string();
    for _ in 0..40 {
        let mut next_sequence = String::new();
        let mut chars = sequence.chars().peekable();
        while let Some(c) = chars.next() {
            let mut count = 1;
            while let Some(&next_c) = chars.peek() {
                if next_c == c {
                    count += 1;
                    chars.next();
                }
                else {
                    break;
                }
            }
            next_sequence.push_str(&format!("{}{}", count, c));
        }
        sequence = next_sequence;
    }

    println!("Length after 40 iterations: {}", sequence.len());

    Ok(())
}

pub fn solve_day10_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day10.txt")?;
    let mut sequence = input.trim().to_string();
    for _ in 0..50 {
        let mut next_sequence = String::new();
        let mut chars = sequence.chars().peekable();
        while let Some(c) = chars.next() {
            let mut count = 1;
            while let Some(&next_c) = chars.peek() {
                if next_c == c {
                    count += 1;
                    chars.next();
                }
                else {
                    break;
                }
            }
            next_sequence.push_str(&format!("{}{}", count, c));
        }
        sequence = next_sequence;
    }

    println!("Length after 40 iterations: {}", sequence.len());

    Ok(())
}

#[test]
fn test_solve_day10_puzzle_part1() {
    assert!(solve_day10_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day10_puzzle_part2() {
    assert!(solve_day10_puzzle_part2().is_ok());
}