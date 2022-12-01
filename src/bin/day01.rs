use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Item {
    Food(u32),
    Separator,
}

impl FromStr for Item {
    type Err = std::num::ParseIntError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.len() > 0 {
            Ok(Item::Food(str.parse()?))
        } else {
            Ok(Item::Separator)
        }
    }
}

struct Backpacks<'a> {
    items: &'a Vec<Item>,
    start_index: usize,
}

impl<'a> Backpacks<'a> {
    fn new(items: &'a Vec<Item>) -> Self {
        let mut start_index = 0;
        while start_index < items.len() && items[start_index] == Item::Separator {
            start_index += 1;
        }
        Backpacks { items, start_index }
    }
}

impl<'a> Iterator for Backpacks<'a> {
    type Item = &'a [Item];

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_index == self.items.len() {
            None
        } else {
            let mut end_index = self.start_index + 1;
            while end_index < self.items.len() && self.items[end_index] != Item::Separator {
                end_index += 1;
            }
            let result = &self.items[self.start_index..end_index];
            let mut next_start_index = end_index;
            while next_start_index < self.items.len() && self.items[next_start_index] == Item::Separator {
                next_start_index += 1;
            }
            self.start_index = next_start_index;
            Some(result)
        }
    }
}

fn sum(items: &[Item]) -> u32 {
    items
        .iter()
        .filter_map(|item| match item {
            Item::Food(v) => Some(v),
            Item::Separator => None,
        })
        .sum()
}

fn sum_top_backpacks(values: &Vec<Item>, n: usize) -> u32 {
    let backpacks = Backpacks::new(values);
    let mut sums = backpacks.map(|b| sum(b)).collect::<Vec<u32>>();
    sums.sort_by(|a, b| b.cmp(a));
    sums.iter().take(n).sum::<u32>()
}

fn main() {
    let values = aoc::read_one_per_line::<Item>("data/day01.txt");
    println!("Part 1: {}", sum_top_backpacks(&values, 1));
    println!("Part 2: {}", sum_top_backpacks(&values, 3));
}
