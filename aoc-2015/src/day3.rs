use common::error::PuzzleError;

pub fn solve_day3_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day3.txt")?;
    let mut x = 0;
    let mut y = 0;
    let mut visited_houses = std::collections::HashSet::new();
    visited_houses.insert((x, y));

    for ch in input.chars() {
        match ch {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        }
        visited_houses.insert((x, y));
    }

    println!("Houses visited: {}", visited_houses.len());

    Ok(())
}

pub fn solve_day3_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day3.txt")?;
    let mut x_reg = 0;
    let mut y_reg = 0;
    let mut x_bot = 0;
    let mut y_bot = 0;
    let mut visited_houses = std::collections::HashSet::new();
    visited_houses.insert((x_reg, y_reg));

    for (i, ch) in input.chars().enumerate() {
        match ch {
            '^' => {
                if i % 2 == 0 {
                    y_reg += 1
                }
                else {
                    y_bot += 1
                }
            },
            'v' => {
                if i % 2 == 0 {
                    y_reg -= 1
                }
                else {
                    y_bot -= 1
                }
            },
            '>' => {
                if i % 2 == 0 {
                    x_reg += 1
                }
                else {
                    x_bot += 1
                }
            },
            '<' => {
                if i % 2 == 0 {
                    x_reg -= 1
                }
                else {
                    x_bot -= 1
                }
            },
            _ => (),
        }
        
        if i % 2 == 0 {
            visited_houses.insert((x_reg, y_reg));
        } 
        else {
            visited_houses.insert((x_bot, y_bot));
        }
    }

    println!("Houses visited: {}", visited_houses.len());

    Ok(())
}

#[test]
fn test_solve_day3_puzzle_part1() {
    solve_day3_puzzle_part1().unwrap();
}

#[test]
fn test_solve_day3_puzzle_part2() {
    solve_day3_puzzle_part2().unwrap();
}