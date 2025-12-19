use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn solve_day3_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day3.txt")?;
    let lines = input.lines();
    let mut max_values = Vec::new();
    for line in lines {
        let values = line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
        let len = values.len();

        let mut max = values[0];
        let mut max_index = 0;
        for (i, &value) in values[..len - 1].iter().enumerate() {
            if value > max {
                max = value;
                max_index = i;
            }
        }

        let mut second_max = values[max_index + 1];
        for (_, &value) in values[max_index + 1..].iter().enumerate() {
            if value > second_max {
                second_max = value;
            }
        }

        let max_value = (max * 10) + second_max;
        println!("Max value of {}: {}", line, max_value);
        max_values.push(max_value);
    }

    let total_joltage = max_values.iter().sum::<u32>();
    println!("Total joltage: {}", total_joltage);

    Ok(())
}

pub fn solve_day3_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day3.txt")?;
    let lines = input.lines();
    let mut max_values = Vec::new();
    for line in lines {
        let values = line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<_>>();
        //println!("Values: {:?}", values);
        let max_value = part_2_helper(&values, 11);
        println!("Max value of {}: {}", line, max_value);
        max_values.push(max_value);
    }

    let total_joltage = max_values.iter().sum::<u64>();
    println!("Total joltage: {}", total_joltage);

    Ok(())
}

fn part_2_helper(values: &[u64], remaining: u64) -> u64 {
    let mut max = values[0];
    let mut max_index = 0;
    for (i, &value) in values[..values.len() - remaining as usize].iter().enumerate() {
        if value > max {
            max = value;
            max_index = i;
        }
    }

    //println!("Max value: {}, index: {}, remaining: {}", max, max_index, remaining);
    if remaining == 0 {
        return max
    }

    let sub_max = part_2_helper(&values[max_index + 1..], remaining - 1);
    (max * 10u64.pow(remaining as u32)) + sub_max
}

#[test]
fn test_solve_day3_puzzle_part1() {
    assert!(solve_day3_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day3_puzzle_part2() {
    assert!(solve_day3_puzzle_part2().is_ok());
}