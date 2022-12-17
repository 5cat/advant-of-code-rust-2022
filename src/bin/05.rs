use std::collections::HashMap;

#[derive(Debug)]
struct Instruction {
    amount: u16,
    from: u16,
    to: u16,
}

fn get_stacks(stack_raw: &str) -> HashMap<u16, Vec<char>> {
    let mut stacks = HashMap::new();
    let mut stack_iter = stack_raw.lines().rev().filter(|x| !x.is_empty());
    let first_line = stack_iter.next().unwrap();
    for c in first_line.chars() {
        if c.is_digit(10) {
            stacks.insert(c.to_digit(10).unwrap() as u16, Vec::new());
        }
    }
    for line in stack_iter {
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                let column = first_line.chars().nth(i).unwrap().to_digit(10).unwrap() as u16;
                stacks.get_mut(&column).unwrap().push(c.clone());
            }
        }
    }
    stacks
}

fn get_instructions(instructions_raw: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in instructions_raw.lines().filter(|x| !x.is_empty()) {
        let mut line_iter = line.split_whitespace();
        let instruction = Instruction {
            amount: line_iter.nth(1).unwrap().parse().unwrap(),
            from: line_iter.nth(1).unwrap().parse().unwrap(),
            to: line_iter.nth(1).unwrap().parse().unwrap(),
        };
        instructions.push(instruction);
    }
    instructions
}

fn apply_instructions(stacks: &mut HashMap<u16, Vec<char>>, instructions: Vec<Instruction>) {
    for instruction in instructions {
        for _ in 0..instruction.amount {
            let item = stacks.get_mut(&instruction.from).unwrap().pop().unwrap();
            stacks.get_mut(&instruction.to).unwrap().push(item);
        }
    }
}

fn apply_instructions_2(stacks: &mut HashMap<u16, Vec<char>>, instructions: Vec<Instruction>) {
    for instruction in instructions {
        let mut tmp_stack = Vec::new();
        for _ in 0..instruction.amount {
            let item = stacks.get_mut(&instruction.from).unwrap().pop().unwrap();
            tmp_stack.push(item);
        }
        for item in tmp_stack.iter().rev() {
            stacks.get_mut(&instruction.to).unwrap().push(*item);
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (stack_raw, instructions_raw) = input.split_once("\n\n").unwrap();
    let mut stacks = get_stacks(stack_raw);
    let instructions = get_instructions(instructions_raw);
    apply_instructions(&mut stacks, instructions);
    let mut keys_sorted = stacks.keys().collect::<Vec<_>>();
    keys_sorted.sort();
    let top_items = keys_sorted
        .iter()
        .map(|x| stacks.get(x).unwrap().last().unwrap());
    Some(top_items.collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    let (stack_raw, instructions_raw) = input.split_once("\n\n").unwrap();
    let mut stacks = get_stacks(stack_raw);
    let instructions = get_instructions(instructions_raw);
    apply_instructions_2(&mut stacks, instructions);
    let mut keys_sorted = stacks.keys().collect::<Vec<_>>();
    keys_sorted.sort();
    let top_items = keys_sorted
        .iter()
        .map(|x| stacks.get(x).unwrap().last().unwrap());
    Some(top_items.collect::<String>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
