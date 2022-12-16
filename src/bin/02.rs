#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Sicssors,
}

fn convert_to_move(m: &str) -> Move {
    match m {
        "X" | "A" => Move::Rock,
        "Y" | "B" => Move::Paper,
        "Z" | "C" => Move::Sicssors,
        &_ => todo!(),
    }
}

fn score(me: &Move, op: &Move) -> u32 {
    let move_value = match me {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Sicssors => 3,
    };

    let mut match_result = match (&me, &op) {
        (Move::Rock, Move::Sicssors) => 6,
        (Move::Sicssors, Move::Paper) => 6,
        (Move::Paper, Move::Rock) => 6,
        _ => 0,
    };

    if me == op {
        match_result = match_result + 3;
    }

    move_value + match_result
}

pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split(" ").map(convert_to_move).collect::<Vec<_>>())
        .map(|x| score(&x[1], &x[0]))
        .sum();
    Some(score)
}

fn convert_to_wanted_move(op: &Move, wanted: &Move) -> Move {
    match wanted {
        // please lose
        Move::Rock => match op {
            Move::Rock => Move::Sicssors,
            Move::Paper => Move::Rock,
            Move::Sicssors => Move::Paper,
        },
        Move::Paper => (*op).clone(),
        Move::Sicssors => match op {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Sicssors,
            Move::Sicssors => Move::Rock,
        },
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split(" ").map(convert_to_move).collect::<Vec<_>>())
        .map(|x| score(&convert_to_wanted_move(&x[0], &x[1]), &x[0]))
        .sum();
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
