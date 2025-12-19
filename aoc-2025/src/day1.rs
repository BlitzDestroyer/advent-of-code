use thiserror::Error;

const MAX_POSITION: i32 = 99;

#[derive(Debug, Error)]
pub enum RotationPuzzleError {
    #[error("IO error occurred: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse error occurred: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Other error occurred: {0}")]
    Other(String),
}

pub fn solve_rotation_puzzle_part1() -> Result<(), RotationPuzzleError> {
    let data = std::fs::read_to_string("./inputs/day1.txt")?;
    let moves = data.lines();
    let mut position = 50i32;
    let mut key = 0;
    for m in moves {
        let direction = m.chars().next().unwrap();
        let steps = m[1..].parse::<i32>()?;
        match direction {
            'L' => {
                position -= steps;
            },
            'R' => {
                position += steps;
            },
            _ => {
                return Err(RotationPuzzleError::Other(format!("Invalid direction: {}", direction)));
            }
        }

        position %= MAX_POSITION + 1;

        if position == 0 {
            key += 1;
        }
    }
    
    println!("The final key is: {}", key);

    Ok(())
}

pub fn solve_rotation_puzzle_part2() -> Result<(), RotationPuzzleError> {
    let data = std::fs::read_to_string("./inputs/day1.txt")?;
    let moves = data.lines();
    let mut position = 50i32;
    let mut key = 0;
    println!("The dial starts by pointing at {}.", position);
    for m in moves {
        let direction = m.chars().next().unwrap();
        let steps = m[1..].parse::<i32>()?;
        for _ in 0..steps {
            match direction {
                'L' => {
                    position -= 1;
                },
                'R' => {
                    position += 1;
                },
                _ => {
                    return Err(RotationPuzzleError::Other(format!("Invalid direction: {}", direction)));
                }
            }

            if position > MAX_POSITION {
                position = 0;
                key += 1;
            }
            else if position < 0 {
                position = MAX_POSITION;
            }
            else if position == 0 {
                key += 1;
            }
        }

        println!("The dial is rotated {} to point at {}.", m, position);
    }
    
    println!("The final key is: {}", key);

    Ok(())
}

#[test]
fn test_solve_rotation_puzzle_part1() {
    let result = solve_rotation_puzzle_part1();
    assert!(result.is_ok());
}

#[test]
fn test_solve_rotation_puzzle_part2() {
    let result = solve_rotation_puzzle_part2();
    assert!(result.is_ok());
}