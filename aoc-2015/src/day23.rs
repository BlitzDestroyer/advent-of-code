use common::error::PuzzleError;

pub fn solve_day23_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day23.txt")?;
    let instructions = input.lines().collect::<Vec<&str>>();
    let mut a = 0u32;
    let mut b = 0u32;
    let mut pc = 0usize;
    while pc < instructions.len() {
        let instr = instructions[pc];
        let parts = instr.split_whitespace().collect::<Vec<&str>>();
        match parts[0] {
            "hlf" => {
                let reg = parts[1];
                match reg {
                    "a" => a /= 2,
                    "b" => b /= 2,
                    _ => panic!("Unsupported register: {}", reg),
                }
            },
            "tpl" => {
                let reg = parts[1];
                match reg {
                    "a" => a *= 3,
                    "b" => b *= 3,
                    _ => panic!("Unsupported register: {}", reg),
                }
            },
            "inc" => {
                let reg = parts[1];
                match reg {
                    "a" => a += 1,
                    "b" => b += 1,
                    _ => panic!("Unsupported register: {}", reg),
                }
            },
            "jmp" => {
                let offset: isize = parts[1].parse().unwrap();
                if offset < 0 {
                    pc -= (-offset) as usize;
                }
                else {
                    pc += offset as usize;
                }

                continue;
            },
            "jie" => {
                let reg = parts[1].trim_end_matches(',');
                let offset: isize = parts[2].parse().unwrap();
                let value = match reg {
                    "a" => a,
                    "b" => b,
                    _ => panic!("Unsupported register: {}", reg),
                };
                if value % 2 == 0 {
                    if offset < 0 {
                        pc -= (-offset) as usize;
                    }
                    else {
                        pc += offset as usize;
                    }
                    continue;
                }
            },
            "jio" => {
                let reg = parts[1].trim_end_matches(',');
                let offset: isize = parts[2].parse().unwrap();
                let value = match reg {
                    "a" => a,
                    "b" => b,
                    _ => panic!("Unsupported register: {}", reg),
                };

                if value == 1 {
                    if offset < 0 {
                        pc -= (-offset) as usize;
                    }
                    else {
                        pc += offset as usize;
                    }

                    continue;
                }
            },
            _ => panic!("Unsupported instruction: {}", parts[0]),
        }

        pc += 1;
    }

    println!("B: {}", b);

    Ok(())
}

pub fn solve_day23_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day23.txt")?;
    let instructions = input.lines().collect::<Vec<&str>>();
    let mut a = 1u32;
    let mut b = 0u32;
    let mut pc = 0usize;
    while pc < instructions.len() {
        let instr = instructions[pc];
        let parts = instr.split_whitespace().collect::<Vec<&str>>();
        match parts[0] {
            "hlf" => {
                let reg = parts[1];
                match reg {
                    "a" => a /= 2,
                    "b" => b /= 2,
                    _ => panic!("Unsupported register: {}", reg),
                }
            },
            "tpl" => {
                let reg = parts[1];
                match reg {
                    "a" => a *= 3,
                    "b" => b *= 3,
                    _ => panic!("Unsupported register: {}", reg),
                }
            },
            "inc" => {
                let reg = parts[1];
                match reg {
                    "a" => a += 1,
                    "b" => b += 1,
                    _ => panic!("Unsupported register: {}", reg),
                }
            },
            "jmp" => {
                let offset: isize = parts[1].parse().unwrap();
                if offset < 0 {
                    pc -= (-offset) as usize;
                }
                else {
                    pc += offset as usize;
                }

                continue;
            },
            "jie" => {
                let reg = parts[1].trim_end_matches(',');
                let offset: isize = parts[2].parse().unwrap();
                let value = match reg {
                    "a" => a,
                    "b" => b,
                    _ => panic!("Unsupported register: {}", reg),
                };
                if value % 2 == 0 {
                    if offset < 0 {
                        pc -= (-offset) as usize;
                    }
                    else {
                        pc += offset as usize;
                    }
                    continue;
                }
            },
            "jio" => {
                let reg = parts[1].trim_end_matches(',');
                let offset: isize = parts[2].parse().unwrap();
                let value = match reg {
                    "a" => a,
                    "b" => b,
                    _ => panic!("Unsupported register: {}", reg),
                };

                if value == 1 {
                    if offset < 0 {
                        pc -= (-offset) as usize;
                    }
                    else {
                        pc += offset as usize;
                    }

                    continue;
                }
            },
            _ => panic!("Unsupported instruction: {}", parts[0]),
        }

        pc += 1;
    }

    println!("B: {}", b);

    Ok(())
}

#[test]
fn test_day23_part1() {
    solve_day23_puzzle_part1().unwrap();
}

#[test]
fn test_day23_part2() {
    solve_day23_puzzle_part2().unwrap();
}