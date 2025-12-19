use std::collections::{HashMap, HashSet};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn new(coords: &str) -> Self {
        let parts: Vec<i64> = coords
            .split(',')
            .map(|part| part.trim().parse::<i64>().unwrap())
            .collect();
        JunctionBox {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        }
    }

    fn get_distance(&self, other: &JunctionBox) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}

impl std::fmt::Display for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct DisjointSetUnion {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(size: usize) -> Self {
        DisjointSetUnion {
            parents: (0..size).collect(),
            sizes: vec![1; size],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parents[v] == v {
            v
        }
        else{
            self.parents[v] = self.find(self.parents[v]);
            self.parents[v]
        }
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a != root_b {
            if self.sizes[root_a] < self.sizes[root_b] {
                self.parents[root_a] = root_b;
                self.sizes[root_b] += self.sizes[root_a];
            }
            else{
                self.parents[root_b] = root_a;
                self.sizes[root_a] += self.sizes[root_b];
            }
        }
    }
}

pub fn solve_day8_puzzle_part1() -> Result<(), PuzzleError> {
    const NUM_PAIRS: usize = 1000;

    let input = std::fs::read_to_string("inputs/day8.txt")?;
    let lines = input.lines();
    let junction_boxes: Vec<JunctionBox> = lines
        .map(|line| JunctionBox::new(line))
        .collect();

    let mut distances = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let distance = junction_boxes[i].get_distance(&junction_boxes[j]);
            distances.push((distance, i, j));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut circuits = Vec::new();
    let mut circuit_map = HashMap::new();
    for (distance, i, j) in distances.iter().take(NUM_PAIRS) {
        println!(
            "Circuit between JunctionBox ({}) {} and JunctionBox ({}) {}: Distance = {:.2}",
            junction_boxes[*i], i, junction_boxes[*j], j, distance
        );

        let mut insert_queue = None;
        if let Some(circuit_idx) = circuit_map.get(i) {
            if let Some(other_circuit_idx) = circuit_map.get(j) {
                if circuit_idx != other_circuit_idx {
                    println!("Merging circuits {} and {} due to JunctionBoxes {} and {}", circuit_idx, other_circuit_idx, i, j);
                    let other_circuit: HashSet<usize> = {
                        let temp: &mut HashSet<usize> = &mut circuits[*other_circuit_idx];
                        temp.drain().collect()
                    };
                    let circuit: &mut HashSet<usize> = &mut circuits[*circuit_idx];
                    for box_idx in other_circuit {
                        circuit.insert(box_idx);
                        insert_queue = Some((box_idx, *circuit_idx));
                    }
                }
            }
            else{
                println!("Found existing circuit for JunctionBox {}", i);
                let circuit: &mut HashSet<usize> = &mut circuits[*circuit_idx];
                circuit.insert(*j);
                circuit_map.insert(*j, *circuit_idx);
            }
        }
        else if let Some(circuit_idx) = circuit_map.get(j) {
            if let Some(other_circuit_idx) = circuit_map.get(i) {
                if circuit_idx != other_circuit_idx {
                    println!("Merging circuits {} and {} due to JunctionBoxes {} and {}", other_circuit_idx, circuit_idx, i, j);
                    let other_circuit: HashSet<usize> = {
                        let temp: &mut HashSet<usize> = &mut circuits[*other_circuit_idx];
                        temp.drain().collect()
                    };
                    let circuit: &mut HashSet<usize> = &mut circuits[*circuit_idx];
                    for box_idx in other_circuit {
                        circuit.insert(box_idx);
                        insert_queue = Some((box_idx, *circuit_idx));
                    }
                }
            }
            else{
                println!("Found existing circuit for JunctionBox {}", j);
                let circuit: &mut HashSet<usize> = &mut circuits[*circuit_idx];
                circuit.insert(*i);
                circuit_map.insert(*i, *circuit_idx);
            }
        }
        else{
            println!("Creating new circuit for JunctionBoxes {} and {}", i, j);
            let mut circuit = HashSet::new();
            circuit.insert(*i);
            circuit.insert(*j);
            let circuit_idx = circuits.len();
            circuits.push(circuit);
            circuit_map.insert(*i, circuit_idx);
            circuit_map.insert(*j, circuit_idx);
        }

        if let Some((box_idx, circuit_idx)) = insert_queue {
            circuit_map.insert(box_idx, circuit_idx);
        }
    }

    let mut circuit_sizes: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
    circuit_sizes.sort_by(|a, b| b.cmp(a));
    println!("Circuit sizes: {:?}", circuit_sizes);
    let mut largest_size = 1;
    for size in &circuit_sizes[..3] {
        largest_size *= size;
    }

    println!("Product of sizes of the three largest circuits: {}", largest_size);

    Ok(())
}

pub fn solve_day8_puzzle_part1_v2() -> Result<(), PuzzleError> {
    const NUM_PAIRS: usize = 1000;

    let input = std::fs::read_to_string("inputs/day8.txt")?;
    let lines = input.lines();
    let junction_boxes: Vec<JunctionBox> = lines
        .map(|line| JunctionBox::new(line))
        .collect();

    let mut distances = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let distance = junction_boxes[i].get_distance(&junction_boxes[j]);
            distances.push((distance, i, j));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut dsu = DisjointSetUnion::new(junction_boxes.len());
    for (distance, i, j) in distances.iter().take(NUM_PAIRS) {
        dsu.union(*i, *j);
        println!(
            "Circuit between JunctionBox ({}) {} and JunctionBox ({}) {}: Distance = {:.2}",
            junction_boxes[*i], i, junction_boxes[*j], j, distance
        );
    }

    let circuit_sizes = dsu.sizes;
    let mut sorted_sizes = circuit_sizes.clone();
    sorted_sizes.sort_by(|a, b| b.cmp(a));
    println!("Circuit sizes: {:?}", sorted_sizes);
    let mut largest_size = 1;
    for size in &sorted_sizes[..3] {
        largest_size *= size;
    }

    println!("Product of sizes of the three largest circuits: {}", largest_size);

    Ok(())
}

pub fn solve_day8_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day8.txt")?;
    let lines = input.lines();
    let junction_boxes: Vec<JunctionBox> = lines
        .map(|line| JunctionBox::new(line))
        .collect();

