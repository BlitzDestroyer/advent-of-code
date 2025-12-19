use std::{collections::HashSet, vec};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    fn new(x: i64, y: i64) -> Self {
        Coordinates { x, y }
    }
    
    fn from_str(coords: &str) -> Self {
        let parts: Vec<i64> = coords
            .split(',')
            .map(|part| part.trim().parse::<i64>().unwrap())
            .collect();
        
        Coordinates {
            x: parts[0],
            y: parts[1],
        }
    }
}

impl std::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn solve_day9_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day9.txt")?;
    let coords = input.lines().map(|line| Coordinates::from_str(line)).collect::<Vec<Coordinates>>();
    let mut max_area = 0;
    for (i, coord) in coords.iter().enumerate() {
        for j in (i + 1)..coords.len() {
            let other = &coords[j];
            let area = ((coord.x - other.x + 1).abs() * (coord.y - other.y + 1).abs()) as usize;
            if area > max_area {
                max_area = area;
            }
        }
    }
    
    println!("Maximum area between any two coordinates: {}", max_area);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Red,
    Green,
}

pub fn solve_day9_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day9.txt")?;
    let coords = input.lines().map(|line| Coordinates::from_str(line)).collect::<Vec<Coordinates>>();
    let mut x_coords: Vec<_> = coords.iter().map(|c| c.x).collect();
    let mut y_coords: Vec<_> = coords.iter().map(|c| c.y).collect();

    x_coords.sort();
    x_coords.dedup();
    y_coords.sort();
    y_coords.dedup();

    let coords = coords.into_iter().map(|c| {
        let Coordinates { x, y } = c;
        let new_x = x_coords.binary_search(&x).unwrap() as i64 * 2;
        let new_y = y_coords.binary_search(&y).unwrap() as i64 * 2;
        Coordinates::new(new_x, new_y)
    })
    .collect::<Vec<Coordinates>>();

    println!("Decompressed coordinates: {:?}", coords);

    //let x_max = coords.iter().map(|c| c.x).max().unwrap() as usize;
    //let y_max = coords.iter().map(|c| c.y).max().unwrap() as usize;
    //let mut grid = vec![vec![Tile::Empty; x_max + 1]; y_max + 1];
    let mut grid = vec![vec![Tile::Empty; x_coords.len() * 2]; y_coords.len() * 2];
    for coord in &coords {
        let Coordinates { x, y } = coord;
        grid[*y as usize][*x as usize] = Tile::Red;
    }

    draw_edges(&mut grid, &coords);
    fill_polygon(&mut grid, &coords);

    print_grid(&grid, "debug_outputs/day9_grid4.txt");

    // for row in &grid {
    //     for tile in row {
    //         match tile {
    //             Tile::Empty => print!("."),
    //             Tile::Red => print!("#"),
    //             Tile::Green => print!("X"),
    //         }
    //     }
    //     println!();
    // }

    //let prefix_table = build_prefix_table(&grid);
    let mut max_area = 0;
    let mut coords_pair = (0, 0, 0, 0, 0, 0, 0, 0);
    for (i, coord) in coords.iter().enumerate() {
        for j in (i + 1)..coords.len() {
            let Coordinates { x: x1, y: y1 } = coord;
            let Coordinates { x: x2, y: y2 } = &coords[j];
            //let decomp_x1 = x_coords[*x1 as usize / 2];
            //let decomp_x2 = x_coords[*x2 as usize / 2];
            //let decomp_y1 = y_coords[*y1 as usize / 2];
            //let decomp_y2 = y_coords[*y2 as usize / 2];
            //let x_dist = (decomp_x1 - decomp_x2).abs();
            //let y_dist = (decomp_y1 - decomp_y2).abs();
            //println!("Checking coordinates pair: ({}, {}) and ({}, {}): Width: {}, Length: {}, Area: {}", decomp_x1, decomp_y1, decomp_x2, decomp_y2, x_dist + 1, y_dist + 1, (x_dist + 1) * (y_dist + 1));
            let (low_x, high_x) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            let (low_y, high_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

            let mut enclosed = true;
            for x in *low_x..*high_x {
                if matches!(grid[*low_y as usize][x as usize], Tile::Empty) ||
                   matches!(grid[*high_y as usize][x as usize], Tile::Empty) {
                    //println!("Not enclosed on top/bottom edge between ({}, {}) and ({}, {})", x, low_y, x, high_y);
                    enclosed = false;
                    break;
                }
            }

            if enclosed {
                for y in *low_y..*high_y {
                    if matches!(grid[y as usize][*low_x as usize], Tile::Empty) ||
                       matches!(grid[y as usize][*high_x as usize], Tile::Empty) {
                        //println!("Not enclosed on left/right edge between ({}, {}) and ({}, {})", low_x, y, high_x, y);
                        enclosed = false;
                        break;
                    }
                }
            }

            if !enclosed {
                continue;
            }

            // let empty_count = get_prefix_sum(&prefix_table, *low_x as usize, *low_y as usize, *high_x as usize, *high_y as usize);
            // if empty_count > 0 {
            //     continue;
            // }

            // Decompress coordinates
            let (decomp_x1, decomp_x2) = (x_coords[*x1 as usize / 2], x_coords[*x2 as usize / 2]);
            let (decomp_y1, decomp_y2) = (y_coords[*y1 as usize / 2], y_coords[*y2 as usize / 2]);
            let area = (((decomp_x1 - decomp_x2).abs() + 1) * ((decomp_y1 - decomp_y2).abs() + 1)) as usize;
            if area > max_area {
                max_area = area;
                coords_pair = (*x1, *y1, *x2, *y2, decomp_x1, decomp_y1, decomp_x2, decomp_y2);
            }
        }
    }
    
    println!("Maximum area between any two coordinates: {}", max_area);
    let (x1, y1, x2, y2, decomp_x1, decomp_y1, decomp_x2, decomp_y2) = coords_pair;
    println!("Coordinates pair with maximum area: ({}, {}) and ({}, {}) which decompress to ({}, {}) and ({}, {})", x1, y1, x2, y2, decomp_x1, decomp_y1, decomp_x2, decomp_y2);
    Ok(())
}

