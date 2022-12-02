use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Strategy {
    X,
    Y,
    Z,
}

#[derive(Debug)]
struct Round {
    a: Shape,
    b: Shape,
}

#[derive(Debug)]
struct Guide {
    shape: Shape,
    strategy: Strategy,
}

impl FromStr for Strategy {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Strategy::X,
            "Y" => Strategy::Y,
            "Z" => Strategy::Z,
            _ => return Err("Cannot parse strategy"),
        })
    }
}

impl Shape {
    fn get_by_offset(self, offset: i32) -> Shape {
        let ordinal = (self as i32 + offset + 3) % 3;
        unsafe { std::mem::transmute::<i32, Shape>(ordinal) }
    }

    fn get_victor(self) -> Shape {
        self.get_by_offset(1)
    }

    fn get_victim(self) -> Shape {
        self.get_by_offset(-1)
    }

    fn beats(self, other: Shape) -> bool {
        self.get_victim() == other
    }

    fn score(self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl Round {
    fn score(&self) -> u32 {
        let outcome_score = if self.b.beats(self.a) {
            6
        } else if self.a.beats(self.b) {
            0
        } else {
            3
        };
        outcome_score + self.b.score()
    }
}

impl FromStr for Shape {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => return Err("Cannot parse Shape"),
        })
    }
}

impl FromStr for Guide {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" ").ok_or("Cannot parse guide")?;
        Ok(Guide {
            shape: Shape::from_str(a)?,
            strategy: Strategy::from_str(b)?,
        })
    }
}

fn get_total_score<F>(guides: &Vec<Guide>, action: F) -> u32
where
    F: Fn(&Guide) -> Shape,
{
    guides
        .iter()
        .map(|g| {
            let r = Round {
                a: g.shape,
                b: action(g),
            };
            r.score()
        })
        .sum::<u32>()
}

fn main() {
    let guides = aoc::read_one_per_line::<Guide>("data/day02.txt");
    println!(
        "Part 1: {}",
        get_total_score(&guides, |g| {
            match g.strategy {
                Strategy::X => Shape::Rock,
                Strategy::Y => Shape::Paper,
                Strategy::Z => Shape::Scissors,
            }
        })
    );
    println!(
        "Part 2: {}",
        get_total_score(&guides, |g| {
            match g.strategy {
                Strategy::X => g.shape.get_victim(),
                Strategy::Y => g.shape,
                Strategy::Z => g.shape.get_victor(),
            }
        })
    );
}

#[cfg(test)]
mod test {
    use crate::Shape;

    #[test]
    fn test() {
        assert_eq!(Shape::beats(Shape::Paper, Shape::Rock), true);
        assert_eq!(Shape::beats(Shape::Paper, Shape::Scissors), false);
    }
}
