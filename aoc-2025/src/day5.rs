use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn solve_day5_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day5.txt")?;
    let mut lines = input.lines();
    let mut valid_items_ranges = Vec::new();
    for line in &mut lines {
        if line.trim().is_empty() {
            break;
        }

        let mut parts = line.split('-');
        let lower = parts.next().unwrap().parse::<i64>().unwrap();
        let upper = parts.next().unwrap().parse::<i64>().unwrap();
        valid_items_ranges.push((lower, upper));
    }

    let mut valid_count = 0;
    for line in lines {
        let item = line.trim().parse::<i64>().unwrap();
        let is_valid = valid_items_ranges.iter().any(|(lower, upper)| item >= *lower && item <= *upper);
        if is_valid {
            valid_count += 1;
        }
    }

    println!("Number of valid items: {}", valid_count);
    
    Ok(())
}

pub fn solve_day5_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day5.txt")?;
    let mut lines = input.lines();
    let mut valid_items_ranges = Vec::new();
    for line in &mut lines {
        if line.trim().is_empty() {
            break;
        }

        let mut parts = line.split('-');
        let lower = parts.next().unwrap().parse::<i64>().unwrap();
        let upper = parts.next().unwrap().parse::<i64>().unwrap();
        valid_items_ranges.push((lower, upper));
    }

    valid_items_ranges.sort_by_key(|&(l, _)| l);
    let mut cleaned_ranges = Vec::new();
    for (mut lower, mut upper) in valid_items_ranges {
        let mut skip = false;
        for (cl_lower, cl_upper) in &cleaned_ranges {
            if lower > *cl_upper || upper < *cl_lower {
                continue;
            }

            if lower >= *cl_lower && upper <= *cl_upper {
                skip = true;
                break;
            }

            if lower < *cl_lower && upper >= *cl_lower && upper <= *cl_upper {
                upper = *cl_lower - 1;
            }
            else if lower <= *cl_upper && upper > *cl_upper {
                lower = *cl_upper + 1;
            }
        }

        if !skip {
            cleaned_ranges.push((lower, upper));
        }
    }

    //println!("Cleaned ranges: {:?}", cleaned_ranges);

    let mut valid_count = 0;
    for (lower, upper) in &cleaned_ranges {
        valid_count += upper - lower + 1;
    }

    println!("Number of valid items: {}", valid_count);
    
    Ok(())
}

#[test]
fn test_solve_day5_puzzle_part1() {
    assert!(solve_day5_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day5_puzzle_part2() {
    assert!(solve_day5_puzzle_part2().is_ok());
}