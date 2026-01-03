use std::collections::HashMap;

use common::error::PuzzleError;

type ArcRefCell<T> = std::sync::Arc<std::cell::RefCell<T>>;

#[derive(Debug)]
struct Wire {
    value: ArcRefCell<u16>,
    initialized: bool,
}

impl Wire {
    fn new() -> Self {
        Wire {
            value: ArcRefCell::new(0.into()),
            initialized: false,
        }
    }

    fn set_value(&mut self, val: u16) {
        *self.value.borrow_mut() = val;
        self.initialized = true;
    }

    fn get_value(&self) -> u16 {
        *self.value.borrow()
    }

    fn reset(&mut self) {
        *self.value.borrow_mut() = 0;
        self.initialized = false;
    }
}

#[derive(Debug)]
enum Operand<'a> {
    Value(u16),
    Wire(&'a str),
}

#[derive(Debug)]
enum Operation<'a> {
    Assignment(Operand<'a>, &'a str),       // value/wire, output wire
    And(Operand<'a>, Operand<'a>, &'a str), // left value/wire, right value/wire, output wire
    Or(Operand<'a>, Operand<'a>, &'a str),  // left value/wire, right value/wire, output wire
    LShift(Operand<'a>, u16, &'a str),      // input value/wire, shift amount, output wire
    RShift(Operand<'a>, u16, &'a str),      // input value/wire, shift amount, output wire
    Not(Operand<'a>, &'a str),              // input value/wire, output wire
}

pub fn solve_day7_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day7.txt")?;
    let lines = input.lines();
    let mut wire_map = HashMap::new();
    let mut operations = Vec::new();
    for line in lines {
        println!("Processing line: {}", line);
        let parts = line.split(' ').collect::<Vec<&str>>();
        if parts.len() == 3 {
            // Assignment
            let value = parse_operand_and_ensure_wire_exists(parts[0], &mut wire_map);
            let output_wire = parts[2];
            let assignment = Operation::Assignment(value, output_wire);
            ensure_wire_exists(&mut wire_map, output_wire);
            operations.push((assignment, false));
        }
        else if parts.len() == 4 {
            // NOT operation
            let input_wire = parse_operand_and_ensure_wire_exists(parts[1], &mut wire_map);
            let output_wire = parts[3];
            let not_op = Operation::Not(input_wire, output_wire);
            ensure_wire_exists(&mut wire_map, output_wire);

            operations.push((not_op, false));
        }
        else if parts.len() == 5 {
            let input_wire = parse_operand_and_ensure_wire_exists(parts[0], &mut wire_map);
            let output_wire = parts[4];
            ensure_wire_exists(&mut wire_map, output_wire);
            match parts[1] {
                "AND" => {
                    let right_wire = parse_operand_and_ensure_wire_exists(parts[2], &mut wire_map);
                    let and_op = Operation::And(input_wire, right_wire, output_wire);
                    operations.push((and_op, false));
                }
                "OR" => {
                    let right_wire = parse_operand_and_ensure_wire_exists(parts[2], &mut wire_map);
                    let or_op = Operation::Or(input_wire, right_wire, output_wire);
                    operations.push((or_op, false));
                }
                "LSHIFT" => {
                    let shift_amount: u16 = parts[2].parse().unwrap();
                    let lshift_op = Operation::LShift(input_wire, shift_amount, output_wire);
                    operations.push((lshift_op, false));
                }
                "RSHIFT" => {
                    let shift_amount: u16 = parts[2].parse().unwrap();
                    let rshift_op = Operation::RShift(input_wire, shift_amount, output_wire);
                    operations.push((rshift_op, false));
                }
                _ => panic!("Unknown operation"),
            };
        }
    }

    propagate_signal_changes(&mut wire_map, &mut operations);

    //println!("Wire map: {:#?}", wire_map);
    let a_wire = wire_map.get("a").unwrap();
    println!("Value on wire 'a': {}", a_wire.get_value());

    Ok(())
}

