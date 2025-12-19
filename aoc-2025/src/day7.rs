use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy)]
enum TachyonTile {
    Empty,
    Source,
    Splitter,
    Beam
}

pub fn solve_day7_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day7.txt")?;
    let mut tachyon_manifold: Vec<Vec<TachyonTile>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<TachyonTile> = Vec::new();
        for ch in line.chars() {
            let tile = match ch {
                '.' => TachyonTile::Empty,
                'S' => TachyonTile::Source,
                '^' => TachyonTile::Splitter,
                _ => continue, // Beams should not be in the input
            };
            row.push(tile);
        }
        tachyon_manifold.push(row);
    }

    let mut split_count = 0;
    for r in 0..tachyon_manifold.len() {
        if r == 0 {
            continue;   
        }

        let row = &tachyon_manifold[r];
        let mut new_row = row.clone();
        for (t, tile) in row.iter().enumerate() {
            if !matches!(tile, TachyonTile::Empty) {
                continue;
            }

            let above_tile = tachyon_manifold[r - 1][t];
            match above_tile {
                TachyonTile::Empty => continue,
                TachyonTile::Source => {
                    new_row[t] = TachyonTile::Beam;
                },
                TachyonTile::Splitter => {
                    let above_above_tile = tachyon_manifold[r - 2][t];
                    if matches!(above_above_tile, TachyonTile::Beam) {
                        if t > 0 {
                            new_row[t - 1] = TachyonTile::Beam;
                        }
                        if t + 1 < row.len() {
                            new_row[t + 1] = TachyonTile::Beam;
                        }

                        split_count += 1;
                    }
                },
                TachyonTile::Beam => {
                    new_row[t] = TachyonTile::Beam;
                },
            }
        }

        tachyon_manifold[r] = new_row;
    }

    println!("Split count: {}", split_count);
    
    Ok(())
}

pub fn solve_day7_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day7.txt")?;
    let mut tachyon_manifold: Vec<Vec<TachyonTile>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<TachyonTile> = Vec::new();
        for ch in line.chars() {
            let tile = match ch {
                '.' => TachyonTile::Empty,
                'S' => TachyonTile::Source,
                '^' => TachyonTile::Splitter,
                _ => continue, // Beams should not be in the input
            };
            row.push(tile);
        }
        tachyon_manifold.push(row);
    }

    let mut new_above_row = tachyon_manifold[1].clone();
    for (t, tile) in tachyon_manifold[1].iter().enumerate() {
        if matches!(tile, TachyonTile::Empty) {
            let above_tile = tachyon_manifold[0][t];
            if matches!(above_tile, TachyonTile::Source) {
                new_above_row[t] = TachyonTile::Beam;
            }
        }
    }

    tachyon_manifold[1] = new_above_row;

    let above_above_row = &tachyon_manifold[0];
    let above_row = &tachyon_manifold[1];
    println!("Above above row: {:?}", above_above_row);
    println!("Above row: {:?}", above_row);
    let split_count = day7_helper(&tachyon_manifold, above_row, above_above_row, 2, 0);

    println!("Split count: {}", split_count);
    
    Ok(())
}

fn day7_helper(tachyon_manifold: &Vec<Vec<TachyonTile>>, above_row: &Vec<TachyonTile>, above_above_row: &Vec<TachyonTile>, r: usize, t_start: usize) -> usize {
    if r == tachyon_manifold.len() {
        //println!("Reached the bottom of the manifold");
        // At this point we are technically below the last row, so above_row is the last row
        return if above_row.iter().any(|&tile| matches!(tile, TachyonTile::Beam)) {
            1
        }
        else {
            0
        }
    }

    let mut splits = 0;
    let row = &tachyon_manifold[r];
    //println!("Processing row {}: {:?}", r, row);
    let mut new_row = row.clone();
    for (t, tile) in row.iter().enumerate() {
        if t < t_start {
            continue;
        }

        match tile {
            TachyonTile::Empty => {
                let above_tile = above_row[t];
                match above_tile {
                    TachyonTile::Empty => continue,
                    TachyonTile::Source => {
                        new_row[t] = TachyonTile::Beam;
                        let new_splits = day7_helper(tachyon_manifold, &new_row, above_row, r + 1, t);
                        //println!("New splits from source: {}", new_splits);
                        splits += new_splits;
                        new_row[t] = TachyonTile::Empty; // backtrack
                    },
                    TachyonTile::Splitter => {
                        let above_above_tile = above_above_row[t];
                        if matches!(above_above_tile, TachyonTile::Beam) {
                            if t > 0 {
                                new_row[t - 1] = TachyonTile::Beam;
                                let new_splits = day7_helper(tachyon_manifold, &new_row, above_row, r + 1, t - 1);
                                //println!("New splits from left branch: {}", new_splits);
                                splits += new_splits;
                                new_row[t - 1] = TachyonTile::Empty; // backtrack
                            }
                            if t + 1 < row.len() {
                                new_row[t + 1] = TachyonTile::Beam;
                                let new_splits = day7_helper(tachyon_manifold, &new_row, above_row, r + 1, t + 1);
                                //println!("New splits from right branch: {}", new_splits);
                                splits += new_splits;
                                new_row[t + 1] = TachyonTile::Empty; // backtrack
                            }
                        }
                    },
                    TachyonTile::Beam => {
                        new_row[t] = TachyonTile::Beam;
                        let new_splits = day7_helper(tachyon_manifold, &new_row, above_row, r + 1, t);
                        //println!("New splits from beam: {}", new_splits);
                        splits += new_splits;
                        new_row[t] = TachyonTile::Empty; // backtrack
                    },
                }
            },
            TachyonTile::Splitter => {
                let above_tile = above_row[t];
                if matches!(above_tile, TachyonTile::Beam) {
                    let new_splits = day7_helper(tachyon_manifold, &new_row, above_row, r + 1, t);
                    //println!("New splits from splitter: {}", new_splits);
                    splits += new_splits;
                }
            }
            _ => continue,
        }
    }

    splits
}

