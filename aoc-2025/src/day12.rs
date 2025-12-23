use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
struct Present {
    _id: usize,
    area: usize,
    _cells: Vec<(i32, i32)>, // (dx, dy) offsets
}

impl Present {
    fn from_shape(shape: Vec<Vec<u8>>) -> Self {
        let mut cells = Vec::new();

        // id is the only non-zero value - 1 used
        let id = (*shape
            .iter()
            .flat_map(|row| row.iter())
            .find(|&&c| c != 0)
            .unwrap()) as usize
            - 1;
        for (y, row) in shape.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c != 0 {
                    cells.push((x as i32, y as i32));
                }
            }
        }

        // Normalize so top-left occupied cell is (0,0)
        let min_x = cells.iter().map(|(x, _)| *x).min().unwrap();
        let min_y = cells.iter().map(|(_, y)| *y).min().unwrap();

        for (x, y) in &mut cells {
            *x -= min_x;
            *y -= min_y;
        }

        let area = cells.len();

        Present { _id: id, area, _cells: cells }
    }
}

#[derive(Debug)]
struct XMasTree {
    width: usize,
    height: usize,
    gifts: Vec<usize>,
}

impl XMasTree {
    fn from_str(line: &str) -> Self {
        let mut parts = line.split(':');
        let dimensions = parts.next().unwrap();
        let mut dim_parts = dimensions.split('x');
        let width = dim_parts.next().unwrap().parse::<usize>().unwrap();
        let height = dim_parts.next().unwrap().parse::<usize>().unwrap();

        let gifts = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        XMasTree {
            width,
            height,
            gifts,
        }
    }

    fn solve_part1(&self, presents: &Vec<Present>) -> bool {
        let total_presents = self.gifts.iter().sum::<usize>();
        let fit_width = self.width / 3;
        let fit_height = self.height / 3;
        if total_presents <= fit_width * fit_height {
            return true; // All presents must fit regardless of shape as maximial shape is 3x3
        }

        let total_present_area: usize = self
            .gifts
            .iter()
            .enumerate()
            .map(|(id, &count)| presents[id].area * count)
            .sum();

        if total_present_area > self.width * self.height {
            return false; // Definitely cannot fit
        }

        // Undecided case
        panic!("Undecided case in solve_part1_v2")
    }
}

pub fn solve_day12_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day12.txt")?;
    let mut lines = input.lines();
    let mut shapes = Vec::new();
    let mut shape = Vec::new();
    let mut past_iteration = None;
    for line in &mut lines {
        if line.contains('x') {
            past_iteration = Some(line);
            break;
        }

        if line.contains(':') {
            // Skip indices as they are already handled by the shapes vec length
            continue;
        }

        if line.trim().is_empty() {
            shapes.push(shape);
            shape = Vec::new();
            continue;
        }

        let row = line
            .chars()
            .map(|c| match c {
                '#' => shapes.len() as u8 + 1,
                '.' => 0,
                _ => panic!("Invalid char {}", c),
            })
            .collect::<Vec<u8>>();
        shape.push(row);
    }

    let presents = shapes
        .into_iter()
        .map(|s| Present::from_shape(s))
        .collect::<Vec<Present>>();

    let mut xmas_trees = Vec::new();
    if let Some(line) = past_iteration {
        let xmas_tree = XMasTree::from_str(line);
        xmas_trees.push(xmas_tree);
    }

    for line in lines {
        let xmas_tree = XMasTree::from_str(line);
        xmas_trees.push(xmas_tree);
    }

    let mut solvable_count = 0;
    for tree in &xmas_trees {
        let can_solve = tree.solve_part1(&presents);
        if can_solve {
            solvable_count += 1;
        }

        println!("Can solve tree {:?}: {}", tree, can_solve);
    }

    println!("Total solvable trees: {}", solvable_count);

    Ok(())
}

#[test]
fn test_day12_part1() {
    assert!(solve_day12_puzzle_part1().is_ok());
}
