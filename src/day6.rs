use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

pub fn solve1() {
    let input = parse_input1();
    let res = calc_weight(&input, "COM", 0);
    println!("{}", res);
}

pub fn solve2() {
    let input = parse_input2();
    let res = bfs(&input, "YOU", "SAN");
    println!("{}", res);
}

fn calc_weight(graph: &HashMap<String, Vec<String>>, node: &str, indirect_orbits: i64) -> i64 {
    match graph.get(node) {
        Some(children) => {
            let weight: i64 = children
                .iter()
                .map(|child| calc_weight(graph, child, indirect_orbits + 1))
                .sum();

            weight + indirect_orbits
        }
        None => indirect_orbits,
    }
}

fn parse_input1() -> HashMap<String, Vec<String>> {
    let lines = fs::read_to_string("./input/day6_1.txt").unwrap();
    let mut graph = HashMap::new();

    for line in lines.lines() {
        let mut split = line.split(')');
        let from = split.next().expect("bad input").into();
        let to = split.next().expect("bad input").into();

        let entry = graph.entry(from).or_insert(vec![]);
        entry.push(to);
    }
    graph
}

fn parse_input2() -> HashMap<String, Vec<String>> {
    let lines = fs::read_to_string("./input/day6_1.txt").unwrap();
    let mut graph = HashMap::new();

    for line in lines.lines() {
        let mut split = line.split(')');
        let from: String = split.next().expect("bad input").into();
        let to: String = split.next().expect("bad input").into();

        let from_entry = graph.entry(from.clone()).or_insert(vec![]);
        from_entry.push(to.clone());

        let to_entry = graph.entry(to).or_insert(vec![]);
        to_entry.push(from);
    }
    graph
}

fn bfs(graph: &HashMap<String, Vec<String>>, from: &str, to: &str) -> u64 {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back((from, 0));
    while !q.is_empty() {
        let (actual, steps) = q.pop_front().expect("q sould not be empty");
        if actual == to {
            return steps - 2;
        }
        visited.insert(actual);

        for child in graph.get(actual).expect("node must have children") {
            if !visited.contains(child.as_str()) {
                q.push_back((child, steps + 1));
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_weight() {
        let mut graph = HashMap::new();
        graph.insert("COM".into(), vec!["B".into()]);
        graph.insert("B".into(), vec!["G".into(), "C".into()]);
        graph.insert("G".into(), vec!["H".into()]);
        graph.insert("C".into(), vec!["D".into()]);
        graph.insert("D".into(), vec!["I".into(), "E".into()]);
        graph.insert("E".into(), vec!["F".into(), "J".into()]);
        graph.insert("J".into(), vec!["K".into()]);
        graph.insert("K".into(), vec!["L".into()]);

        assert_eq!(42, super::calc_weight(&graph, "COM", 0));
    }
}
