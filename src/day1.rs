use std::fs;

pub fn solve1() -> i32 {
    read_input().iter().map(|mass| calc(*mass)).sum()
}

pub fn solve2() -> i32 {
    read_input().iter().map(|mass| calc2(*mass)).sum()
}

fn read_input() -> Vec<i32> {
    fs::read_to_string("./input/day1_1.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn calc(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn calc2(mut mass: i32) -> i32 {
    let mut result = 0;
    mass = (mass / 3) - 2;
    while mass > 0 {
        result += mass;
        mass = (mass / 3) - 2;
    }
    result
}