fn _build_prefix_table(grid: &Vec<Vec<Tile>>) -> Vec<Vec<i64>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut prefix_table = vec![vec![0; cols + 1]; rows + 1];

    for r in 1..=rows {
        for c in 1..=cols {
            let add = if matches!(grid[r - 1][c - 1], Tile::Empty) { 1 } else { 0 };
            let above = prefix_table[r - 1][c];
            let left = prefix_table[r][c - 1];
            let diag = prefix_table[r - 1][c - 1];
            prefix_table[r][c] = above + left - diag + add;
        }
    }

    prefix_table
}

fn print_grid(grid: &Vec<Vec<Tile>>, out_path: &str) {
    let mut output = String::new();
    for row in grid {
        for tile in row {
            match tile {
                Tile::Empty => output.push('.'),
                Tile::Red => output.push('#'),
                Tile::Green => output.push('X'),
            }
        }
        output.push('\n');
    }

    std::fs::write(out_path, output).expect("Failed to write grid to file");
}

fn _get_prefix_sum(prefix_table: &Vec<Vec<i64>>, x0: usize, y0: usize, x1: usize, y1: usize) -> i64 {
    prefix_table[y1 + 1][x1 + 1] - prefix_table[y0][x1 + 1] - prefix_table[y1 + 1][x0] + prefix_table[y0][x0]
}

fn fill_polygon(grid: &mut Vec<Vec<Tile>>, vertices: &Vec<Coordinates>) {
    let mut filled = HashSet::new();

    let min_y = vertices.iter().map(|v| v.y).min().unwrap();
    let max_y = vertices.iter().map(|v| v.y).max().unwrap();

    for y in min_y..max_y {
        let mut xs = Vec::new();

        for i in 0..vertices.len() {
            let v1 = &vertices[i];
            let v2 = &vertices[(i + 1) % vertices.len()];

            if v1.y == v2.y {
                continue;
            }

            let (low, high) = if v1.y < v2.y {
                (v1, v2)
            } else {
                (v2, v1)
            };

            // half-open: include lower, exclude upper
            if y >= low.y && y < high.y {
                xs.push(low.x);
            }
        }

        xs.sort_unstable();

        for pair in xs.chunks_exact(2) {
            let (x_start, x_end) = (pair[0], pair[1]);
            
            for x in x_start..=x_end {
                filled.insert((x, y));
            }
        }
    }

    for (x, y) in filled {
        if matches!(grid[y as usize][x as usize], Tile::Red) {
            continue;
        }

        grid[y as usize][x as usize] = Tile::Green;
    }
}

fn draw_edges(grid: &mut Vec<Vec<Tile>>, vertices: &[Coordinates]) {
    for i in 0..vertices.len() {
        let a = &vertices[i];
        let b = &vertices[(i + 1) % vertices.len()];

        if a.x == b.x {
            // vertical edge
            let x = a.x;
            let (y0, y1) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
            for y in y0..=y1 {
                if matches!(grid[y as usize][x as usize], Tile::Red) {
                    continue;
                }

                grid[y as usize][x as usize] = Tile::Green;
            }
        }
        else if a.y == b.y {
            // horizontal edge
            let y = a.y;
            let (x0, x1) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
            for x in x0..=x1 {
                if matches!(grid[y as usize][x as usize], Tile::Red) {
                    continue;
                }

                grid[y as usize][x as usize] = Tile::Green;
            }
        }
    }
}

#[test]
fn test_solve_day9_puzzle_part1() {
    assert!(solve_day9_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day9_puzzle_part2() {
    assert!(solve_day9_puzzle_part2().is_ok());
}