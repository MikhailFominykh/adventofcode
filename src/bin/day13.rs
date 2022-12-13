use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug, Eq, Ord, Clone)]
enum Value {
    Int(i32),
    List(Vec<Value>),
}

#[derive(Debug, Eq, Ord, Clone)]
struct Packet {
    content: Vec<Value>,
}

struct Parser<'a> {
    s: &'a str,
    cursor: usize,
}

impl FromStr for Packet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        let mut curr = Vec::<Value>::new();
        let parser = Parser::new(s);
        for tok in parser.skip(1) {
            match tok {
                "[" => {
                    stack.push(curr);
                    curr = Vec::new();
                }
                "]" => {
                    let prev = stack.pop();
                    match prev {
                        Some(mut v) => {
                            v.push(Value::List(curr));
                            curr = v;
                        }
                        None => (),
                    }
                }
                "," => (),
                v => curr.push(Value::Int(v.parse::<i32>().map_err(|_| "Cannot parse")?)),
            }
        }
        Ok(Packet { content: curr })
    }
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
        Self { s, cursor: 0 }
    }

    fn next_token(&mut self) -> Option<&'a str> {
        let bytes = self.s.as_bytes();
        let mut cursor = self.cursor;
        while cursor < bytes.len() && bytes[cursor] == b' ' {
            cursor += 1;
        }

        if cursor == self.s.len() {
            return None;
        }

        let mut end = cursor + 1;
        let b = bytes[cursor];
        if b >= b'0' && b <= b'9' {
            while end < bytes.len() {
                match bytes[end] {
                    b'0'..=b'9' => end += 1,
                    _ => break,
                }
            }
        }
        self.cursor = end;
        Some(&self.s[cursor..end])
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        compare_lists(&self.content, &other.content) == Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_lists(&self.content, &other.content))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        compare_values(self, other) == Ordering::Equal
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_values(self, other))
    }
}

fn compare_values(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Int(ia), Value::Int(ib)) => ia.cmp(ib),
        (Value::Int(ia), Value::List(lb)) => compare_lists(&[Value::Int(*ia)], lb),
        (Value::List(la), Value::Int(ib)) => compare_lists(la, &[Value::Int(*ib)]),
        (Value::List(la), Value::List(lb)) => compare_lists(la, lb),
    }
}

fn compare_lists(a: &[Value], b: &[Value]) -> Ordering {
    let mut index = 0;
    loop {
        if index == a.len() && index == b.len() {
            return Ordering::Equal;
        }
        if index == a.len() {
            return Ordering::Less;
        }
        if index == b.len() {
            return Ordering::Greater;
        }
        let ordering = compare_values(&a[index], &b[index]);
        if ordering == Ordering::Equal {
            index += 1;
        } else {
            return ordering;
        }
    }
}

fn main() {
    let mut packets = aoc::read_one_per_non_empty_line::<Packet>("data/day13.txt");
    let sum = packets
        .chunks(2)
        .enumerate()
        .filter(|(_, pair)| pair[0] < pair[1])
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    println!("Part 1: {}", sum);

    let p2 = Packet::from_str("[[2]]").unwrap();
    let p6 = Packet::from_str("[[6]]").unwrap();
    packets.push(p2.clone());
    packets.push(p6.clone());
    packets.sort();
    let filtered = packets.iter().enumerate().filter(|(_, p)| **p == p2 || **p == p6).map(|(i, _)| i + 1).collect::<Vec<_>>();
    let result2 = filtered[0] * filtered[1];
    println!("Part 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::Packet;
    use super::compare_lists;
    use super::Parser;
    use std::str::FromStr;

    #[test]
    fn test() {
        let parser = Parser::new("[1, [2, [3, 4]], 5]");
        let tokens = parser.collect::<Vec<_>>();
        assert_eq!(
            tokens,
            &["[", "1", ",", "[", "2", ",", "[", "3", ",", "4", "]", "]", ",", "5", "]"]
        );
    }

    #[test]
    fn test_compare() {
        let a = Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
        let b = Packet::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();
        assert_eq!(compare_lists(&a.content, &b.content), std::cmp::Ordering::Greater);
    }
}
