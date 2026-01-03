use std::collections::HashMap;

use common::error::PuzzleError;

pub fn solve_day9_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day9.txt")?;
    let lines = input.lines();
    let mut cities = Vec::new();
    let mut edges: HashMap<usize, HashMap<usize, u32>> = HashMap::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let city1 = parts[0];
        let city2 = parts[2];
        let city1_index = if let Some(index) = cities.iter().position(|&c| c == city1) {
            index
        }
        else {
            cities.push(city1);
            cities.len() - 1
        };

        let city2_index = if let Some(index) = cities.iter().position(|&c| c == city2) {
            index
        }
        else {
            cities.push(city2);
            cities.len() - 1
        };

        let distance: u32 = parts[4].parse().unwrap();
        if let Some(city1_edges) = edges.get_mut(&city1_index) {
            city1_edges.insert(city2_index, distance);
        }
        else {
            let mut city1_edges = HashMap::new();
            city1_edges.insert(city2_index, distance);
            edges.insert(city1_index, city1_edges);
        }

        if let Some(city2_edges) = edges.get_mut(&city2_index) {
            city2_edges.insert(city1_index, distance);
        }
        else {
            let mut city2_edges = HashMap::new();
            city2_edges.insert(city1_index, distance);
            edges.insert(city2_index, city2_edges);
        }
    }

    let opt = tsp_brute_force(&edges);
    println!("Shortest path has length {}", opt);

    Ok(())
}

pub fn solve_day9_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day9.txt")?;
    let lines = input.lines();
    let mut cities = Vec::new();
    let mut edges: HashMap<usize, HashMap<usize, u32>> = HashMap::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let city1 = parts[0];
        let city2 = parts[2];
        let city1_index = if let Some(index) = cities.iter().position(|&c| c == city1) {
            index
        }
        else {
            cities.push(city1);
            cities.len() - 1
        };

        let city2_index = if let Some(index) = cities.iter().position(|&c| c == city2) {
            index
        }
        else {
            cities.push(city2);
            cities.len() - 1
        };

        let distance: u32 = parts[4].parse().unwrap();
        if let Some(city1_edges) = edges.get_mut(&city1_index) {
            city1_edges.insert(city2_index, distance);
        }
        else {
            let mut city1_edges = HashMap::new();
            city1_edges.insert(city2_index, distance);
            edges.insert(city1_index, city1_edges);
        }

        if let Some(city2_edges) = edges.get_mut(&city2_index) {
            city2_edges.insert(city1_index, distance);
        }
        else {
            let mut city2_edges = HashMap::new();
            city2_edges.insert(city1_index, distance);
            edges.insert(city2_index, city2_edges);
        }
    }

    let opt = tsp_brute_force_max(&edges);
    println!("Longest path has length {}", opt);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BitSet {
    value: u64,
}

impl BitSet {
    fn new() -> Self {
        BitSet { value: 0 }
    }

    fn insert(&mut self, index: usize) {
        self.value |= 1 << index;
    }

    fn contains(&self, index: usize) -> bool {
        (self.value & (1 << index)) != 0
    }

    fn remove(&mut self, index: usize) {
        self.value &= !(1 << index);
    }

    fn is_empty(&self) -> bool {
        self.value == 0
    }
}

fn tsp_brute_force(edges: &HashMap<usize, HashMap<usize, u32>>) -> u32 {
    let mut unvisited: BitSet = BitSet::new();
    for edge in edges.keys() {
        unvisited.insert(*edge);
    }

    let mut min_dist = u32::MAX;
    for edge in edges.keys() {
        let mut unvisted_clone = unvisited.clone();
        println!("Edge: {}", edge);
        unvisted_clone.remove(*edge);
        let dist = tsp_brute_force_helper(edges, &mut unvisted_clone, *edge);
        min_dist = min_dist.min(dist);
    }

    min_dist
}

fn tsp_brute_force_helper(edges: &HashMap<usize, HashMap<usize, u32>>, unvisited: &mut BitSet, current: usize) -> u32 {
    if unvisited.is_empty() {
        return 0;
    }
    
    let neighbors = edges.get(&current).unwrap();
    let mut min_dist = u32::MAX;
    for neighbor in neighbors.keys() {
        if unvisited.contains(*neighbor) {
            unvisited.remove(*neighbor);
            let dist = neighbors.get(neighbor).unwrap() + tsp_brute_force_helper(edges, unvisited, *neighbor);
            min_dist = min_dist.min(dist);
            unvisited.insert(*neighbor);
        }
    }

    min_dist
}

fn tsp_brute_force_max(edges: &HashMap<usize, HashMap<usize, u32>>) -> u32 {
    let mut unvisited: BitSet = BitSet::new();
    for edge in edges.keys() {
        unvisited.insert(*edge);
    }

    let mut max_dist = u32::MIN;
    for edge in edges.keys() {
        let mut unvisted_clone = unvisited.clone();
        println!("Edge: {}", edge);
        unvisted_clone.remove(*edge);
        let dist = tsp_brute_force_max_helper(edges, &mut unvisted_clone, *edge);
        max_dist = max_dist.max(dist);
    }

    max_dist
}

fn tsp_brute_force_max_helper(edges: &HashMap<usize, HashMap<usize, u32>>, unvisited: &mut BitSet, current: usize) -> u32 {
    if unvisited.is_empty() {
        return 0;
    }
    
    let neighbors = edges.get(&current).unwrap();
    let mut max_dist = u32::MIN;
    for neighbor in neighbors.keys() {
        if unvisited.contains(*neighbor) {
            unvisited.remove(*neighbor);
            let dist = neighbors.get(neighbor).unwrap() + tsp_brute_force_max_helper(edges, unvisited, *neighbor);
            max_dist = max_dist.max(dist);
            unvisited.insert(*neighbor);
        }
    }

    max_dist
}

#[test]
fn test_solve_day9_puzzle_part1() {
    assert!(solve_day9_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day9_puzzle_part2() {
    assert!(solve_day9_puzzle_part2().is_ok());
}