type NodeId = usize;

#[derive(Debug)]
struct TachyonNode {
    parent: Option<NodeId>,
    left_child: Option<NodeId>,
    direct_child: Option<NodeId>,
    right_child: Option<NodeId>,
    tile: TachyonTile
}

impl TachyonNode {
    fn new() -> Self {
        TachyonNode {
            parent: None,
            left_child: None,
            direct_child: None,
            right_child: None,
            tile: TachyonTile::Empty,
        }
    }

    fn is_leaf(&self) -> bool {
        self.left_child.is_none() && self.direct_child.is_none() && self.right_child.is_none()
    }
}

#[derive(Debug)]
struct TachyonTree {
    nodes: Vec<TachyonNode>,
}

impl TachyonTree {
    fn new(tachyon_manifold: &Vec<Vec<TachyonTile>>) -> Self {
        let height = tachyon_manifold.len();
        let width = tachyon_manifold[0].len();
        let mut nodes = Vec::with_capacity(width * height);

        for _ in 0..(width * height) {
            nodes.push(TachyonNode::new());
        }

        for (r, row) in tachyon_manifold.iter().enumerate() {
            for (t, &tile) in row.iter().enumerate() {
                let node_id = Self::id(r, t, width);
                nodes[node_id].tile = tile;

                if r + 1 < height {
                    let down_id = Self::id(r + 1, t, width);
                    nodes[node_id].direct_child = Some(down_id);
                    nodes[down_id].parent = Some(node_id);
                }

                if t > 0 && r + 1 < height {
                    let left_down_id = Self::id(r + 1, t - 1, width);
                    nodes[node_id].left_child = Some(left_down_id);
                }

                if t + 1 < width && r + 1 < height {
                    let right_down_id = Self::id(r + 1, t + 1, width);
                    nodes[node_id].right_child = Some(right_down_id);
                }
            }
        }

        TachyonTree {
            nodes,
        }
    }

    fn get_source_node(&self) -> Option<NodeId> {
        for (id, node) in self.nodes.iter().enumerate() {
            if matches!(node.tile, TachyonTile::Source) {
                return Some(id);
            }
        }

        None
    }

    // fn get_left_child(&self, node: &TachyonNode) -> Option<&TachyonNode> {
    //     match node.left_child {
    //         Some(id) => Some(&self.nodes[id]),
    //         None => None,
    //     }
    // }

    // fn get_direct_child(&self, node: &TachyonNode) -> Option<&TachyonNode> {
    //     match node.direct_child {
    //         Some(id) => Some(&self.nodes[id]),
    //         None => None,
    //     }
    // }

    // fn get_right_child(&self, node: &TachyonNode) -> Option<&TachyonNode> {
    //     match node.right_child {
    //         Some(id) => Some(&self.nodes[id]),
    //         None => None,
    //     }
    // }

    fn id(row: usize, col: usize, width: usize) -> usize {
        row * width + col
    }
}

pub fn solve_day7_puzzle_part2_v2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day7.txt")?;
    let mut tachyon_manifold: Vec<Vec<TachyonTile>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<TachyonTile> = Vec::new();
        for ch in line.chars() {
            let tile = match ch {
                '.' => TachyonTile::Empty,
                'S' => TachyonTile::Source,
                '^' => TachyonTile::Splitter,
                _ => continue, // Beams should not be in the input
            };
            row.push(tile);
        }
        tachyon_manifold.push(row);
    }

    let tachyon_tree = TachyonTree::new(&tachyon_manifold);
    let source_node = tachyon_tree.get_source_node().unwrap();
    println!("Source node ID: {}", source_node);
    //let has_leaf_nodes = tachyon_tree.nodes.iter().any(|node| node.is_leaf());
    //println!("Tree has leaf nodes: {}", has_leaf_nodes);
    let mut memo = HashMap::new();
    let split_count = day7_helper_v2(&tachyon_tree, source_node, &mut memo);

    println!("Split count: {}", split_count);
    
    Ok(())
}

fn day7_helper_v2(tree: &TachyonTree, node_id: NodeId, memo: &mut HashMap<NodeId, usize>) -> usize {
    if let Some(&cached) = memo.get(&node_id) {
        return cached;
    }

    let node = &tree.nodes[node_id];

    let result = if node.is_leaf() {
        1
    }
    else {
        match node.tile {
            TachyonTile::Empty | TachyonTile::Source => {
                node.direct_child
                    .map(|c| day7_helper_v2(tree, c, memo))
                    .unwrap_or(0)
            }

            TachyonTile::Splitter => {
                let left = node.left_child
                    .map(|c| day7_helper_v2(tree, c, memo))
                    .unwrap_or(0);

                let right = node.right_child
                    .map(|c| day7_helper_v2(tree, c, memo))
                    .unwrap_or(0);

                left + right
            }

            TachyonTile::Beam => unreachable!(),
        }
    };

    memo.insert(node_id, result);
    result
}

#[test]
fn test_solve_day7_puzzle_part1() {
    assert!(solve_day7_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day7_puzzle_part2_v2() {
    assert!(solve_day7_puzzle_part2_v2().is_ok());
}