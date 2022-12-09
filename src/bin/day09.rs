use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    amount: i32,
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" ").ok_or("Cannot parse command")?;
        let amount = b.parse::<i32>().map_err(|_| "Cannot parse command")?;
        let direction = match a {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => return Err("Cannot parse command"),
        };
        Ok(Move { direction, amount })
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Default, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn vec2(x: i32, y: i32) -> Vec2 {
    Vec2 { x, y }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Vec2>,
}

impl Rope {
    fn new(count: usize) -> Self {
        let knots = vec![Vec2::default(); count];
        Rope { knots }
    }

    fn head(&self) -> Vec2 {
        self.knots[0]
    }

    fn tail(&self) -> Vec2 {
        *self.knots.last().unwrap()
    }

    fn make_move(&mut self, direction: Direction) {
        let head = self.knots[0];
        let new_head = match direction {
            Direction::U => vec2(head.x, head.y + 1),
            Direction::D => vec2(head.x, head.y - 1),
            Direction::L => vec2(head.x - 1, head.y),
            Direction::R => vec2(head.x + 1, head.y),
        };
        self.knots[0] = new_head;
        for i in 0..self.knots.len() - 1 {
            let head = self.knots[i];
            let mut tail = self.knots[i + 1];
            let dx = head.x - tail.x;
            debug_assert!(dx.abs() <= 2);
            let dy = head.y - tail.y;
            debug_assert!(dy.abs() <= 2);
            if dx.abs() == 2 || dy.abs() == 2 {
                tail.x += dx.signum();
                tail.y += dy.signum();
                self.knots[i + 1] = tail;
            }
        }
    }
}

fn get_tail_positions_count(knots_count: usize, moves: &[Move]) -> usize {
    let mut rope = Rope::new(knots_count);
    let mut tail_positions = HashSet::new();
    tail_positions.insert(rope.tail());
    for m in moves {
        for _ in 0..m.amount {
            rope.make_move(m.direction);
            tail_positions.insert(rope.tail());
        }
    }
    tail_positions.len()
}

fn main() {
    let moves = aoc::read_one_per_line::<Move>("data/day09.txt");

    let count1 = get_tail_positions_count(2, &moves);
    println!("Part 1: {}", count1);

    let count2 = get_tail_positions_count(10, &moves);
    println!("Part 2: {}", count2);
}

#[cfg(test)]
mod test {
    use super::vec2;
    use super::Direction;
    use super::Move;
    use super::Rope;
    use std::str::FromStr;

    #[test]
    fn test_move_from_str() {
        assert_eq!(Move::from_str("U 4"), Ok(Move { direction: Direction::U, amount: 4 }));
        assert_eq!(Move::from_str("D 3"), Ok(Move { direction: Direction::D, amount: 3 }));
        assert_eq!(Move::from_str("L 2"), Ok(Move { direction: Direction::L, amount: 2 }));
        assert_eq!(Move::from_str("R 1"), Ok(Move { direction: Direction::R, amount: 1 }));
    }

    #[test]
    fn test_rope() {
        let mut r = Rope::new(2);
        r.knots[0] = vec2(1, 1);
        r.make_move(Direction::U);
        assert_eq!(r.head(), vec2(1, 2));
        assert_eq!(r.tail(), vec2(1, 1));
    }
}
