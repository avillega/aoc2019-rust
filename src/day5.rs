use crate::computer::Interpreter;
use std::fs;

pub fn solve1() {
    let program = parse_input();
    let mut interpreter = Interpreter::new(program);
    interpreter.add_input(1);
    interpreter.execute_program();
    println!("{:?}", interpreter.get_output());
}

pub fn solve2() {
    let program = parse_input();
    let mut interpreter = Interpreter::new(program);
    interpreter.add_input(5);
    interpreter.execute_program();
    println!("{:?}", interpreter.get_output());
}

fn parse_input() -> Vec<i64> {
    fs::read_to_string("./input/day5_1.txt")
        .unwrap()
        .lines()
        .flat_map(|line| line.split(','))
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}
