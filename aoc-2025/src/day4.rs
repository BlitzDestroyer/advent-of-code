use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

enum Tile {
    Empty,
    Paper
}

pub fn solve_day4_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day4.txt")?;
    let lines = input.lines();
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    for line in lines {
        let row: Vec<Tile> = line.chars().map(|c| {
            match c {
                '.' => Tile::Empty,
                '@' => Tile::Paper,
                _ => Tile::Empty
            }
        }).collect();

        grid.push(row);
    }

    let mut accessable_paper_tiles = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, tile) in row.iter().enumerate() {
            let mut surrounding_paper_count = 0;
            // Check left
            if c > 0 && matches!(row[c - 1], Tile::Paper) {
                surrounding_paper_count += 1;
            }

            // Check right
            if c < row.len() - 1 && matches!(row[c + 1], Tile::Paper) {
                surrounding_paper_count += 1;
            }

            // Check up row
            if r > 0 {
                // Check up-left
                if c > 0 && matches!(grid[r - 1][c - 1], Tile::Paper) {
                    surrounding_paper_count += 1;
                }
                // Check up
                if matches!(grid[r - 1][c], Tile::Paper) {
                    surrounding_paper_count += 1;
                }
                // Check up-right
                if c < row.len() - 1 && matches!(grid[r - 1][c + 1], Tile::Paper) {
                    surrounding_paper_count += 1;
                }
            }

            // Check down row
            if r < grid.len() - 1 {
                // Check down-left
                if c > 0 && matches!(grid[r + 1][c - 1], Tile::Paper) {
                    surrounding_paper_count += 1;
                }
                // Check down
                if matches!(grid[r + 1][c], Tile::Paper) {
                    surrounding_paper_count += 1;
                }
                // Check down-right
                if c < row.len() - 1 && matches!(grid[r + 1][c + 1], Tile::Paper) {
                    surrounding_paper_count += 1;
                }
            }

            if surrounding_paper_count < 4 && matches!(tile, Tile::Paper) {
                accessable_paper_tiles += 1;
            }
        }
    }

    println!("Number of accessable paper tiles: {}", accessable_paper_tiles);

    Ok(())
}

pub fn solve_day4_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day4.txt")?;
    let lines = input.lines();
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    for line in lines {
        let row: Vec<Tile> = line.chars().map(|c| {
            match c {
                '.' => Tile::Empty,
                '@' => Tile::Paper,
                _ => Tile::Empty
            }
        }).collect();

        grid.push(row);
    }

    let mut accessable_paper_tiles = 0;
    loop {
        let (accessable_tiles, indices) = day4_helper(&grid);
        if accessable_tiles == 0 {
            break;
        }

        accessable_paper_tiles += accessable_tiles;
        for (r, c) in indices {
            grid[r][c] = Tile::Empty;
        }
    }
    

    println!("Number of accessable paper tiles: {}", accessable_paper_tiles);

    Ok(())
}

fn day4_helper(grid: &Vec<Vec<Tile>>) -> (i64, Vec<(usize, usize)>) {
    let mut accessable_paper_tiles = 0;
    let mut indices = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, tile) in row.iter().enumerate() {
            let mut surrounding_paper_count = 0;
            // Check left
            if c > 0 && matches!(row[c - 1], Tile::Paper) {
                surrounding_paper_count += 1;
            }

            // Check right
            if c < row.len() - 1 && matches!(row[c + 1], Tile::Paper) {
                surrounding_paper_count += 1;
            }

            // Check up row
            if r > 0 {
                // Check up-left
                if c > 0 && matches!(grid[r - 1][c - 1], Tile::Paper) {
                    surrounding_paper_count += 1;
                }
                // Check up
                if matches!(grid[r - 1][c], Tile::Paper) {
                    surrounding_paper_count += 1;
                }
                // Check up-right
                if c < row.len() - 1 && matches!(grid[r - 1][c + 1], Tile::Paper) {
                    surrounding_paper_count += 1;
                }
            }

            // Check down row
            if r < grid.len() - 1 {
                // Check down-left
                if c > 0 && matches!(grid[r + 1][c - 1], Tile::Paper) {
                    surrounding_paper_count += 1;
                }
                // Check down
                if matches!(grid[r + 1][c], Tile::Paper) {
                    surrounding_paper_count += 1;
                }
                // Check down-right
                if c < row.len() - 1 && matches!(grid[r + 1][c + 1], Tile::Paper) {
                    surrounding_paper_count += 1;
                }
            }

            if surrounding_paper_count < 4 && matches!(tile, Tile::Paper) {
                accessable_paper_tiles += 1;
                indices.push((r, c));
            }
        }
    }

    (accessable_paper_tiles, indices)
}

#[test]
fn test_solve_day4_puzzle_part1() {
    assert!(solve_day4_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day4_puzzle_part2() {
    assert!(solve_day4_puzzle_part2().is_ok());
}