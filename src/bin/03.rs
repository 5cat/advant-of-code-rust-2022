use std::collections::HashSet;

fn split_compartment(input: &str) -> (&str, &str) {
    let mid = input.len() / 2;
    (&input[..mid], &input[mid..])
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.lines().filter(|x| !x.is_empty()) {
        let (compartment_1, compartment_2) = split_compartment(line);
        let compartment_1_set: HashSet<_> =
            HashSet::from_iter(compartment_1.chars().filter(|x| *x != '\n'));
        let compartment_2_set: HashSet<_> =
            HashSet::from_iter(compartment_2.chars().filter(|x| *x != '\n'));
        let diff: &char = compartment_1_set
            .intersection(&compartment_2_set)
            .into_iter()
            .next()
            .unwrap();
        if diff.is_lowercase() {
            score = score + (*diff as u32 - 'a' as u32) + 1
        } else {
            score = score + (*diff as u32 - 'A' as u32) + 27
        }
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0;
    let mut group: Vec<&str> = Vec::new();
    for line in input.lines().filter(|x| !x.is_empty()) {
        group.push(line);
        if group.len() < 3 {
            continue;
        }
        let elves = group.iter()
            .map(|x| HashSet::from_iter(x.chars()))
            .collect::<Vec<HashSet<_>>>();
        group.clear();
        let first_elve = &elves[0];
        let second_elve = &elves[1];
        let third_elve = &elves[2];
        let common_1_2_elve = first_elve
            .intersection(&second_elve)
            .map(|x| *x)
            .collect::<HashSet<char>>();
        let common_clear_char = common_1_2_elve
            .intersection(&third_elve)
            .into_iter()
            .next()
            .unwrap();
        if common_clear_char.is_lowercase() {
            score = score + (common_clear_char.clone() as u32 - 'a' as u32) + 1
        } else {
            score = score + (common_clear_char.clone() as u32 - 'A' as u32) + 27
        }
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
