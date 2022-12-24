use std::borrow::BorrowMut;

#[derive(Debug)]
struct Tree {
    height: i32,
    visible: bool,
    score: u32,
}

impl Tree {
    fn new(height: i32) -> Self {
        Tree {
            height,
            visible: false,
            score: 0,
        }
    }
}

#[derive(Debug)]
struct TreeMap {
    map: Vec<Vec<Tree>>,
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

struct MyRange {
    start: usize,
    end: usize,
    curr: usize,
    finished: bool,
}

impl MyRange {
    fn new(start: usize, end: usize) -> Self {
        MyRange {
            start,
            end,
            curr: start,
            finished: false,
        }
    }
}

impl Iterator for MyRange {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let res = Some(self.curr);
        if self.start <= self.end {
            self.curr += 1;
            if self.curr > self.end {
                self.finished = true;
            }
        } else {
            if self.curr == 0 && self.end == 0 {
                self.finished = true;
            } else {
                self.curr -= 1;
                if self.curr < self.end {
                    self.finished = true;
                }
            }
        }
        res
    }
}

impl TreeMap {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<Tree>> = input
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.chars()
                    .map(|y| Tree::new(y.to_digit(10).unwrap() as i32))
                    .collect()
            })
            .collect();
        TreeMap { map }
    }

    fn check(&mut self, row_reverse: bool, column_reverse: bool) {
        let mut height_view = (0..self.map.get(0).unwrap().len())
            .map(|_| -1)
            .collect::<Vec<i32>>();
        let mut row_iter: Vec<Vec<&mut Tree>> = self
            .map
            .iter_mut()
            .map(|x| x.into_iter().collect())
            .collect();
        if column_reverse {
            row_iter = transpose2(row_iter);
        }
        if row_reverse {
            row_iter = row_iter.into_iter().rev().collect();
        }
        for row in row_iter {
            let mut i = 0;
            for mut item in row {
                if item.height > height_view[i] {
                    item.visible = true;
                    height_view[i] = item.height;
                }
                i += 1;
            }
        }
    }
    fn check_all(&mut self) {
        self.check(false, false);
        self.check(true, false);
        self.check(false, true);
        self.check(true, true);
    }

    fn count_visible(&self) -> usize {
        self.map
            .iter()
            .map(|x| x.iter().filter(|x| x.visible).count())
            .sum()
    }

    fn map_shape(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    fn measure_score(&self, x: usize, y: usize) -> u32 {
        let mut score = 1;
        let mut count = 0;
        let current_height = self.map[x][y].height;
        let shape = self.map_shape();
        // left
        if y > 0 {
            for i in MyRange::new(y - 1, 0) {
                if current_height > self.map[x][i].height {
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
            }
        }
        score *= count;
        count = 0;

        // right
        if y < shape.1 - 1 {
            for i in MyRange::new(y + 1, shape.1 - 1) {
                if current_height > self.map[x][i].height {
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
            }
        }
        score *= count;
        count = 0;
        // up
        if x > 0 {
            for i in MyRange::new(x - 1, 0) {
                if current_height > self.map[i][y].height {
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
            }
        }
        score *= count;
        count = 0;

        // down
        if x < shape.0 - 1 {
            for i in MyRange::new(x + 1, shape.0 - 1) {
                if current_height > self.map[i][y].height {
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
            }
        }
        score *= count;

        score
    }

    fn calculate_scores(&mut self) {
        let (n_rows, n_columns) = self.map_shape();
        for id_row in 0..n_rows {
            for id_column in 0..n_columns {
                self.map[id_row][id_column].score = self.measure_score(id_row, id_column);
            }
        }
    }

    fn get_max_score(&self) -> u32 {
        self.map
            .iter()
            .map(|x| x.iter().map(|y| y.score).max().unwrap())
            .max()
            .unwrap()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    // dbg!(&tree_map);
    let mut tree_map = TreeMap::new(input);
    tree_map.check_all();
    // dbg!(&tree_map);
    Some(tree_map.count_visible())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut tree_map = TreeMap::new(input);
    tree_map.calculate_scores();
    Some(tree_map.get_max_score())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