pub fn solve_day7_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day7.txt")?;
    let lines = input.lines();
    let mut wire_map = HashMap::new();
    let mut operations = Vec::new();
    for line in lines {
        println!("Processing line: {}", line);
        let parts = line.split(' ').collect::<Vec<&str>>();
        if parts.len() == 3 {
            // Assignment
            let value = parse_operand_and_ensure_wire_exists(parts[0], &mut wire_map);
            let output_wire = parts[2];
            let assignment = Operation::Assignment(value, output_wire);
            ensure_wire_exists(&mut wire_map, output_wire);
            operations.push((assignment, false));
        }
        else if parts.len() == 4 {
            // NOT operation
            let input_wire = parse_operand_and_ensure_wire_exists(parts[1], &mut wire_map);
            let output_wire = parts[3];
            let not_op = Operation::Not(input_wire, output_wire);
            ensure_wire_exists(&mut wire_map, output_wire);

            operations.push((not_op, false));
        }
        else if parts.len() == 5 {
            let input_wire = parse_operand_and_ensure_wire_exists(parts[0], &mut wire_map);
            let output_wire = parts[4];
            ensure_wire_exists(&mut wire_map, output_wire);
            match parts[1] {
                "AND" => {
                    let right_wire = parse_operand_and_ensure_wire_exists(parts[2], &mut wire_map);
                    let and_op = Operation::And(input_wire, right_wire, output_wire);
                    operations.push((and_op, false));
                }
                "OR" => {
                    let right_wire = parse_operand_and_ensure_wire_exists(parts[2], &mut wire_map);
                    let or_op = Operation::Or(input_wire, right_wire, output_wire);
                    operations.push((or_op, false));
                }
                "LSHIFT" => {
                    let shift_amount: u16 = parts[2].parse().unwrap();
                    let lshift_op = Operation::LShift(input_wire, shift_amount, output_wire);
                    operations.push((lshift_op, false));
                }
                "RSHIFT" => {
                    let shift_amount: u16 = parts[2].parse().unwrap();
                    let rshift_op = Operation::RShift(input_wire, shift_amount, output_wire);
                    operations.push((rshift_op, false));
                }
                _ => panic!("Unknown operation"),
            };
        }
    }

    let operator_len = operations.len();
    propagate_signal_changes(&mut wire_map, &mut operations);

    //println!("Wire map: {:#?}", wire_map);
    let a_wire = wire_map.get("a").unwrap();
    let a_value = a_wire.get_value();
    for (id, wire) in wire_map.iter_mut() {
        if *id == "b" {
            wire.set_value(a_value);
        }
        else {
            wire.reset();
        }
    }

    for i in 0..operator_len {
        let (op, evaluated) = &mut operations[i];
        match op {
            Operation::Assignment(_, output_wire) if *output_wire == "b" => {
                *evaluated = true;
            }
            _ => {
                *evaluated = false;
            }
        }
    }

    propagate_signal_changes(&mut wire_map, &mut operations);
    let a_wire = wire_map.get("a").unwrap();
    println!("Value on wire 'a' after overriding 'b': {}", a_wire.get_value());

    Ok(())
}

fn ensure_wire_exists<'a>(wire_map: &mut HashMap<&'a str, Wire>, wire_name: &'a str) {
    match wire_map.get(wire_name) {
        None => {
            wire_map.insert(wire_name, Wire::new());
        }
        _ => {}
    }
}

fn parse_operand_and_ensure_wire_exists<'a>(
    token: &'a str,
    wire_map: &mut HashMap<&'a str, Wire>,
) -> Operand<'a> {
    match token.parse::<u16>() {
        Ok(v) => Operand::Value(v),
        Err(_) => {
            ensure_wire_exists(wire_map, token);
            Operand::Wire(token)
        }
    }
}

