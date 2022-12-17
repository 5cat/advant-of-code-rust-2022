fn is_inside(x: (u32, u32), y: (u32, u32)) -> bool {
    x.0 <= y.0 && x.1 >= y.1
}

fn is_partially_inside(x: (u32, u32), y: (u32, u32)) -> bool {
    x.0 <= y.0 && y.0 <= x.1
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = input.lines().filter(|x| !x.is_empty()).map(|x| {
        let mut x_t = x.split(',')
            .map(|y| {
                let (y_1, y_2) = y.split_once('-').unwrap();
                (y_1.parse::<u32>().unwrap(), y_2.parse::<u32>().unwrap())
            }).into_iter();
        (x_t.next().unwrap(), x_t.next().unwrap())
    }).collect::<Vec<_>>();
    let mut count = 0;
    for (pair_1, pair_2) in data {
        if is_inside(pair_1, pair_2) || is_inside(pair_2, pair_1) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = input.lines().filter(|x| !x.is_empty()).map(|x| {
        let mut x_t = x.split(',')
            .map(|y| {
                let (y_1, y_2) = y.split_once('-').unwrap();
                (y_1.parse::<u32>().unwrap(), y_2.parse::<u32>().unwrap())
            }).into_iter();
        (x_t.next().unwrap(), x_t.next().unwrap())
    }).collect::<Vec<_>>();
    let mut count = 0;
    for (pair_1, pair_2) in data {
        if is_partially_inside(pair_1, pair_2) || is_partially_inside(pair_2, pair_1) {
            count += 1;
        }
    }
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
