pub fn solve1(lower: u32, upper: u32) -> u32 {
    (lower..=upper)
        .filter(check_not_decreasing)
        .filter(check_two_adjacent)
        .count() as u32
}

pub fn solve2(lower: u32, upper: u32) -> u32 {
    (lower..=upper)
        .filter(check_not_decreasing)
        .filter(check_at_most_two_adjacent)
        .count() as u32
}

fn check_not_decreasing(num: &u32) -> bool {
    let chars: Vec<char> = num.to_string().chars().collect();
    for i in 1..chars.len() {
        if chars[i] < chars[i - 1] {
            return false;
        }
    }

    true
}

fn check_two_adjacent(num: &u32) -> bool {
    let chars: Vec<char> = num.to_string().chars().collect();
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }

    false
}

fn check_at_most_two_adjacent(num: &u32) -> bool {
    let mut digits = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    num.to_string()
        .chars()
        .for_each(|c| digits[c.to_string().parse::<usize>().unwrap()] += 1);

    digits.into_iter().any(|digit| digit == 2)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn not_decreasing() {
        assert!(check_not_decreasing(&111111));
        assert!(!check_not_decreasing(&223450));
        assert!(check_not_decreasing(&122345));
        assert!(!check_not_decreasing(&123234));
        assert!(check_not_decreasing(&123789));
        assert!(!check_not_decreasing(&135676));
    }

    #[test]
    fn check_adjacent() {
        assert!(check_two_adjacent(&122345));
        assert!(check_two_adjacent(&123455));
        assert!(!check_two_adjacent(&123456));
        assert!(!check_two_adjacent(&124589));
    }

    #[test]
    fn check_two_at_most_adjacent() {
        assert!(check_at_most_two_adjacent(&122345));
        assert!(check_at_most_two_adjacent(&111122));
        assert!(!check_at_most_two_adjacent(&111123));
        assert!(!check_at_most_two_adjacent(&123444));
    }
}
