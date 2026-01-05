use common::error::PuzzleError;
use itertools::Itertools;

pub fn solve_day24_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day24.txt")?;
    let mut packages = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    packages.sort_unstable_by(|a, b| b.cmp(a));
    let mut min_package_count = usize::MAX;
    let mut min_quantum_entanglement = u64::MAX;
    for first_group_size in 1..packages.len() / 3 {
        let combinations = packages.iter().combinations(first_group_size);
        for combination in combinations {
            let first_group = combination.iter().map(|&&x| x).collect::<Vec<u32>>();
            let remaining_packages = packages
                .iter()
                .filter(|&&x| !first_group.contains(&x))
                .map(|&x| x)
                .collect::<Vec<u32>>();
            //println!("first_group: {:?}, remaining: {:?}", first_group, remaining_packages);
            let first_group_weight: u32 = first_group.iter().sum();
            let remaining_weight: u32 = remaining_packages.iter().sum();
            //println!("first_group_weight: {}, remaining_weight: {}, first_group_weight * 2 = {}", first_group_weight, remaining_weight, first_group_weight * 2);
            if first_group_weight * 2 != remaining_weight {
                continue;
            }

            let mut dp = vec![false; (first_group_weight + 1) as usize];
            dp[0] = true;

            for &pkg in &remaining_packages {
                for w in (pkg..=first_group_weight).rev() {
                    if dp[(w - pkg) as usize] {
                        dp[w as usize] = true;
                    }
                }
            }

            if dp[first_group_weight as usize] {
                let quantum_entanglement: u64 = first_group.iter().map(|&x| x as u64).product();
                if first_group_size < min_package_count ||
                   (first_group_size == min_package_count && quantum_entanglement < min_quantum_entanglement) {
                    min_package_count = first_group_size;
                    min_quantum_entanglement = quantum_entanglement;
                }
            }
        }
    }

    println!("Minimum Quantum Entanglement = {}", min_quantum_entanglement);

    Ok(())
}

pub fn solve_day24_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day24.txt")?;
    let mut packages = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    packages.sort_unstable_by(|a, b| b.cmp(a));
    let mut min_package_count = usize::MAX;
    let mut min_quantum_entanglement = u64::MAX;
    'group_size_loop:
    for first_group_size in 1..packages.len() / 4 {
        if first_group_size > min_package_count {
            break;
        }

        //let min_possible = packages.iter().rev().take(first_group_size).sum::<u32>();
        let mut combinations = packages
            .iter()
            .combinations(first_group_size)
            .map(|comb| comb.into_iter().map(|&x| x).collect::<Vec<u32>>())
            .map(|comb| {
                let weight: u32 = comb.iter().sum();
                (comb, weight)
            })
            .collect::<Vec<_>>();
        combinations.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        for (first_group, first_group_weight) in combinations {
            let remaining_packages = packages
                .iter()
                .filter(|&&x| !first_group.contains(&x)) // There are no duplicate weights in the input
                .map(|&x| x)
                .collect::<Vec<u32>>();
            //println!("first_group: {:?}, remaining: {:?}", first_group, remaining_packages);
            let remaining_weight: u32 = remaining_packages.iter().sum();
            let remaining_weight_target = first_group_weight * 3;
            if remaining_weight_target < remaining_weight {
                // Since combinations are sorted by weight descending, no need to check further
                continue 'group_size_loop;
            }

            //println!("first_group_weight: {}, remaining_weight: {}, first_group_weight * 2 = {}", first_group_weight, remaining_weight, first_group_weight * 2);
            if remaining_weight_target != remaining_weight {
                continue;
            }

            let mut dp = vec![false; first_group_weight as usize + 1];
            dp[0] = true;

            for &w in &remaining_packages {
                for s in (w as usize..=first_group_weight as usize).rev() {
                    dp[s] |= dp[s - w as usize];
                }
            }

            if !dp[first_group_weight as usize] {
                continue; // remaining packages cannot form one group
            }

            if dp[first_group_weight as usize] {
                let quantum_entanglement: u64 = first_group.iter().map(|&x| x as u64).product();
                if first_group_size < min_package_count ||
                   (first_group_size == min_package_count && quantum_entanglement < min_quantum_entanglement) {
                    min_package_count = first_group_size;
                    min_quantum_entanglement = quantum_entanglement;
                }
            }
        }
    }

    println!("Minimum Quantum Entanglement = {}", min_quantum_entanglement);

    Ok(())
}

#[test]
fn test_day24_part1() {
    solve_day24_puzzle_part1().unwrap();
}

#[test]
fn test_day24_part2() {
    solve_day24_puzzle_part2().unwrap();
}