fn propagate_signal_changes<'a>(
    wire_map: &mut HashMap<&'a str, Wire>,
    operations: &mut Vec<(Operation<'a>, bool)>,
) {
    // Evaluate all operations till all have been evaluated
    let mut loop_iter = 1;
    loop {
        println!("Loop iteration {}", loop_iter);
        loop_iter += 1;

        let mut all_evaluated = true;
        let mut operations_evaluated = 0;
        for i in 0..operations.len() {
            let (operation, evaluated) = &mut operations[i];
            if *evaluated {
                operations_evaluated += 1;
                continue;
            }

            match operation {
                Operation::Assignment(operand, output) => {
                    let result = match operand {
                        Operand::Value(v) => {
                            println!("{} -> {}", v, output);
                            *v
                        }
                        Operand::Wire(wire_name) => {
                            let wire = wire_map.get(wire_name).unwrap();
                            if wire.initialized {
                                println!("{} -> {}", wire_name, output);
                                wire.get_value()
                            }
                            else {
                                all_evaluated = false;
                                continue;
                            }
                        }
                    };

                    let output_wire = wire_map.get_mut(output).unwrap();
                    output_wire.set_value(result);
                    *evaluated = true;
                    operations_evaluated += 1;
                }
                Operation::And(left, right, output) => {
                    let (left_value, right_value) = match (left, right) {
                        (Operand::Value(left_value), Operand::Value(right_value)) => {
                            (*left_value, *right_value)
                        }
                        (Operand::Value(left_value), Operand::Wire(right_wire)) => {
                            let right_wire = wire_map.get(right_wire).unwrap();
                            if right_wire.initialized {
                                (*left_value, right_wire.get_value())
                            }
                            else {
                                all_evaluated = false;
                                continue;
                            }
                        }
                        (Operand::Wire(left_wire), Operand::Value(right_value)) => {
                            let left_wire = wire_map.get(left_wire).unwrap();
                            if left_wire.initialized {
                                (left_wire.get_value(), *right_value)
                            }
                            else {
                                all_evaluated = false;
                                continue;
                            }
                        }
                        (Operand::Wire(left_wire), Operand::Wire(right_wire)) => {
                            let left_wire = wire_map.get(left_wire).unwrap();
                            let right_wire = wire_map.get(right_wire).unwrap();
                            if left_wire.initialized && right_wire.initialized {
                                (left_wire.get_value(), right_wire.get_value())
                            }
                            else {
                                all_evaluated = false;
                                continue;
                            }
                        }
                    };

                    let result = left_value & right_value;
                    let output_wire = wire_map.get_mut(output).unwrap();
                    output_wire.set_value(result);
                    *evaluated = true;
                    operations_evaluated += 1;
                }
                Operation::Or(left, right, output) => {
                    let (left_value, right_value) = match (left, right) {
                        (Operand::Value(left_value), Operand::Value(right_value)) => {
                            (*left_value, *right_value)
                        }
                        (Operand::Value(left_value), Operand::Wire(right_wire)) => {
                            let right_wire = wire_map.get(right_wire).unwrap();
                            if right_wire.initialized {
                                (*left_value, right_wire.get_value())
                            }
                            else {
                                all_evaluated = false;
                                continue;
                            }
                        }
                        (Operand::Wire(left_wire), Operand::Value(right_value)) => {
                            let left_wire = wire_map.get(left_wire).unwrap();
                            if left_wire.initialized {
                                (left_wire.get_value(), *right_value)
                            }
                            else {
                                all_evaluated = false;
                                continue;
                            }
                        }
                        (Operand::Wire(left_wire), Operand::Wire(right_wire)) => {
                            let left_wire = wire_map.get(left_wire).unwrap();
                            let right_wire = wire_map.get(right_wire).unwrap();
                            if left_wire.initialized && right_wire.initialized {
                                (left_wire.get_value(), right_wire.get_value())
                            }
                            else {
                                all_evaluated = false;
                                continue;
                            }
                        }
                    };

                    let result = left_value | right_value;
                    let output_wire = wire_map.get_mut(output).unwrap();
                    output_wire.set_value(result);
                    *evaluated = true;
                    operations_evaluated += 1;
                }
                Operation::LShift(left, shift_amount, output) => {
                    let left_value = match left {
                        Operand::Value(value) => *value,
                        Operand::Wire(wire_name) => {
                            let wire = wire_map.get(wire_name).unwrap();
                            if wire.initialized {
                                wire.get_value()
                            }
                            else {
                                all_evaluated = false;
                                continue;
                            }
                        }
                    };

                    let result = left_value << *shift_amount;
                    let output_wire = wire_map.get_mut(output).unwrap();
                    output_wire.set_value(result);
                    *evaluated = true;
                    operations_evaluated += 1;
                }
                Operation::RShift(left, shift_amount, output) => {
                    let left_value = match left {
                        Operand::Value(value) => *value,
                        Operand::Wire(wire_name) => {
                            let wire = wire_map.get(wire_name).unwrap();
                            if wire.initialized {
                                wire.get_value()
                            }
                            else {
                                all_evaluated = false;
                                continue;
                            }
                        }
                    };

                    let result = left_value >> *shift_amount;
                    let output_wire = wire_map.get_mut(output).unwrap();
                    output_wire.set_value(result);
                    *evaluated = true;
                    operations_evaluated += 1;
                }
                Operation::Not(input, output) => {
                    let input_value = match input {
                        Operand::Value(value) => *value,
                        Operand::Wire(wire_name) => {
                            let wire = wire_map.get(wire_name).unwrap();
                            if wire.initialized {
                                wire.get_value()
                            }
                            else {
                                all_evaluated = false;
                                continue;
                            }
                        }
                    };

                    let result = !input_value;
                    let output_wire = wire_map.get_mut(output).unwrap();
                    output_wire.set_value(result);
                    *evaluated = true;
                    operations_evaluated += 1;
                }
            }
        }

        println!(
            "Evaluated {}/{} operations",
            operations_evaluated,
            operations.len()
        );
        if all_evaluated {
            break;
        }
    }
}

#[test]
fn test_solve_day7_puzzle_part1() {
    assert!(solve_day7_puzzle_part1().is_ok());
}

#[test]
fn test_solve_day7_puzzle_part2() {
    assert!(solve_day7_puzzle_part2().is_ok());
}