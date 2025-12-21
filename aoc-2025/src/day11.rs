use std::collections::HashMap;

use flagset::{FlagSet, flags};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn solve_day11_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day11.txt")?;
    let devices = input.lines().map(|line| {
        let mut parts = line.split(':');
        let key = parts.next().unwrap();
        let values = parts.next().unwrap().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        (key.to_string(), values)
    })
    .collect::<HashMap<String, Vec<String>>>();

    println!("Devices: {:?}", devices);
    let total_paths = solve_day11_part1_helper(&devices, &mut HashMap::new(), "you");
    println!("Total paths to reach reactor: {}", total_paths);

    Ok(())
}

fn solve_day11_part1_helper(devices: &HashMap<String, Vec<String>>, memo: &mut HashMap<String, usize>, device: &str) -> usize {
    if memo.contains_key(device) {
        return memo[device];
    }

    if device == "out" {
        return 1;
    }

    let output_list = devices.get(device).unwrap();
    let mut total_paths = 0;
    for output in output_list {
        let num_paths = solve_day11_part1_helper(devices, memo, output);
        total_paths += num_paths;
    }

    memo.insert(device.to_string(), total_paths);

    total_paths
}

flags! {
    #[derive(Hash)]
    enum VisitedDevices : u8 {
        None = 0,
        DAC = 1 << 0,
        FFT = 1 << 1,
        ALL = (VisitedDevices::DAC | VisitedDevices::FFT).bits(),
    }
}

pub fn solve_day11_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day11.txt")?;
    let devices = input.lines().map(|line| {
        let mut parts = line.split(':');
        let key = parts.next().unwrap();
        let values = parts.next().unwrap().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        (key.to_string(), values)
    })
    .collect::<HashMap<String, Vec<String>>>();

    println!("Devices: {:?}", devices);
    let total_paths = solve_day11_part2_helper(&devices, &mut HashMap::new(), "svr", VisitedDevices::None.into());
    println!("Total paths to reach reactor: {}", total_paths);

    Ok(())
}

fn solve_day11_part2_helper<'a>(devices: &'a HashMap<String, Vec<String>>, memo: &mut HashMap<(&'a str, FlagSet<VisitedDevices>), usize>, device: &'a str, visited: FlagSet<VisitedDevices>) -> usize {
    if device == "out" {
        return if visited == VisitedDevices::ALL { 1 } else { 0 };
    }
    
    if let Some(&num_paths) = memo.get(&(device, visited)) {
        return num_paths;
    }

    let visited = match device {
        "dac" => visited | VisitedDevices::DAC,
        "fft" => visited | VisitedDevices::FFT,
        _ => visited,
    };

    //println!("Visiting device: {}, visited: {:?}", device, visited);

    let output_list = devices.get(device).unwrap();
    let mut total_paths = 0;
    for output in output_list {
        let num_paths = solve_day11_part2_helper(devices, memo, output, visited);
        total_paths += num_paths;
    }

    memo.insert((device, visited), total_paths);

    total_paths
}

#[test]
fn test_solve_day11_puzzle_part1() {
    assert!(solve_day11_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day11_puzzle_part2() {
    assert!(solve_day11_puzzle_part2().is_ok());
}