pub fn part_one(input: &str) -> Option<u32> {
    let elves_string = input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter(|&y| !y.is_empty())
                .map(|y| y.parse::<f32>().unwrap())
                .sum::<f32>() as u32
        })
        .max();
    elves_string
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves_string = input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter(|&y| !y.is_empty())
                .map(|y| y.parse::<f32>().unwrap())
                .sum::<f32>() as u32
        })
        .collect::<Vec<u32>>();
    elves_string.sort();
    let top_3_elves = elves_string
        .iter()
        .rev()
        .take(3)
        .map(|x| x.clone() as f32)
        .sum::<f32>() as u32;
    Some(top_3_elves)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
