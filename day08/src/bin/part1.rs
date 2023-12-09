use day08::{get_idx, parse_input, Connections, Instruction};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input))
}

fn solve(input: &str) -> u32 {
    let (map, instructions) = parse_input(input);
    follow_instructions(&*map, &instructions)
}

fn follow_instructions(map: &[Connections], instructions: &[Instruction]) -> u32 {
    let target = get_idx("ZZZ");

    let mut steps_taken = 0;
    let mut current = get_idx("AAA");
    for instruction in instructions.iter().cycle() {
        if current == target {
            break;
        }

        let current_connection = &map[current];
        match instruction {
            Instruction::Left => current = current_connection.left,
            Instruction::Right => current = current_connection.right,
        }

        steps_taken += 1;
    }

    steps_taken
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            solve(
                "RL

                AAA = (BBB, CCC)
                BBB = (DDD, EEE)
                CCC = (ZZZ, GGG)
                DDD = (DDD, DDD)
                EEE = (EEE, EEE)
                GGG = (GGG, GGG)
                ZZZ = (ZZZ, ZZZ)"
            )
        )
    }

    #[test]
    fn it_cycles() {
        assert_eq!(
            6,
            solve(
                "LLR

                AAA = (BBB, BBB)
                BBB = (AAA, ZZZ)
                ZZZ = (ZZZ, ZZZ)"
            )
        )
    }
}
