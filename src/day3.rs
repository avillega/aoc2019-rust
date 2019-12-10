use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

enum Direction {
    R,
    U,
    L,
    D,
}
struct Move {
    direction: Direction,
    steps: i32,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Coor(i32, i32);

struct Node {
    coor: Coor,
    steps: i32,
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coor.hash(state)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coor == other.coor
    }
}

impl Eq for Node {}

impl Move {
    fn new(raw_dir: &str) -> Self {
        let direction = match raw_dir.chars().nth(0) {
            Some('R') => Direction::R,
            Some('U') => Direction::U,
            Some('L') => Direction::L,
            Some('D') => Direction::D,
            _ => unreachable!(),
        };
        let steps = raw_dir[1..].parse().unwrap();

        Move { direction, steps }
    }

    fn to_coord(&self, current_coord: &Coor) -> Vec<Coor> {
        (1..=self.steps)
            .map(|step| match self.direction {
                Direction::R => Coor(current_coord.0 + step, current_coord.1),
                Direction::L => Coor(current_coord.0 - step, current_coord.1),
                Direction::U => Coor(current_coord.0, current_coord.1 + step),
                Direction::D => Coor(current_coord.0, current_coord.1 - step),
            })
            .collect()
    }

    fn to_nodes(&self, current_coord: &Coor, mut current_num_step: i32) -> Vec<Node> {
        (1..=self.steps)
            .map(|step| match self.direction {
                Direction::R => Node {
                    coor: Coor(current_coord.0 + step, current_coord.1),
                    steps: current_num_step + step,
                },
                Direction::L => Node {
                    coor: Coor(current_coord.0 - step, current_coord.1),
                    steps: current_num_step + step,
                },
                Direction::U => Node {
                    coor: Coor(current_coord.0, current_coord.1 + step),
                    steps: current_num_step + step,
                },
                Direction::D => Node {
                    coor: Coor(current_coord.0, current_coord.1 - step),
                    steps: current_num_step + step,
                },
            })
            .collect()
    }
}

pub fn solve1() -> i32 {
    let input = parse_input();
    let line1 = &input[0];
    let line2 = &input[1];

    let wire1 = wire(line1);
    let wire2 = wire(line2);

    wire1
        .intersection(&wire2)
        .map(dist_to_origin)
        .min()
        .unwrap()
}

pub fn solve2() -> i32 {
    let input = parse_input();
    let line1 = &input[0];
    let line2 = &input[1];

    let wire1 = wire2(line1);
    let wire2 = wire2(line2);

    wire1
        .intersection(&wire2)
        .map(|inter| {
            let steps1 = wire1.get(&inter).unwrap().steps;
            let steps2 = wire2.get(&inter).unwrap().steps;

            steps1 + steps2
        })
        .min()
        .unwrap()
}

fn dist_to_origin(coor: &Coor) -> i32 {
    coor.0.abs() + coor.1.abs()
}

fn wire(moves: &Vec<Move>) -> HashSet<Coor> {
    let mut current_coor = Coor(0, 0);
    moves
        .into_iter()
        .flat_map(|m| {
            let coors = m.to_coord(&current_coor);
            let last_coor = coors.last().expect("last coor should exist");
            current_coor = Coor(last_coor.0, last_coor.1);
            coors
        })
        .collect()
}

fn wire2(moves: &Vec<Move>) -> HashSet<Node> {
    let mut current_coor = Coor(0, 0);
    let mut current_step = 0;
    moves
        .into_iter()
        .flat_map(|m| {
            println!("current Step {}", current_step);
            let nodes = m.to_nodes(&current_coor, current_step);
            let last_node = nodes.last().expect("last node must exist");
            current_coor = last_node.coor.clone();
            current_step += nodes.len() as i32;
            nodes
        })
        .collect()
}

fn parse_input() -> Vec<Vec<Move>> {
    fs::read_to_string("./input/day3_1.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|str_dir| Move::new(str_dir))
                .collect::<Vec<_>>()
        })
        .take(2)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_coord() {
        let mve_r = Move::new("R2");
        let mve_l = Move::new("L2");
        let mve_u = Move::new("U2");
        let mve_d = Move::new("D2");
        assert_eq!(mve_r.to_coord(&Coor(0, 0)), vec![Coor(1, 0), Coor(2, 0)]);
        assert_eq!(mve_l.to_coord(&Coor(0, 0)), vec![Coor(-1, 0), Coor(-2, 0)]);
        assert_eq!(mve_u.to_coord(&Coor(0, 0)), vec![Coor(0, 1), Coor(0, 2)]);
        assert_eq!(mve_d.to_coord(&Coor(0, 0)), vec![Coor(0, -1), Coor(0, -2)]);
    }

    #[test]
    fn nodes_set() {
        let mut a = HashSet::new();
        a.insert(Node {
            coor: Coor(0, 0),
            steps: 0,
        });

        a.insert(Node {
            coor: Coor(0, 0),
            steps: 1,
        });

        assert!(a.len() == 1);
        assert!(a.into_iter().take(1).next().unwrap().steps == 0);
    }
}
