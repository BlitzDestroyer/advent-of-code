use std::collections::HashSet;

use once_cell::sync::Lazy;
use thiserror::Error;

static TRIGRAM_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let trigrams = [
        "abc", "bcd", "cde", "def", "efg", "fgh", "ghi", "hij", "ijk", "jkl", "klm", "lmn", "mno", "nop", "opq", "pqr", "qrs", "rst", "stu", "tuv", "uvw", "vwx", "wxy", "xyz"
    ];

    trigrams.iter().cloned().collect()
});

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn solve_day11_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day11.txt")?;
    let mut new_password = increment_password(input.trim());
    while !is_valid_password(&new_password) {
        new_password = increment_password(&new_password);
    }

    println!("Next valid password: {}", new_password);

    Ok(())
}

pub fn solve_day11_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day11.txt")?;
    let mut new_password = increment_password(input.trim());
    while !is_valid_password(&new_password) {
        new_password = increment_password(&new_password);
    }

    new_password = increment_password(&new_password);
    while !is_valid_password(&new_password) {
        new_password = increment_password(&new_password);
    }

    println!("Next valid password: {}", new_password);

    Ok(())
}

fn increment_password(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().collect();
    let mut index = chars.len() - 1;
    loop {
        chars[index] = get_next_letter(chars[index]);
        if chars[index] == 'a' {
            if index == 0 {
                break;
            }
            index -= 1;
        }
        else {
            break;
        }
    }

    chars.iter().collect()
}

fn get_next_letter(c: char) -> char {
    match c {
        'z' => 'a',
        _ => ((c as u8) + 1) as char,
    }
}

fn is_valid_password(password: &str) -> bool {
    let chars = password.chars();
    let mut has_increasing_straight = false;
    let mut last_char = '\0';
    let mut pairs_found = HashSet::new();
    for (i, c) in chars.enumerate() {
        match c {
            'i' | 'o' | 'l' => return false,
            _ => {}
        }

        if i < password.len() - 2 {
            let trigram = &password[i..i + 3];
            if TRIGRAM_SET.contains(trigram) {
                has_increasing_straight = true;
            }
        }

        if c == last_char {
            pairs_found.insert(c);
        }

        last_char = c;
    }

    has_increasing_straight && pairs_found.len() >= 2
}

#[test]
fn test_solve_day11_puzzle_part1() {
    assert!(solve_day11_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day11_puzzle_part2() {
    assert!(solve_day11_puzzle_part2().is_ok());
}