use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Sections {
    start: u32,
    end: u32,
}

#[derive(Debug, PartialEq)]
struct Pair {
    a: Sections,
    b: Sections,
}

impl FromStr for Sections {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("-").ok_or("Cannot parse sections.")?;
        Ok(Sections {
            start: a.parse::<u32>().map_err(|_| "Cannot parse sections")?,
            end: b.parse::<u32>().map_err(|_| "Cannot parse sections")?,
        })
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(",").ok_or("Cannot parse pair")?;
        Ok(Pair {
            a: Sections::from_str(a)?,
            b: Sections::from_str(b)?,
        })
    }
}

fn main() {
    let pairs = aoc::read_one_per_line::<Pair>("data/day04.txt");
    let count1 = pairs
        .iter()
        .filter(|p| {
            (p.a.start <= p.b.start && p.a.end >= p.b.end)
                || (p.b.start <= p.a.start && p.b.end >= p.a.end)
        })
        .count();
    println!("Part 1: {}", count1);

    let count2 = pairs
        .iter()
        .filter(|p| p.a.start <= p.b.end && p.a.end >= p.b.start)
        .count();
    println!("Part 2: {}", count2);
}

#[cfg(test)]
mod test {
    use crate::Pair;
    use crate::Sections;
    use std::str::FromStr;

    #[test]
    fn test_sections_from_str() {
        let s = Sections::from_str("4-8");
        assert_eq!(s, Ok(Sections { start: 4, end: 8 }));
    }

    #[test]
    fn test_pair_from_str() {
        let p = Pair::from_str("1-4,2-8");
        assert_eq!(
            p,
            Ok(Pair {
                a: Sections { start: 1, end: 4 },
                b: Sections { start: 2, end: 8 },
            })
        );
    }
}
