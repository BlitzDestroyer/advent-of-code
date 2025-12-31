use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn solve_day2_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day2.txt")?;
    let mut total_paper = 0;

    for line in input.lines() {
        let dims: Vec<usize> = line
            .split('x')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let (l, w, h) = (dims[0], dims[1], dims[2]);

        let side1 = l * w;
        let side2 = w * h;
        let side3 = h * l;

        let surface_area = 2 * (side1 + side2 + side3);
        let slack = *[side1, side2, side3].iter().min().unwrap();

        total_paper += surface_area + slack;
    }

    println!("Total wrapping paper needed: {}", total_paper);

    Ok(())
}

pub fn solve_day2_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day2.txt")?;
    let mut total_ribbon = 0;

    for line in input.lines() {
        let dims: Vec<usize> = line
            .split('x')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let (l, w, h) = (dims[0], dims[1], dims[2]);

        let volume = l * w * h;
        let min_perimeter = 2 * (l + w + h - dims.iter().max().unwrap());

        total_ribbon += volume + min_perimeter;
    }

    println!("Total ribbon needed: {}", total_ribbon);

    Ok(())
}

#[test]
fn test_day2_puzzle_part1() {
    assert!(solve_day2_puzzle_part1().is_ok());
}

#[test]
fn test_day2_puzzle_part2() {
    assert!(solve_day2_puzzle_part2().is_ok());
}