use serde::Deserialize;

use common::error::PuzzleError;

#[derive(Debug, Deserialize)]
struct AuntInterests {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>
}

impl AuntInterests {
    fn new(children: Option<u32>, cats: Option<u32>, samoyeds: Option<u32>, pomeranians: Option<u32>, akitas: Option<u32>, vizslas: Option<u32>, goldfish: Option<u32>, trees: Option<u32>, cars: Option<u32>, perfumes: Option<u32>) -> Self {
        AuntInterests {
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes
        }
    }
}

pub fn solve_day16_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day16.txt")?;
    let lines = input.lines();
    let mut aunts = Vec::new();
    for line in lines {
        let parts = line.splitn(2, ": ").collect::<Vec<&str>>();
        let interests_part = parts[1];
        let json = format!("{{\"{}}}", interests_part.replace(", ", ", \"").replace(": ", "\": "));
        let aunt: AuntInterests = serde_json::from_str(&json)?;
        aunts.push(aunt);
    }

    let target_aunt = AuntInterests::new(
        Some(3),
        Some(7),
        Some(2),
        Some(3),
        Some(0),
        Some(0),
        Some(5),
        Some(3),
        Some(2),
        Some(1)
    );

    for (index, aunt) in aunts.iter().enumerate() {
        if let Some(children) = aunt.children {
            if children != target_aunt.children.unwrap() {
                continue;
            }
        }
        if let Some(cats) = aunt.cats {
            if cats != target_aunt.cats.unwrap() {
                continue;
            }
        }
        if let Some(samoyeds) = aunt.samoyeds {
            if samoyeds != target_aunt.samoyeds.unwrap() {
                continue;
            }
        }
        if let Some(pomeranians) = aunt.pomeranians {
            if pomeranians != target_aunt.pomeranians.unwrap() {
                continue;
            }
        }
        if let Some(akitas) = aunt.akitas {
            if akitas != target_aunt.akitas.unwrap() {
                continue;
            }
        }
        if let Some(vizslas) = aunt.vizslas {
            if vizslas != target_aunt.vizslas.unwrap() {
                continue;
            }
        }
        if let Some(goldfish) = aunt.goldfish {
            if goldfish != target_aunt.goldfish.unwrap() {
                continue;
            }
        }
        if let Some(trees) = aunt.trees {
            if trees != target_aunt.trees.unwrap() {
                continue;
            }
        }
        if let Some(cars) = aunt.cars {
            if cars != target_aunt.cars.unwrap() {
                continue;
            }
        }
        if let Some(perfumes) = aunt.perfumes {
            if perfumes != target_aunt.perfumes.unwrap() {
                continue;
            }
        }

        println!("Found matching Aunt Sue at index: {}", index + 1);
        break;
    }

    Ok(())
}

pub fn solve_day16_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day16.txt")?;
    let lines = input.lines();
    let mut aunts = Vec::new();
    for line in lines {
        let parts = line.splitn(2, ": ").collect::<Vec<&str>>();
        let interests_part = parts[1];
        let json = format!("{{\"{}}}", interests_part.replace(", ", ", \"").replace(": ", "\": "));
        let aunt: AuntInterests = serde_json::from_str(&json)?;
        aunts.push(aunt);
    }

    let target_aunt = AuntInterests::new(
        Some(3),
        Some(7),
        Some(2),
        Some(3),
        Some(0),
        Some(0),
        Some(5),
        Some(3),
        Some(2),
        Some(1)
    );

    for (index, aunt) in aunts.iter().enumerate() {
        if let Some(children) = aunt.children {
            if children != target_aunt.children.unwrap() {
                continue;
            }
        }
        if let Some(cats) = aunt.cats {
            if cats <= target_aunt.cats.unwrap() {
                continue;
            }
        }
        if let Some(samoyeds) = aunt.samoyeds {
            if samoyeds != target_aunt.samoyeds.unwrap() {
                continue;
            }
        }
        if let Some(pomeranians) = aunt.pomeranians {
            if pomeranians >= target_aunt.pomeranians.unwrap() {
                continue;
            }
        }
        if let Some(akitas) = aunt.akitas {
            if akitas != target_aunt.akitas.unwrap() {
                continue;
            }
        }
        if let Some(vizslas) = aunt.vizslas {
            if vizslas != target_aunt.vizslas.unwrap() {
                continue;
            }
        }
        if let Some(goldfish) = aunt.goldfish {
            if goldfish >= target_aunt.goldfish.unwrap() {
                continue;
            }
        }
        if let Some(trees) = aunt.trees {
            if trees <= target_aunt.trees.unwrap() {
                continue;
            }
        }
        if let Some(cars) = aunt.cars {
            if cars != target_aunt.cars.unwrap() {
                continue;
            }
        }
        if let Some(perfumes) = aunt.perfumes {
            if perfumes != target_aunt.perfumes.unwrap() {
                continue;
            }
        }

        println!("Found matching Aunt Sue at index: {}", index + 1);
        break;
    }

    Ok(())
}

#[test]
fn test_day16_part1() {
    let result = solve_day16_puzzle_part1();
    assert!(result.is_ok());
}

#[test]
fn test_day16_part2() {
    let result = solve_day16_puzzle_part2();
    assert!(result.is_ok());
}