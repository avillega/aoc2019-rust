use crate::computer::Interpreter;
use permutohedron::Heap;
use std::fs;

pub fn solve2() {
    let original_program = parse_input();
    let mut phases = vec![5, 6, 7, 8, 9];
    let phases = Heap::new(&mut phases);

    println!(
        "{}",
        phases
            .map(|permutation| run_loop_for_permutation(&original_program, permutation))
            .max()
            .unwrap()
    );
}
pub fn solve1() {
    let original_program = parse_input();
    let mut data = vec![0, 1, 2, 3, 4];
    let phases = Heap::new(&mut data);

    println!(
        "{}",
        phases
            .map(|permutation| run_for_permutation(&original_program, permutation))
            .max()
            .unwrap()
    );
}

fn run_for_permutation(original_program: &Vec<i64>, phases: Vec<i64>) -> i64 {
    let mut input = 0;
    for phase in phases {
        input = run_step(original_program.clone(), phase, input);
    }
    input
}

fn run_loop_for_permutation(original_program: &Vec<i64>, phases: Vec<i64>) -> i64 {
    let mut interpreters: Vec<Interpreter> = phases
        .iter()
        .map(|phase| {
            let mut interpreter = Interpreter::new(original_program.clone()).halt_on_output();
            interpreter.add_input(*phase);
            interpreter
        })
        .collect();

    let mut output = 0;
    for index in 0.. {
        let index = index % 5;
        interpreters[index].add_input(output);
        interpreters[index].execute_program();
        match interpreters[index].output.pop_back() {
            Some(o) => output = o,
            None => break,
        }
    }
    output
}

fn run_step(program: Vec<i64>, phase: i64, input: i64) -> i64 {
    let mut interpreter = Interpreter::new(program);
    interpreter.add_input(phase);
    interpreter.add_input(input);
    interpreter.execute_program();
    *interpreter.get_output().last().expect("output expected")
}

fn parse_input() -> Vec<i64> {
    fs::read_to_string("./input/day7_1.txt")
        .unwrap()
        .lines()
        .flat_map(|line| line.split(',').map(|s| s.parse::<i64>().unwrap()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_permutation() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];

        assert_eq!(
            139629729,
            run_loop_for_permutation(&program, vec![9, 8, 7, 6, 5])
        );

        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];

        assert_eq!(
            18216,
            run_loop_for_permutation(&program, vec![9, 7, 8, 5, 6])
        );
    }
}
