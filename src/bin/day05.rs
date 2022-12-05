use std::str::Chars;
use std::str::FromStr;

#[derive(Debug)]
struct Command {
    amount: u32,
    from: usize,
    to: usize,
}

struct BoxesSlice<'a> {
    chars: Chars<'a>,
}

impl<'a> BoxesSlice<'a> {
    fn new(s: &'a str) -> Self {
        BoxesSlice { chars: s.chars() }
    }
}

impl<'a> Iterator for BoxesSlice<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(_) = self.chars.next() {
            let result = self.chars.next();
            self.chars.next(); // closing bracket ]
            self.chars.next(); // space separator
            result
        } else {
            None
        }
    }
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // move [amount] from [from] to [to]
        let mut tokens = s.split_whitespace();
        tokens.next();
        let amount = tokens.next().map(|v| v.parse::<u32>()).unwrap().unwrap();
        tokens.next();
        let from = tokens.next().map(|v| v.parse::<usize>()).unwrap().unwrap() - 1;
        tokens.next();
        let to = tokens.next().map(|v| v.parse::<usize>()).unwrap().unwrap() - 1;

        Ok(Command { amount, from, to })
    }
}

fn create_stacks(stacks_data: &Vec<&str>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let mut iter = stacks_data.iter().rev();
    let numbers_line = iter.next().unwrap();
    for _ in numbers_line.split_whitespace() {
        result.push(Vec::<char>::new());
    }
    for slice_line in iter {
        let boxes_slice = BoxesSlice::new(slice_line);
        for (i, ch) in boxes_slice.enumerate() {
            if ch != ' ' {
                result[i].push(ch);
            }
        }
    }
    result
}

fn get_stacks_tops(stacks: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    result.extend(stacks.iter().map(|s| s.last().unwrap()));
    result
}

fn main() {
    let data = aoc::read_to_string("data/day05.txt");
    let mut lines = data.lines();
    let mut stacks_data = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.len() == 0 {
            break;
        }
        stacks_data.push(line);
    }

    let commands = aoc::map_lines::<Command>(lines);
    {
        let mut stacks = create_stacks(&stacks_data);

        for c in &commands {
            for _ in 0..c.amount {
                let cr = stacks[c.from].pop().unwrap();
                stacks[c.to].push(cr);
            }
        }

        let result1 = get_stacks_tops(&stacks);
        println!("Part 1: {}", result1);
    }

    {
        let mut stacks = create_stacks(&stacks_data);
        for c in &commands {
            let mut tmp = Vec::new();
            for _ in 0..c.amount {
                tmp.push(stacks[c.from].pop().unwrap());
            }
            for cr in tmp.iter().rev() {
                stacks[c.to].push(*cr);
            }
        }
        println!("Part 2: {}", get_stacks_tops(&stacks));
    }
}

#[cfg(test)]
mod test {
    use crate::BoxesSlice;

    #[test]
    fn test_chars() {
        let s = "[A] [B]     [D]    ";
        let chars = BoxesSlice::new(s).collect::<Vec<char>>();
        assert_eq!(chars, vec!['A', 'B', ' ', 'D', ' ']);
    }
}
