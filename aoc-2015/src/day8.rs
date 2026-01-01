use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn solve_day8_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day8.txt")?;
    let lines = input.lines();
    let mut string_char_diff = 0;
    for line in lines {
        let code_chars = line.len();
        let mut memory_chars = 0;
        let mut chars = line.chars().skip(1).take(line.len() - 2).peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.peek() {
                    Some('"') | Some('\\') => {
                        memory_chars += 1;
                        chars.next();
                    }
                    Some('x') => {
                        memory_chars += 1;
                        chars.next();
                        chars.next();
                        chars.next();
                    }
                    _ => {
                        memory_chars += 1;
                    }
                }
            } 
            else {
                memory_chars += 1;
            }
        }

        string_char_diff += code_chars - memory_chars;
    }

    println!("String char difference: {}", string_char_diff);

    Ok(())
}

pub fn solve_day8_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day8.txt")?;
    let lines = input.lines();
    let mut string_char_diff = 0;
    for line in lines {
        let code_chars = line.len();
        let mut encoded_chars = 0;
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                '"' | '\\' => {
                    encoded_chars += 2;
                }
                _ => {
                    encoded_chars += 1;
                }
            }
        }

        println!("Line: {}, Encoded chars: {}, code chars: {}", line, encoded_chars + 2, code_chars);
        string_char_diff += 2 + encoded_chars - code_chars;
    }

    println!("String char difference: {}", string_char_diff);

    Ok(())
}

#[test]
fn test_solve_day8_puzzle_part1() {
    assert!(solve_day8_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day8_puzzle_part2() {
    assert!(solve_day8_puzzle_part2().is_ok());
}