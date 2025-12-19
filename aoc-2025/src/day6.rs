use std::vec;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn solve_day6_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day6.txt")?;
    let mut lines = input.lines();
    
    let mut values = Vec::new();
    let first_line = lines.next().unwrap();
    let parts = first_line.split_whitespace();
    for part in parts {
        let value = part.parse::<i64>().unwrap();
        values.push(vec![value]);
    }

    let mut operators = Vec::new();
    for line in lines {
        let parts = line.split_whitespace();
        for (i, part) in parts.enumerate() {
            let result = part.parse::<i64>();
            if let Ok(value) = result {
                values[i].push(value);
            }
            else {
                operators.push(part.chars().next().unwrap());
            }
        }
    }

    let mut total_value = 0;
    for (i, values_list) in values.iter().enumerate() {
        match operators[i] {
            '+' => {
                let total: i64 = values_list.iter().sum();
                println!("Total sum for column {}: {}", i + 1, total);
                total_value += total;
            },
            '*' => {
                let total: i64 = values_list.iter().product();
                println!("Total product for column {}: {}", i + 1, total);
                total_value += total;
            },
            _ => {
                println!("Unknown operator '{}' for column {}", operators[i], i + 1);
            }
        }
    }

    println!("Overall total value: {}", total_value);

    Ok(())
}

pub fn solve_day6_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day6.txt")?;
    let lines = input.lines();
    
    let mut rows = Vec::new();
    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();
        rows.push(chars);
    }

    let operators = rows[rows.len() - 1]
        .iter()
        .filter(|&&c| c == '+' || c == '*')
        .cloned()
        .collect::<Vec<char>>();

    let mut temp_str_values = vec![vec![String::new(); operators.len()]; rows.len() - 1];

    let mut current_col = 0;
    for row_char_idx in 0..rows[0].len() {
        if rows.iter().all(|row| row[row_char_idx] == ' ') {
            current_col += 1;
            continue;
        }

        for (i, row) in rows.iter().enumerate().take(rows.len() - 1) {
            temp_str_values[i][current_col].push(row[row_char_idx]);
        }
    }

    println!("Temp values: {:?}", temp_str_values);
    for col_idx in 0..temp_str_values[0].len() {
        let mut row_iter = temp_str_values.iter();
        let mut temp_col_values = Vec::new();
        let row = row_iter.next().unwrap();
        let col_value = &row[col_idx];
        temp_col_values.extend(col_value.chars().map(|c| c.to_string()));
        for row in row_iter  {
            let col_value = &row[col_idx];
            for (i, c) in col_value.chars().enumerate() {
                temp_col_values[i].push(c);
            }
        }
    }

    let mut transposed_temp_str_values = vec![vec![String::new(); rows.len() - 1]; operators.len()];
    for i in 0..temp_str_values.len() {
        for j in 0..temp_str_values[i].len() {
            transposed_temp_str_values[j][i] = temp_str_values[i][j].clone();
        }
    }

    println!("Transposed Temp values: {:?}", transposed_temp_str_values);

    let mut values = Vec::new();
    for row in transposed_temp_str_values {
        let mut temp = vec![String::new(); row[0].len()];
        for col_idx in 0..row[0].len() {
            for r in &row {
                let c = r.chars().nth(col_idx).unwrap();
                if c != ' ' {
                    temp[col_idx].push(c);
                }
            }
        }

        println!("Parsed temp values: {:?}", temp);
        let parsed_values = temp
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        values.push(parsed_values);
    }

    let mut total_value = 0;
    for (i, values_list) in values.iter().enumerate() {
        match operators[i] {
            '+' => {
                let total: i64 = values_list.iter().sum();
                println!("Total sum for column {}: {}", i + 1, total);
                total_value += total;
            },
            '*' => {
                let total: i64 = values_list.iter().product();
                println!("Total product for column {}: {}", i + 1, total);
                total_value += total;
            },
            _ => {
                println!("Unknown operator '{}' for column {}", operators[i], i + 1);
            }
        }
    }

    println!("Overall total value: {}", total_value);

    Ok(())
}


#[test]
fn test_solve_day6_puzzle_part1() {
    assert!(solve_day6_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day6_puzzle_part2() {
    assert!(solve_day6_puzzle_part2().is_ok());
}