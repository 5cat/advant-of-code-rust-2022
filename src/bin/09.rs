use std::{
    cmp::{max, min},
    collections::HashSet,
    fmt::Debug,
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Motion {
    direction: Direction,
    step: u32,
}

fn parse_input(input: &str) -> Vec<Motion> {
    let mut moves = vec![];
    for line in input.split('\n').filter(|x| !x.is_empty()) {
        let (direction_str, n_moves_str) = line.split_once(' ').unwrap();
        let n_moves = n_moves_str.parse::<u32>().unwrap();
        let direciton_i = match direction_str {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => todo!(),
        };
        let motion = Motion {
            direction: direciton_i,
            step: n_moves,
        };
        moves.push(motion);
    }
    moves
}

struct Board {
    visited: HashSet<(i32, i32)>,
    width_range: (i32, i32),
    height_range: (i32, i32),
    starting_point: (i32, i32),
    head: (i32, i32),
    tails: Vec<(i32, i32)>,
}

fn clip_int(v: i32) -> i32 {
    min(max(v, -1), 1)
}

impl Board {
    fn new(tails: u32) -> Self {
        let mut visited = HashSet::new();
        let starting_point = (0, 0);
        visited.insert(starting_point);
        Board {
            visited,
            starting_point,
            width_range: (min(0, starting_point.0), max(0, starting_point.0)),
            height_range: (min(0, starting_point.1), max(0, starting_point.1)),
            head: starting_point,
            tails: (0..tails).map(|_| starting_point).collect(),
        }
    }

    fn apply(&mut self, m: Motion) {
        for _ in 0..m.step {
            self.head = match m.direction {
                Direction::Up => (self.head.0 + 1, self.head.1),
                Direction::Down => (self.head.0 - 1, self.head.1),
                Direction::Left => (self.head.0, self.head.1 - 1),
                Direction::Right => (self.head.0, self.head.1 + 1),
            };
            self.height_range = (
                min(self.height_range.0, self.head.0),
                max(self.height_range.1, self.head.0),
            );
            self.width_range = (
                min(self.width_range.0, self.head.1),
                max(self.width_range.1, self.head.1),
            );
            let mut current_head = self.head;
            for i in 0..self.tails.len() {
                self.tails[i] = self.refresh_tail(current_head, self.tails[i]);
                current_head = self.tails[i];
            }
            self.visited.insert(*self.tails.last().unwrap());
        }
    }

    fn refresh_tail(&mut self, head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
        let diff = (head.0 - tail.0, head.1 - tail.1);
        if diff.0.abs() <= 1 && diff.1.abs() <= 1 {
            return tail;
        }
        let diff_clipped = (clip_int(diff.0), clip_int(diff.1));
        let new_tail = (tail.0 + diff_clipped.0, tail.1 + diff_clipped.1);
        new_tail
    }

    fn print_board(&self) {
        let mut board: Vec<Vec<String>> = (self.height_range.0..self.height_range.1 + 1)
            .map(|_| {
                (self.width_range.0..self.width_range.1 + 1)
                    .map(|_| ".".to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for (x, y) in self.visited.iter() {
            board[(x - self.height_range.0) as usize][(y - self.width_range.0) as usize] =
                "#".to_string();
        }
        board[(self.starting_point.0 - self.height_range.0) as usize]
            [(self.starting_point.0 - self.width_range.0) as usize] = "s".to_string();
        for i in 0..self.tails.len() {
            let i_fixed = i + 1;
            let tail_str = format!("{}", i_fixed);
            board[(self.tails[i].0 - self.height_range.0) as usize]
                [(self.tails[i].1 - self.width_range.0) as usize] = tail_str;
        }
        board[(self.head.0 - self.height_range.0) as usize]
            [(self.head.1 - self.width_range.0) as usize] = "H".to_string();
        println!(
            "{}",
            board
                .iter()
                .rev()
                .map(|x| x.join(""))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    fn get_len_visited(&self) -> usize {
        self.visited.len()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let moves = parse_input(input);
    let mut board = Board::new(1);
    for motion in moves {
        board.apply(motion);
    }
    Some(board.get_len_visited())
}

pub fn part_two(input: &str) -> Option<usize> {
    let moves = parse_input(input);
    let mut board = Board::new(9);
    for motion in moves {
        board.apply(motion);
    }
    Some(board.get_len_visited())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
