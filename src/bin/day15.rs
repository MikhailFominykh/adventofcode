use std::collections::HashSet;
use std::str::FromStr;

type Vec2 = (i32, i32);

#[derive(Debug, Clone)]
struct DataItem {
    s: Vec2,
    b: Vec2,
}

impl DataItem {
    fn distance(&self) -> i32 {
        manhattan_distance(self.s, self.b)
    }

    fn get_coverage_interval(&self, line_index: i32) -> Option<(i32, i32)> {
        let distance_to_line = (line_index - self.s.1).abs();
        let d = self.distance();
        if distance_to_line > d {
            None
        } else {
            let delta = d - distance_to_line;
            Some((self.s.0 - delta, self.s.0 + delta))
        }
    }
}

#[inline]
fn manhattan_distance(a: Vec2, b: Vec2) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

impl FromStr for DataItem {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, b) = s.split_once(":").ok_or("Cannot parse")?;
        let s = &s[10..];
        let b = &b[22..];
        Ok(Self {
            s: parse_vec2(s),
            b: parse_vec2(b),
        })
    }
}

fn parse_vec2(s: &str) -> (i32, i32) {
    let (x, y) = s.split_once(", ").unwrap();
    let x = x[2..].parse::<i32>().unwrap();
    let y = y[2..].parse::<i32>().unwrap();
    (x, y)
}

fn get_line_coverage(data: &[DataItem], line_index: i32) -> HashSet<i32> {
    data.iter().fold(HashSet::<i32>::new(), |mut acc, item| {
        if let Some((start, end)) = item.get_coverage_interval(line_index) {
            for i in start..=end {
                acc.insert(i);
            }
        }
        acc
    })
}

fn get_coverage_count(data: &[DataItem], line_index: i32) -> usize {
    let mut coverage = get_line_coverage(data, line_index);
    for item in data {
        if item.b.1 == line_index {
            coverage.remove(&item.b.0);
        }
    }
    coverage.len()
}

fn any_interval_contains(x: i32, intervals: &[(i32, i32)]) -> bool {
    for i in intervals {
        if x >= i.0 && x <= i.1 {
            return true;
        }
    }
    false
}

fn find_not_covered(line_index: i32, max_x: i32, data: &[DataItem]) -> Option<i32> {
    let intervals: Vec<_> = data
        .iter()
        .filter_map(|d| d.get_coverage_interval(line_index))
        .collect();
    for interval in &intervals {
        let left = interval.0 - 1;
        if left >= 0 {
            if !any_interval_contains(left, &intervals) {
                return Some(left);
            }
        }
        let right = interval.1 + 1;
        if right <= max_x {
            if !any_interval_contains(right, &intervals) {
                return Some(right);
            }
        }
    }
    None
}

fn main() {
    let data = aoc::read_one_per_line::<DataItem>("data/day15.txt");
    let count1 = get_coverage_count(&data, 2000000);
    println!("Part1: {}", count1);

    const MAX_INDEX: usize = 4000000;
    const MAX_LINE_INDEX: usize = 4000000;
    let mut found = None;
    'lines: for line_index in 0..=MAX_LINE_INDEX {
        if let Some(x) = find_not_covered(line_index as i32, MAX_INDEX as i32, &data) {
            found = Some((x, line_index as i32));
            break 'lines;
        }
    }
    if let Some((x, y)) = found {
        let tuning_freq = x as u64 * 4000000 + y as u64;
        println!("Part2: {}", tuning_freq);
    } else {
        println!("Not found");
    }
}
