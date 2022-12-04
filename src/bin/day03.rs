use std::collections::HashSet;

fn get_priority(c: u8) -> u32 {
    match c {
        b'a'..=b'z' => (c - b'a' + 1) as u32,
        b'A'..=b'Z' => (c - b'A' + 27) as u32,
        _ => panic!("Wrong character: {}", c),
    }
}

fn get_duplicated(s: &[u8]) -> Option<u8> {
    let half_len = s.len() / 2;
    let mut set: HashSet<u8> = HashSet::new();
    for c in &s[..half_len] {
        set.insert(*c);
    }
    for c in &s[half_len..] {
        if set.contains(c) {
            return Some(*c);
        }
    }
    None
}

fn get_common_item(backpaks: &[String]) -> Option<u8> {
    let intersection = backpaks.iter().map(|s| {
        let mut set = HashSet::new();
        for b in s.as_bytes() {
            set.insert(*b);
        }
        set
    }).reduce(|acc, s| acc.intersection(&s).map(|v| *v).collect());
    intersection.map(|it| it.iter().next().copied()).flatten()
}

fn main() {
    let backpaks = aoc::read_one_per_line::<String>("data/day03.txt");
    let sum = backpaks
        .iter()
        .filter_map(|b| {
            get_duplicated(b.as_bytes()).map(get_priority)
        })
        .sum::<u32>();
    println!("Part 1: {}", sum);

    let sum2 = backpaks.chunks(3).filter_map(get_common_item).map(get_priority).sum::<u32>();
    println!("Part 2: {}", sum2);
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let bytes = s.as_bytes();
        assert_eq!(s.len(), bytes.len(), "Should be equal length");
        let dup = crate::get_duplicated(s.as_bytes());
        assert_eq!(dup, Some(b'p'), "Should be correct symbol");
    }

    #[test]
    fn badge() {
        let backpaks = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string()
        ];
        let common_item = crate::get_common_item(&backpaks[..]);
        assert_eq!(common_item, Some(b'Z'));
    }
}
