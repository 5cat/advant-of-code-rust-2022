use std::collections::HashSet;

fn naive(input: &str, start_of_packet_size: usize) -> Option<u32> {
    let mut found: Option<u32> = None;
    for index in 0..input.len() - start_of_packet_size {
        let window = &input[index..index + start_of_packet_size];
        if window.chars().collect::<HashSet<_>>().len() == start_of_packet_size {
            found = Some(index as u32 + start_of_packet_size as u32);
            dbg!(window);
            break;
        }
    }
    found
}

pub fn part_one(input: &str) -> Option<u32> {
    naive(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    naive(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
