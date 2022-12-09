use bitflags::bitflags;

bitflags! {
    struct VisibilityFlags: u8 {
        const L = 0b0001;
        const R = 0b0010;
        const T = 0b0100;
        const B = 0b1000;
    }
}

struct Grid {
    w: usize,
    h: usize,
    values: Vec<i32>,
    visibility: Vec<VisibilityFlags>,
}

impl Grid {
    fn row(&self, index: usize) -> usize {
        index / self.w
    }

    fn col(&self, index: usize) -> usize {
        index - self.w * (index / self.w)
    }

    fn index(&self, col: usize, row: usize) -> usize {
        row * self.w + col
    }

    fn fill_visibility<T>(&mut self, indices: T, flag: VisibilityFlags)
    where
        T: Iterator<Item = usize>,
    {
        let mut max = -1;
        for index in indices {
            let value = self.values[index];
            if value > max {
                max = value;
                self.visibility[index] |= flag;
            }
        }
    }

    fn fill_visibilities(&mut self) {
        for row in 0..self.h {
            let row_indices = Indices::create_for_row(self.w, row);
            self.fill_visibility(row_indices, VisibilityFlags::L);
            let row_indices = Indices::create_for_row_rev(self.w, row);
            self.fill_visibility(row_indices, VisibilityFlags::R);
        }
        for col in 0..self.w {
            let col_indices = Indices::create_for_col(self.w, self.h, col);
            self.fill_visibility(col_indices, VisibilityFlags::B);
            let col_indices = Indices::create_for_col_rev(self.w, self.h, col);
            self.fill_visibility(col_indices, VisibilityFlags::T);
        }
    }

    fn is_visible(&self, index: usize) -> bool {
        !self.visibility[index].is_empty()
    }

    fn count_visible_trees(&self, reference_height: i32, indices: Indices) -> i32 {
        let mut count = 0;
        for index in indices {
            count += 1;
            if self.values[index] >= reference_height {
                break;
            }
        }
        count
    }

    fn get_scenic_score(&self, index: usize) -> i32 {
        let row = index / self.w;
        let col = index - row * self.w;

        let indices = Indices::create_for_row_from(self.w, row, col + 1);
        let count1 = self.count_visible_trees(self.values[index], indices);

        let indices = Indices::create_for_row_rev_from(self.w, row, col as isize - 1);
        let count2 = self.count_visible_trees(self.values[index], indices);

        let indices = Indices::create_for_col_from(self.w, self.h, col, row as isize + 1);
        let count3 = self.count_visible_trees(self.values[index], indices);

        let indices = Indices::create_for_col_rev_from(self.w, row as isize - 1, col);
        let count4 = self.count_visible_trees(self.values[index], indices);

        count1 * count2 * count3 * count4
    }
}

struct Indices {
    index: isize,
    end: isize,
    step: isize,
}

impl Indices {
    fn create_for_row(width: usize, row: usize) -> Self {
        Indices::create_for_row_from(width, row, 0)
    }

    fn create_for_row_from(width: usize, row: usize, start_col: usize) -> Self {
        let row_start = (width * row) as isize;
        Indices {
            index: row_start + start_col as isize,
            end: row_start + width as isize,
            step: 1,
        }
    }

    fn create_for_row_rev(width: usize, row: usize) -> Indices {
        Indices::create_for_row_rev_from(width, row, width as isize - 1)
    }

    fn create_for_row_rev_from(width: usize, row: usize, start_col: isize) -> Indices {
        let row_start = (width * row) as isize;
        let end = row_start - 1;
        Indices {
            index: row_start + start_col as isize,
            end,
            step: -1,
        }
    }

    fn create_for_col(width: usize, height: usize, col: usize) -> Self {
        Indices::create_for_col_from(width, height, col, 0)
    }

    fn create_for_col_from(width: usize, height: usize, col: usize, start_row: isize) -> Self {
        let start = (width as isize * start_row) + col as isize;
        let step = width as isize;
        let end = col as isize + (width * height) as isize;
        Indices {
            index: start,
            end,
            step,
        }
    }

    fn create_for_col_rev(width: usize, height: usize, col: usize) -> Self {
        Indices::create_for_col_rev_from(width, height as isize - 1, col)
    }

    fn create_for_col_rev_from(width: usize, start_row: isize, col: usize) -> Self {
        let step = -(width as isize);
        let end = col as isize + step;
        Indices {
            index: col as isize + (width as isize * start_row),
            end,
            step,
        }
    }
}

impl Iterator for Indices {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.end {
            None
        } else {
            let result = self.index;
            self.index += self.step;
            Some(result as usize)
        }
    }
}

fn get_grid() -> Grid {
    let data = std::fs::read_to_string("data/day08.txt").unwrap();
    let lines = data.lines().collect::<Vec<&str>>();
    let w = lines[0].len();
    let h = lines.len();
    let mut values = Vec::with_capacity(w * h);
    let mut visibility = Vec::with_capacity(w * h);
    for line in lines {
        for b in line.bytes() {
            values.push((b - b'0') as i32);
            visibility.push(VisibilityFlags::empty());
        }
    }

    Grid {
        w,
        h,
        values,
        visibility,
    }
}

fn main() {
    let mut grid = get_grid();
    grid.fill_visibilities();
    let count1 = grid.visibility.iter().filter(|v| !v.is_empty()).count();
    println!("Part 1: {}", count1);

    let max_score = grid.values.iter().enumerate().map(|(index, _)| {
        grid.get_scenic_score(index)
    }).max();
    println!("Part 2: {}", max_score.unwrap());
}

#[cfg(test)]
mod test {
    use super::Indices;

    #[test]
    fn col_indices() {
        // 16 17 18 19
        // 12 13 14 15
        //  8  9 10 11
        //  4  5  6  7
        //  0  1  2  3
        let w = 4;
        let h = 5;
        let indices = Indices::create_for_col(w, h, 2);
        let expected = &[2, 6, 10, 14, 18];
        assert_eq!(indices.collect::<Vec<usize>>(), expected);

        let indices = Indices::create_for_col_rev(w, h, 2);
        let mut collected = indices.collect::<Vec<usize>>();
        collected.reverse();
        assert_eq!(collected, expected);
    }

    #[test]
    fn row_indices() {
        let w = 4;
        let expected = &[8, 9, 10, 11];
        let indices = Indices::create_for_row(w, 2);
        assert_eq!(indices.collect::<Vec<usize>>(), expected);
        let indices = Indices::create_for_row_rev(w, 2);
        let mut collected = indices.collect::<Vec<usize>>();
        collected.reverse();
        assert_eq!(collected, expected);
    }
}
