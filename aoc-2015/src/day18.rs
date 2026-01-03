use common::error::PuzzleError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LightState {
    Off,
    On,
}

pub fn solve_day18_puzzle_part1() -> Result<(), PuzzleError> {
    const NUM_STEPS: usize = 100;
    const GRID_SIZE: usize = 100;

    let input = std::fs::read_to_string("inputs/day18.txt")?;
    let lines = input.lines();
    let mut current_grid = [[LightState::Off; GRID_SIZE]; GRID_SIZE];
    let mut new_grid = [[LightState::Off; GRID_SIZE]; GRID_SIZE];
    for (r, line) in lines.enumerate() {
        for (c, ch) in line.chars().enumerate() {
            current_grid[r][c] = match ch {
                '#' => LightState::On,
                '.' => LightState::Off,
                _ => return Err(PuzzleError::Custom("Invalid character in input")),
            };
        }
    }

    for _ in 0..NUM_STEPS {
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                let mut on_neighbors = 0;
                if r > 0 {
                    if c > 0 && current_grid[r - 1][c - 1] == LightState::On {
                        on_neighbors += 1;
                    }
                    if current_grid[r - 1][c] == LightState::On {
                        on_neighbors += 1;
                    }
                    if c < GRID_SIZE - 1 && current_grid[r - 1][c + 1] == LightState::On {
                        on_neighbors += 1;
                    }
                }

                if c > 0 && current_grid[r][c - 1] == LightState::On {
                    on_neighbors += 1;
                }

                if c < GRID_SIZE - 1 && current_grid[r][c + 1] == LightState::On {
                    on_neighbors += 1;
                }

                if r < GRID_SIZE - 1 {
                    if c > 0 && current_grid[r + 1][c - 1] == LightState::On {
                        on_neighbors += 1;
                    }
                    if current_grid[r + 1][c] == LightState::On {
                        on_neighbors += 1;
                    }
                    if c < GRID_SIZE - 1 && current_grid[r + 1][c + 1] == LightState::On {
                        on_neighbors += 1;
                    }
                }
                match current_grid[r][c] {
                    LightState::Off => {
                        if on_neighbors == 3 {
                            new_grid[r][c] = LightState::On;
                        }
                        else {
                            new_grid[r][c] = LightState::Off;
                        }
                    },
                    LightState::On => {
                        if on_neighbors == 2 || on_neighbors == 3 {
                            new_grid[r][c] = LightState::On;
                        }
                        else {
                            new_grid[r][c] = LightState::Off;
                        }
                    },
                }
            }
        }

        std::mem::swap(&mut current_grid, &mut new_grid);
    }

    let mut on_count = 0;
    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            if current_grid[r][c] == LightState::On {
                on_count += 1;
            }
        }
    }

    println!("Number of lights on: {}", on_count);

    Ok(())
}

pub fn solve_day18_puzzle_part2() -> Result<(), PuzzleError> {
    const NUM_STEPS: usize = 100;
    const GRID_SIZE: usize = 100;

    let input = std::fs::read_to_string("inputs/day18.txt")?;
    let lines = input.lines();
    let mut current_grid = [[LightState::Off; GRID_SIZE]; GRID_SIZE];
    let mut new_grid = [[LightState::Off; GRID_SIZE]; GRID_SIZE];
    for (r, line) in lines.enumerate() {
        for (c, ch) in line.chars().enumerate() {
            current_grid[r][c] = match ch {
                '#' => LightState::On,
                '.' => LightState::Off,
                _ => return Err(PuzzleError::Custom("Invalid character in input")),
            };
        }
    }

    // Ensure corners are always on
    current_grid[0][0] = LightState::On;
    current_grid[0][GRID_SIZE - 1] = LightState::On;
    current_grid[GRID_SIZE - 1][0] = LightState::On;
    current_grid[GRID_SIZE - 1][GRID_SIZE - 1] = LightState::On;

    for _ in 0..NUM_STEPS {
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                let mut on_neighbors = 0;
                if r > 0 {
                    if c > 0 && current_grid[r - 1][c - 1] == LightState::On {
                        on_neighbors += 1;
                    }
                    if current_grid[r - 1][c] == LightState::On {
                        on_neighbors += 1;
                    }
                    if c < GRID_SIZE - 1 && current_grid[r - 1][c + 1] == LightState::On {
                        on_neighbors += 1;
                    }
                }

                if c > 0 && current_grid[r][c - 1] == LightState::On {
                    on_neighbors += 1;
                }

                if c < GRID_SIZE - 1 && current_grid[r][c + 1] == LightState::On {
                    on_neighbors += 1;
                }

                if r < GRID_SIZE - 1 {
                    if c > 0 && current_grid[r + 1][c - 1] == LightState::On {
                        on_neighbors += 1;
                    }
                    if current_grid[r + 1][c] == LightState::On {
                        on_neighbors += 1;
                    }
                    if c < GRID_SIZE - 1 && current_grid[r + 1][c + 1] == LightState::On {
                        on_neighbors += 1;
                    }
                }
                match current_grid[r][c] {
                    LightState::Off => {
                        if on_neighbors == 3 {
                            new_grid[r][c] = LightState::On;
                        }
                        else {
                            new_grid[r][c] = LightState::Off;
                        }
                    },
                    LightState::On => {
                        if on_neighbors == 2 || on_neighbors == 3 {
                            new_grid[r][c] = LightState::On;
                        }
                        else {
                            new_grid[r][c] = LightState::Off;
                        }
                    },
                }
            }
        }

        // Ensure corners are always on
        new_grid[0][0] = LightState::On;
        new_grid[0][GRID_SIZE - 1] = LightState::On;
        new_grid[GRID_SIZE - 1][0] = LightState::On;
        new_grid[GRID_SIZE - 1][GRID_SIZE - 1] = LightState::On;

        std::mem::swap(&mut current_grid, &mut new_grid);
    }

    let mut on_count = 0;
    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            if current_grid[r][c] == LightState::On {
                on_count += 1;
            }
        }
    }

    println!("Number of lights on: {}", on_count);

    Ok(())
}

#[test]
fn test_day18_part1() {
    let result = solve_day18_puzzle_part1();
    assert!(result.is_ok());
}

#[test]
fn test_day18_part2() {
    let result = solve_day18_puzzle_part2();
    assert!(result.is_ok());
}