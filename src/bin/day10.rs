#[derive(Debug, PartialEq)]
enum Op {
    Noop,
    Addx(i32),
}

const ERROR_MESSAGE: &'static str = "Cannot parse operation";

impl std::str::FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let op_name = parts.next().ok_or(ERROR_MESSAGE)?;
        let op = match op_name {
            "addx" => {
                let v = parts
                    .next()
                    .ok_or(ERROR_MESSAGE)?
                    .parse::<i32>()
                    .map_err(|_| ERROR_MESSAGE)?;
                Op::Addx(v)
            }
            "noop" => Op::Noop,
            _ => return Err(ERROR_MESSAGE),
        };
        Ok(op)
    }
}

struct Cycles<'a> {
    ops: std::slice::Iter<'a, Op>,
    reg: i32,
    remaining_cycles: i32,
    op_value: i32,
}

impl<'a> Cycles<'a> {
    fn new(ops: &'a [Op]) -> Self {
        Cycles {
            ops: ops.iter(),
            reg: 1,
            remaining_cycles: 0,
            op_value: 0,
        }
    }
}

impl Iterator for Cycles<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_cycles == 0 {
            let op = self.ops.next();
            match op {
                Some(o) => {
                    self.reg += self.op_value;
                    match o {
                        Op::Noop => {
                            self.op_value = 0;
                            self.remaining_cycles = 0;
                        }
                        Op::Addx(v) => {
                            self.op_value = *v;
                            self.remaining_cycles = 1;
                        }
                    }
                    Some(self.reg)
                }
                None => None,
            }
        } else {
            self.remaining_cycles -= 1;
            Some(self.reg)
        }
    }
}

fn get_symbol(x: usize, reg: i32) -> char {
    if (x as i32 - reg).abs() < 2 { '#' } else { ' ' }
}

fn main() {
    let ops = aoc::read_one_per_line::<Op>("data/day10.txt");
    let cycles = Cycles::new(&ops);
    let sum = cycles
        .enumerate()
        .take(220)
        .skip(19)
        .step_by(40)
        .map(|(i, v)| (i + 1) as i32 * v)
        .sum::<i32>();
    println!("Part 1: {}", sum);

    println!("Part 2:");
    let cycles = Cycles::new(&ops);
    const WIDTH: usize = 40;
    for (i, v) in cycles.enumerate() {
        let x = i % WIDTH;
        let ch = get_symbol(x, v);
        print!("{}", ch);
        if x == WIDTH - 1 {
            println!("");
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cycles;
    use super::Op;
    use std::str::FromStr;

    #[test]
    fn test_op_from_string() {
        assert_eq!(Op::from_str("noop"), Ok(Op::Noop));
        assert_eq!(Op::from_str("addx 3"), Ok(Op::Addx(3)));
    }

    #[test]
    fn test_cycles() {
        let ops = [Op::Noop, Op::Addx(3), Op::Addx(1), Op::Noop];
        let mut cycles = Cycles::new(&ops);
        assert_eq!(cycles.next(), Some(0));
        assert_eq!(cycles.next(), Some(0));
        assert_eq!(cycles.next(), Some(0));
        assert_eq!(cycles.next(), Some(3));
    }
}
