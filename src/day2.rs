use std::fs;

pub fn solve1() -> i32 {
    let mut input = parse_input();
    input[1] = 12;
    input[2] = 2;
    process_program(&mut input).unwrap()
}

pub fn solve2() -> i32 {
    let original_input = parse_input();
    let mut result = 0i32;
    for i in 0..1000 {
        for j in 0..1000 {
            let mut input = original_input.clone();
            input[1] = i;
            input[2] = j;
            result = process_program(&mut input).unwrap_or_default();

            if result == 19690720 {
                println!("terms: {} {}", i, j);
                return 100 * i + j;
            }
        }
    }
    result
}

fn process_program(program: &mut Vec<i32>) -> Result<i32, &str> {
    let mut i = 0;
    while program[i] != 99 {
        let pos1 = program[i + 1] as usize;
        let pos2 = program[i + 2] as usize;
        let pos_result = program[i + 3] as usize;
        if pos1 >= program.len() || pos2 >= program.len() || pos_result >= program.len() {
            return Err("no valid program");
        }
        match program[i] {
            1 => program[pos_result] = program[pos1] + program[pos2],
            2 => program[pos_result] = program[pos1] * program[pos2],
            _ => panic!("Unknown op code"),
        }
        i += 4;
    }
    Ok(program[0])
}

fn parse_input() -> Vec<i32> {
    fs::read_to_string("./input/day2_1.txt")
        .unwrap()
        .lines()
        .flat_map(|line| line.split(',').map(|e| e.parse::<i32>().unwrap()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            process_program(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).unwrap(),
            30
        );
        assert_eq!(process_program(&mut vec![1, 0, 0, 0, 99]).unwrap(), 2);
        assert_eq!(process_program(&mut vec![2, 3, 0, 3, 99]).unwrap(), 2);
        assert_eq!(process_program(&mut vec![2, 4, 4, 5, 99, 0]).unwrap(), 2);
    }
}
