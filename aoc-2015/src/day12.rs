use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn solve_day12_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day12.txt")?;
    let json: serde_json::Value = serde_json::from_str(&input)?;
    let sum = sum_numbers_in_json_part1(&json);
    println!("Sum of all numbers: {}", sum);

    Ok(())
}

fn sum_numbers_in_json_part1(value: &serde_json::Value) -> i64 {
    match value {
        serde_json::Value::Number(n) => n.as_i64().unwrap_or(0),
        serde_json::Value::Array(arr) => arr.iter().map(sum_numbers_in_json_part1).sum(),
        serde_json::Value::Object(map) => map.values().map(sum_numbers_in_json_part1).sum(),
        _ => 0,
    }
}

pub fn solve_day12_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day12.txt")?;
    let json: serde_json::Value = serde_json::from_str(&input)?;
    let sum = sum_numbers_in_json_part2(&json);
    println!("Sum of all numbers: {}", sum);

    Ok(())
}

fn sum_numbers_in_json_part2(value: &serde_json::Value) -> i64 {
    match value {
        serde_json::Value::Number(n) => n.as_i64().unwrap_or(0),
        serde_json::Value::Array(arr) => arr.iter().map(sum_numbers_in_json_part2).sum(),
        serde_json::Value::Object(map) => {
            let mut sum = 0;
            for v in map.values() {
                if v == "red" {
                    return 0;
                }
                
                sum += sum_numbers_in_json_part2(v);
            }

            sum
        },
        _ => 0,
    }
}

#[test]
fn test_solve_day12_puzzle_part1() {
    assert!(solve_day12_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day12_puzzle_part2() {
    assert!(solve_day12_puzzle_part2().is_ok());
}