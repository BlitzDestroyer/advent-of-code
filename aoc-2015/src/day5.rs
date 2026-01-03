use std::collections::HashMap;

use common::error::PuzzleError;

pub fn solve_day5_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day5.txt")?;
    
    let mut nice_count = 0;
    'outer_loop:
    for line in input.lines() {
        let mut vowel_count = 0;
        let mut has_double = false;
        let mut last_char = '\0';
        for c in line.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
                'b' => {
                    if last_char == 'a' {
                        continue 'outer_loop;
                    }
                },
                'd' => {
                    if last_char == 'c' {
                        continue 'outer_loop;
                    }
                },
                'q' => {
                    if last_char == 'p' {
                        continue 'outer_loop;
                    }
                },
                'y' => {
                    if last_char == 'x' {
                        continue 'outer_loop;
                    }
                },
                _ => (),
            }
            if c == last_char {
                has_double = true;
            }
            last_char = c;
        }

        if vowel_count >= 3 && has_double {
            nice_count += 1;
        }
    }

    println!("Nice strings: {}", nice_count);
    Ok(())
}

pub fn solve_day5_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day5.txt")?;
    
    let mut nice_count = 0;
    for line in input.lines() {
        let mut repeat_pair_map = HashMap::new();
        let mut last_char = '\0';
        let mut second_last_char = '\0';
        let mut found_repeat_pair = false;
        let mut found_repeat_with_one_between = false;
        for (i, c) in line.chars().enumerate() {
            if let Some(&first_index) = repeat_pair_map.get(&(last_char, c)) {
                if i - first_index >= 3 {
                    //println!("Found repeat pair: {}{} at indices {} and {}", last_char, c, first_index, i);
                    found_repeat_pair = true;
                }
            }
            else if i > 0 {
                repeat_pair_map.insert((last_char, c), i - 1);
            }
            
            if second_last_char == c {
                found_repeat_with_one_between = true;
            }

            second_last_char = last_char;
            last_char = c;
        }

        if found_repeat_pair && found_repeat_with_one_between {
            nice_count += 1;
        }
    }

    println!("Nice strings: {}", nice_count);
    Ok(())
}

#[test]
pub fn test_solve_day5_puzzle_part1() {
    assert!(solve_day5_puzzle_part1().is_ok())
}

#[test]
pub fn test_solve_day5_puzzle_part2() {
    assert!(solve_day5_puzzle_part2().is_ok())
}