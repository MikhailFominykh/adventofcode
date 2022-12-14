use std::collections::HashMap;
use std::str::FromStr;

type Vec2 = (i32, i32);
type Map = HashMap<Vec2, Cell>;

struct Cave {
    map: Map,
    implicit_bottom_y: Option<i32>,
}

#[derive(Debug)]
struct Path(Vec<(i32, i32)>);

enum Cell {
    Rock,
    Sand,
}

impl FromStr for Path {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split("->")
            .map(|p| {
                let (x, y) = p.trim().split_once(",").unwrap();
                let x = x.parse::<i32>().unwrap();
                let y = y.parse::<i32>().unwrap();
                (x, y)
            })
            .collect::<Vec<_>>();
        Ok(Path(points))
    }
}

fn get_range(a: i32, b: i32) -> std::ops::Range<i32> {
    if a <= b {
        a..b + 1
    } else {
        b..a + 1
    }
}

impl Cave {
    fn from_str(s: &str) -> Self {
        let mut map = HashMap::new();
        for p in s.lines().map(|x| Path::from_str(x).unwrap()) {
            p.fill(&mut map);
        }
        Cave {
            map,
            implicit_bottom_y: None,
        }
    }

    fn get_bounds(&self) -> (Vec2, Vec2) {
        let mut min = (1000i32, 0i32);
        let mut max = (0i32, 0i32);
        for v in self.map.keys() {
            min = (i32::min(min.0, v.0), i32::min(min.1, v.1));
            max = (i32::max(max.0, v.0), i32::max(max.1, v.1));
        }
        (min, max)
    }

    fn get_bottom(&self, p: Vec2, max_y: i32) -> Option<Vec2> {
        for y in p.1..=max_y {
            if self.map.contains_key(&(p.0, y)) {
                return Some((p.0, y));
            }
        }
        self.implicit_bottom_y.map(|y| (p.0, y))
    }

    fn get_rest_position(&self, p: Vec2, max_y: i32) -> Option<Vec2> {
        let mut curr = p;
        loop {
            if let Some(bottom) = self.get_bottom(curr, max_y) {
                if curr == bottom {
                    return Some(curr);
                }
                curr = (bottom.0, bottom.1 - 1);
                if let Some(bottom_y) = self.implicit_bottom_y {
                    if bottom_y == bottom.1 {
                        return Some(curr);
                    }
                }
                let left = (curr.0 - 1, curr.1 + 1);
                if self.map.contains_key(&left) {
                    let right = (curr.0 + 1, curr.1 + 1);
                    if self.map.contains_key(&right) {
                        return Some(curr);
                    } else {
                        curr = right;
                    }
                } else {
                    curr = left;
                }
            } else {
                return None;
            }
        }
    }

    fn drop_sand(&mut self, p: Vec2, max_y: i32) -> Option<Vec2> {
        if let Some(pos) = self.get_rest_position(p, max_y) {
            self.map.insert(pos, Cell::Sand);
            return Some(pos);
        } else {
            return None;
        }
    }

    fn print(&self) {
        let (min, max) = self.get_bounds();
        let w = max.0 - min.0 + 1;
        let h = max.1 - min.1 + 1;
        for y in 0..h {
            for x in 0..w {
                let key = (min.0 + x, min.1 + y);
                let ch = if let Some(v) = self.map.get(&key) {
                    match v {
                        Cell::Rock => "#",
                        Cell::Sand => "o",
                    }
                } else {
                    "."
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

impl Path {
    fn fill(&self, map: &mut HashMap<(i32, i32), Cell>) {
        for line in self.0.windows(2) {
            let a = line[0];
            let b = line[1];
            let dx = b.0 - a.0;
            let dy = b.1 - a.1;
            assert!((dx == 0 && dy != 0) || (dx != 0 && dy == 0));
            if dx != 0 {
                for x in get_range(a.0, b.0) {
                    map.insert((x, a.1), Cell::Rock);
                }
            } else {
                for y in get_range(a.1, b.1) {
                    map.insert((a.0, y), Cell::Rock);
                }
            }
        }
    }
}

fn part1(data: &str, print: bool) {
    let mut cave = Cave::from_str(&data);
    let (_, max) = cave.get_bounds();
    let max_y = max.1;
    let drop_position = (500, 0);
    let mut count = 0;
    loop {
        let rest_pos = cave.drop_sand(drop_position, max_y);
        if print {
            cave.print();
        }
        if rest_pos.is_none() {
            break;
        } else {
            count += 1;
        }
    }
    println!("Count: {}", count);
}

fn part2(data: &str, print: bool) {
    let mut cave = Cave::from_str(&data);
    let (_, max) = cave.get_bounds();
    let max_y = max.1 + 2;
    cave.implicit_bottom_y = Some(max_y);
    let drop_position = (500, 0);
    let mut count = 0;
    loop {
        let rest_pos = cave.drop_sand(drop_position, max_y);
        if print {
            cave.print();
        }
        if let Some(p) = rest_pos {
            count += 1;
            if p == drop_position {
                break;
            }
        } else {
            break;
        }
    }
    println!("Count: {}", count);
}

fn main() {
    let data = aoc::read_to_string("data/day14.txt");
    part1(&data, false);
    part2(&data, false);
}
