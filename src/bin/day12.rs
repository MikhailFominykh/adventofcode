use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum MapCell {
    Start,
    End,
    Ground(i32),
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    cells: Vec<MapCell>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    L,
    R,
    U,
    D,
}

impl Direction {
    fn transform(&self, ij: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::L => (ij.0 - 1, ij.1),
            Direction::R => (ij.0 + 1, ij.1),
            Direction::U => (ij.0, ij.1 - 1),
            Direction::D => (ij.0, ij.1 + 1),
        }
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            MapCell::Start => 'S',
            MapCell::End => 'E',
            MapCell::Ground(v) => char::from_u32(*v as u32).unwrap(),
        };
        write!(f, "{}", v)
    }
}

impl MapCell {
    fn from_byte(b: u8) -> Self {
        match b {
            b'S' => Self::Start,
            b'E' => Self::End,
            v => Self::Ground(v as i32),
        }
    }

    fn get_height(&self) -> i32 {
        match self {
            MapCell::Start => b'a' as i32,
            MapCell::End => b'z' as i32 + 1,
            MapCell::Ground(h) => *h as i32,
        }
    }
}

impl Map {
    fn from_string(s: &str) -> Self {
        let mut height = 0;
        let mut width = 0;
        let mut cells = Vec::new();
        for line in s.lines() {
            width = line.len();
            height += 1;
            for ch in line.bytes() {
                cells.push(MapCell::from_byte(ch));
            }
        }
        Map {
            width,
            height,
            cells,
        }
    }

    fn get_start_index(&self) -> usize {
        self.cells.iter().position(|c| *c == MapCell::Start).unwrap()
    }

    fn can_move(&self, from_index: usize, to_index: usize) -> bool {
        match self.cells[to_index] {
            MapCell::Start => false,
            g => g.get_height() - self.cells[from_index].get_height() < 2,
        }
    }

    fn index1d(&self, coords: (usize, usize)) -> usize {
        assert!(coords.0 < self.width);
        assert!(coords.1 < self.height);
        coords.1 * self.width + coords.0
    }

    fn index2d(&self, index: usize) -> (usize, usize) {
        let j = index / self.width;
        let i = index - j * self.width;
        (i, j)
    }

    fn get_directions(&self, i: usize, j: usize) -> Directions {
        let directions = [Direction::L, Direction::R, Direction::U, Direction::D];
        let w = self.width;
        let h = self.height;
        let directions = directions
            .into_iter()
            .filter(move |d| match d {
                Direction::L => i > 0,
                Direction::R => i < w - 1,
                Direction::U => j > 0,
                Direction::D => j < h - 1,
            })
            .collect::<Vec<_>>();
        Directions {
            values: directions,
            index: 0,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, c) in self.cells.iter().enumerate() {
            write!(f, "{}", c)?;
            if (i + 1) % self.width == 0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

struct Step {
    i: usize,
    j: usize,
    directions: Option<Directions>,
    direction: Direction,
}

#[derive(Debug)]
struct Directions {
    values: Vec<Direction>,
    index: usize,
}

impl Step {
    fn new(coords: (usize, usize)) -> Self {
        Self {
            i: coords.0,
            j: coords.1,
            directions: None,
            direction: Direction::U,
        }
    }
}

impl Directions {
    fn next(&mut self) -> Option<Direction> {
        if self.index >= self.values.len() {
            return None;
        }
        let result = self.values[self.index];
        self.index += 1;
        Some(result)
    }
}

fn find_path(map: &Map) -> Option<Vec<Direction>> {
    let mut paths = Vec::<Vec<Direction>>::new();
    let mut cost = vec![usize::MAX; map.width * map.height];
    let mut prev_steps = Vec::<Step>::new();
    let start_index = map.cells.iter().position(|x| *x == MapCell::Start).unwrap();
    let mut current_step = Step::new(map.index2d(start_index));
    loop {
        let current_index = map.index1d((current_step.i, current_step.j));
        match current_step.directions.as_mut() {
            Some(n) => {
                let dir = n.next();
                match dir {
                    Some(d) => {
                        let (i, j) = d.transform((current_step.i, current_step.j));
                        let next_index = map.index1d((i, j));
                        let new_cost = prev_steps.len() + 1;
                        if cost[next_index] > new_cost && map.can_move(current_index, next_index) {
                            current_step.direction = d;
                            prev_steps.push(current_step);
                            current_step = Step::new(map.index2d(next_index));
                        }
                    }
                    None => {
                        match prev_steps.pop() {
                            Some(s) => current_step = s,
                            None => break,
                        }
                    }
                }
            }
            None => {
                // Just arrived at the cell.
                if map.cells[current_index] == MapCell::End {
                    paths.push(prev_steps.iter().map(|s| s.direction).collect::<Vec<_>>());
                    // Empty directions set so we return to the previous step on the next
                    // iteration.
                    current_step.directions = Some(Directions {
                        values: Vec::new(),
                        index: 0,
                    });
                } else {
                    cost[current_index] = prev_steps.len();
                    current_step.directions = Some(map.get_directions(current_step.i, current_step.j));
                }
            }
        }
    }
    paths.sort_by_key(|p| p.len());
    if paths.len() > 0 {
        Some(paths.remove(0))
    } else {
        None
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day12.txt").unwrap();
    let map = Map::from_string(&data);
    let path = find_path(&map);
    println!("{:?}", &path);

    if let Some(p) = path {
        let mut cells = map.cells.clone();
        let start_index = map.get_start_index();
        let mut coords = map.index2d(start_index);
        for d in &p {
            let index = map.index1d(coords);
            cells[index] = match d {
                Direction::L => MapCell::Ground(b'.' as i32),
                Direction::R => MapCell::Ground(b'.' as i32),
                Direction::U => MapCell::Ground(b'.' as i32),
                Direction::D => MapCell::Ground(b'.' as i32),
            };
            coords = d.transform(coords);
        }
        let map2 = Map { width: map.width, height: map.height, cells };
        println!("{}", map);
        println!("{}", map2);
        println!("Path length: {}", p.len());
    }
}

