use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
pub enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Off,
    On,
}

pub fn solve_day6_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day6.txt")?;
    let lines = input.lines();
    let mut instructions = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let instruction = if parts[0] == "turn" {
            let start = parts[2].split(',');
            let start_row: usize = start.clone().next().unwrap().parse().unwrap();
            let start_col: usize = start.skip(1).next().unwrap().parse().unwrap();
            let end = parts[4].split(',');
            let end_row: usize = end.clone().next().unwrap().parse().unwrap();
            let end_col: usize = end.skip(1).next().unwrap().parse().unwrap();
            if parts[1] == "on" {
                (Instruction::TurnOn, (start_row, start_col), (end_row, end_col))
            } 
            else {
                (Instruction::TurnOff, (start_row, start_col), (end_row, end_col))
            }
        }
        else{ // Toggle
            let start = parts[1].split(',');
            let start_row: usize = start.clone().next().unwrap().parse().unwrap();
            let start_col: usize = start.skip(1).next().unwrap().parse().unwrap();
            let end = parts[3].split(',');
            let end_row: usize = end.clone().next().unwrap().parse().unwrap();
            let end_col: usize = end.skip(1).next().unwrap().parse().unwrap();
            (Instruction::Toggle, (start_row, start_col), (end_row, end_col))
        };

        instructions.push(instruction);
    }

    let mut grid = vec![vec![State::Off; 1000]; 1000];
    for instr in instructions {
        let (instruction, (start_row, start_col), (end_row, end_col)) = instr;
        for row in start_row..=end_row {
            for col in start_col..=end_col {
                match instruction {
                    Instruction::TurnOn => grid[row][col] = State::On,
                    Instruction::TurnOff => grid[row][col] = State::Off,
                    Instruction::Toggle => {
                        grid[row][col] = match grid[row][col] {
                            State::On => State::Off,
                            State::Off => State::On,
                        }
                    }
                }
            }
        }
    }

    let mut count_on = 0;
    for row in grid {
        for light in row {
            if let State::On = light {
                count_on += 1;
            }
        }
    }

    println!("Number of lights on: {}", count_on);

    Ok(())
}

pub fn solve_day6_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day6.txt")?;
    let lines = input.lines();
    let mut instructions = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let instruction = if parts[0] == "turn" {
            let start = parts[2].split(',');
            let start_row: usize = start.clone().next().unwrap().parse().unwrap();
            let start_col: usize = start.skip(1).next().unwrap().parse().unwrap();
            let end = parts[4].split(',');
            let end_row: usize = end.clone().next().unwrap().parse().unwrap();
            let end_col: usize = end.skip(1).next().unwrap().parse().unwrap();
            if parts[1] == "on" {
                (Instruction::TurnOn, (start_row, start_col), (end_row, end_col))
            } 
            else {
                (Instruction::TurnOff, (start_row, start_col), (end_row, end_col))
            }
        }
        else{ // Toggle
            let start = parts[1].split(',');
            let start_row: usize = start.clone().next().unwrap().parse().unwrap();
            let start_col: usize = start.skip(1).next().unwrap().parse().unwrap();
            let end = parts[3].split(',');
            let end_row: usize = end.clone().next().unwrap().parse().unwrap();
            let end_col: usize = end.skip(1).next().unwrap().parse().unwrap();
            (Instruction::Toggle, (start_row, start_col), (end_row, end_col))
        };

        instructions.push(instruction);
    }

    let mut grid = vec![vec![0u64; 1000]; 1000];
    for instr in instructions {
        let (instruction, (start_row, start_col), (end_row, end_col)) = instr;
        for row in start_row..=end_row {
            for col in start_col..=end_col {
                match instruction {
                    Instruction::TurnOn => grid[row][col] += 1,
                    Instruction::TurnOff => {
                        if grid[row][col] > 0 {
                            grid[row][col] -= 1;
                        }
                    },
                    Instruction::Toggle => {
                        grid[row][col] += 2;
                    }
                }
            }
        }
    }

    let mut count_on = 0;
    for row in grid {
        for light in row {
            count_on += light;
        }
    }

    println!("Number of lights on: {}", count_on);

    Ok(())
}

#[test]
fn test_solve_day6_puzzle_part1() {
    assert!(solve_day6_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day6_puzzle_part2() {
    assert!(solve_day6_puzzle_part2().is_ok());
}