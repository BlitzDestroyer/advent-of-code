use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn solve_day2_puzzle_part1() -> Result<(), PuzzleError> {
    let ids = std::fs::read_to_string("inputs/day2.txt")?;
    let ranges = ids.split(',');
    let mut invalid_ids = Vec::new();
    for range in ranges {
        let mut bounds = range.split('-');
        let (start, end) = (bounds.next().unwrap(), bounds.next().unwrap());
        //println!("{} contains {} and {}", range, start, end);
        let start = start.parse::<i64>().unwrap();
        let end = end.parse::<i64>().unwrap();
        let mut partial_invalid_ids = Vec::new();
        for id in start..=end {
            let digits = id.to_string().chars().collect::<Vec<char>>();
            let midpoint = digits.len() / 2;
            if digits.len() % 2 != 0 {
                continue;
            }

            let first_half = &digits[0..midpoint];
            let second_half = &digits[midpoint..];
            let is_same = first_half.iter().zip(second_half.iter()).all(|(a, b)| a == b);

            if is_same {
                partial_invalid_ids.push(id);
            }
        }

        if partial_invalid_ids.is_empty() {
            println!("{} contains no invalid IDs.", range);
        }
        else {
            match partial_invalid_ids.len() {
                1 => println!("{} has one invalid ID, {}", range, partial_invalid_ids[0]),
                2 => println!("{} has two invalid IDs, {} and {}", range, partial_invalid_ids[0], partial_invalid_ids[1]),
                _ => {
                    print!("{} has {} invalid IDs, ", range, partial_invalid_ids.len());
                    for (i, id) in partial_invalid_ids.iter().enumerate() {
                        if i == partial_invalid_ids.len() - 1 {
                            print!("and {}", id);
                        }
                        else {
                            print!("{}, ", id);
                        }
                    }
                    println!();
                }
            }
        }

        invalid_ids.extend(partial_invalid_ids);
    }

    let sum = invalid_ids.into_iter().sum::<i64>();
    println!("Sum of invalid IDs: {}", sum);

    Ok(())
}

pub fn solve_day2_puzzle_part2() -> Result<(), PuzzleError> {
    let ids = std::fs::read_to_string("inputs/day2.txt")?;
    let ranges = ids.split(',');
    let mut invalid_ids = Vec::new();
    for range in ranges {
        let mut bounds = range.split('-');
        let (start, end) = (bounds.next().unwrap(), bounds.next().unwrap());
        //println!("{} contains {} and {}", range, start, end);
        let start = start.parse::<i64>().unwrap();
        let end = end.parse::<i64>().unwrap();
        let mut partial_invalid_ids = Vec::new();
        for id in start..=end {
            let digits = id.to_string().chars().collect::<Vec<char>>();
            let len = digits.len();
            let midpoint = digits.len() / 2;
            for i in 1..=midpoint {
                if len % i != 0 {
                    continue;
                }

                let chunk_size = i;
                let mut chunks = digits.chunks(chunk_size);
                let first = chunks.next().unwrap();
                let are_all_same = chunks.all(|chunk| chunk == first);
                if are_all_same {
                    //println!("{} is invalid because all chunks ({:?}) are the same.", id, first);
                    partial_invalid_ids.push(id);
                    break;
                }
            }
        }

        if partial_invalid_ids.is_empty() {
            println!("{} contains no invalid IDs.", range);
        }
        else {
            match partial_invalid_ids.len() {
                1 => println!("{} has one invalid ID, {}", range, partial_invalid_ids[0]),
                2 => println!("{} has two invalid IDs, {} and {}", range, partial_invalid_ids[0], partial_invalid_ids[1]),
                _ => {
                    print!("{} has {} invalid IDs, ", range, partial_invalid_ids.len());
                    for (i, id) in partial_invalid_ids.iter().enumerate() {
                        if i == partial_invalid_ids.len() - 1 {
                            print!("and {}", id);
                        }
                        else {
                            print!("{}, ", id);
                        }
                    }
                    println!();
                }
            }
        }

        invalid_ids.extend(partial_invalid_ids);
    }

    let sum = invalid_ids.into_iter().sum::<i64>();
    println!("Sum of invalid IDs: {}", sum);

    Ok(())
}

#[test]
fn test_solve_day2_puzzle_part1() {
    assert!(solve_day2_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day2_puzzle_part2() {
    assert!(solve_day2_puzzle_part2().is_ok());
}