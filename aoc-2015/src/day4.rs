use common::error::PuzzleError;

pub fn solve_day4_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day4.txt")?;
    let secret_key = input.trim();

    for i in 0.. {
        let hash_input = format!("{}{}", secret_key, i);
        let digest = md5::compute(hash_input);
        if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xF0) == 0 {
            println!("Lowest number for five leading zeroes: {}", i);
            break;
        }
    }

    Ok(())
}

pub fn solve_day4_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day4.txt")?;
    let secret_key = input.trim();

    for i in 0.. {
        let hash_input = format!("{}{}", secret_key, i);
        let digest = md5::compute(hash_input);
        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            println!("Lowest number for six leading zeroes: {}", i);
            break;
        }
    }

    Ok(())
}

#[test]
pub fn test_solve_day4_puzzle_part1() -> Result<(), PuzzleError> {
    solve_day4_puzzle_part1()
}

#[test]
pub fn test_solve_day4_puzzle_part2() -> Result<(), PuzzleError> {
    solve_day4_puzzle_part2()
}