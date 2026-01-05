use common::error::PuzzleError;

pub fn solve_day25_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day25.txt")?;
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    let row: usize = parts[15].trim_end_matches(',').parse().unwrap();
    let col: usize = parts[17].trim_end_matches('.').parse().unwrap();
    let size = row.max(col);
    //let mut table = vec![vec![0u64; size]; size];
    const MULTIPLIER: u64 = 252533;
    const MODULO: u64 = 33554393;
    //table[0][0] = 20151125;
    //let mut last_value = table[0][0];
    let mut last_value: u64 = 20151125;
    let mut diag = 1;
    'outer:
    loop {
        diag += 1;
        let start_row = diag;
        let start_col = 1;
        //println!("Filling diagonal {}, starting at ({}, {})", diag, start_row, start_col);
        let mut r = start_row;
        let mut c = start_col;
        while r >= 1 && c <= diag {
            last_value = (last_value * MULTIPLIER) % MODULO;
            //println!("At row {}, col {}: code {}", r, c, last_value);
            //table[r][c] = last_value;
            if r == row && c == col {
                println!("Code at row {}, col {} is {}", row, col, last_value);
                break 'outer;
            }

            // if r == 6 && c == 6 {
            //     println!("Debug: Reached (6, 6)");
            //     break 'outer;
            // }

            r = r.saturating_sub(1);
            c += 1;
        }
    }

    //println!("Code at row {}, col {} is {}", row, col, table[row - 1][col - 1]);

    Ok(())
}

#[test]
fn test_day25_part1() {
    let result = solve_day25_puzzle_part1();
    assert!(result.is_ok());
}