    let mut distances = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let distance = junction_boxes[i].get_distance(&junction_boxes[j]);
            distances.push((distance, i, j));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut dsu = DisjointSetUnion::new(junction_boxes.len());
    for (distance, i, j) in distances.iter() {
        dsu.union(*i, *j);
        println!(
            "Circuit between JunctionBox ({}) {} and JunctionBox ({}) {}: Distance = {:.2}",
            junction_boxes[*i], i, junction_boxes[*j], j, distance
        );

        //println!("Parents: {:?}", dsu.parents);
        //let first = dsu.parents[0];
        //let all_connected = dsu.parents.iter().all(|&parent| parent == first);
        let all_connected = dsu.sizes.iter().any(|&size| size == junction_boxes.len());
        if all_connected {
            let last_connected_1 = &junction_boxes[*i];
            let last_connected_2 = &junction_boxes[*j];
            let x_product = last_connected_1.x * last_connected_2.x;
            println!(
                "All junction boxes are now connected. Last connected boxes: {} and {}. Product of their x-coordinates: {}",
                last_connected_1, last_connected_2, x_product
            );
            
            break;
        }
    }

    Ok(())
}

#[test]
fn test_day8_part1() {
    assert!(solve_day8_puzzle_part1().is_ok());
}

#[test]
fn test_day8_part1_v2() {
    assert!(solve_day8_puzzle_part1_v2().is_ok());
}

#[test]
fn test_day8_part2() {
    assert!(solve_day8_puzzle_part2().is_ok());
}