use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn solve_day14_puzzle_part1() -> Result<(), PuzzleError> {
    const MAX_TIME: i32 = 2503;

    let input = std::fs::read_to_string("inputs/day14.txt")?;
    let lines = input.lines();
    let mut max_distance = 0;
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let speed = parts[3].parse::<i32>().unwrap();
        let fly_time = parts[6].parse::<i32>().unwrap();
        let rest_time = parts[13].parse::<i32>().unwrap();
        let mut time_remaining = MAX_TIME;
        let mut distance = 0;
        while time_remaining > 0 {
            if time_remaining >= fly_time {
                distance += speed * fly_time;
                time_remaining -= fly_time + rest_time;
            }
            else {
                distance += speed * time_remaining;
                break;
            }
        }

        if distance > max_distance {
            max_distance = distance;
        }
    }

    println!("Distance of winning reindeer: {}", max_distance);
    
    Ok(())
}

#[derive(Debug)]
struct Reigndeer {
    speed: i32,
    fly_time: i32,
    rest_time: i32,
    distance: i32,
    time_remaining_in_state: i32,
    flying: bool,
    points: i32,
}

impl Reigndeer {
    fn new(speed: i32, fly_time: i32, rest_time: i32) -> Self {
        Self {
            speed,
            fly_time,
            rest_time,
            distance: 0,
            time_remaining_in_state: fly_time,
            flying: true,
            points: 0,
        }
    }

    fn tick(&mut self) {
        if self.flying {
            self.distance += self.speed;
        }
        self.time_remaining_in_state -= 1;
        if self.time_remaining_in_state == 0 {
            if self.flying {
                self.flying = false;
                self.time_remaining_in_state = self.rest_time;
            }
            else {
                self.flying = true;
                self.time_remaining_in_state = self.fly_time;
            }
        }
    }
}

pub fn solve_day14_puzzle_part2() -> Result<(), PuzzleError> {
    const MAX_TIME: i32 = 2503;

    let input = std::fs::read_to_string("inputs/day14.txt")?;
    let lines = input.lines();
    let mut reindeers = Vec::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let speed = parts[3].parse::<i32>().unwrap();
        let fly_time = parts[6].parse::<i32>().unwrap();
        let rest_time = parts[13].parse::<i32>().unwrap();
        let reigndeer = Reigndeer::new(speed, fly_time, rest_time);
        reindeers.push(reigndeer);
    }

    for _ in 0..MAX_TIME {
        for reindeer in reindeers.iter_mut() {
            reindeer.tick();
        }
        
        let max_distance = reindeers.iter().map(|r| r.distance).max().unwrap();
        for reindeer in reindeers.iter_mut() {
            if reindeer.distance == max_distance {
                reindeer.points += 1;
            }
        }
    }

    let max_points = reindeers.iter().map(|r| r.points).max().unwrap();
    println!("Points of winning reindeer: {}", max_points);
    
    Ok(())
}

#[test]
fn test_day14_part1() {
    assert!(solve_day14_puzzle_part1().is_ok());
}

#[test]
fn test_day14_part2() {
    assert!(solve_day14_puzzle_part2().is_ok());
}