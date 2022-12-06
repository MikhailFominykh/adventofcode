use std::collections::HashSet;

fn are_distinct(chars: &[char]) -> bool {
    let mut set: HashSet<char> = HashSet::with_capacity(chars.len());
    set.extend(chars);
    set.len() == chars.len()
}

fn get_first_offset(chars: &[char], window_size: usize) -> usize {
    let mut start_index = None;
    for (i, w) in chars.windows(window_size).enumerate() {
        if are_distinct(w) {
            start_index = Some(i);
            break;
        }
    }
    start_index.map(|x| x + window_size).unwrap_or(0)
}

fn main() {
    let data = std::fs::read_to_string("data/day06.txt").unwrap();
    let s = &data.chars().collect::<Vec<char>>()[..];
    println!("Part 1: {}", get_first_offset(s, 4));
    println!("Part 2: {}", get_first_offset(s, 14));
}
