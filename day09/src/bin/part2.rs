use day09::parse_input;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input))
}

fn solve(input: &str) -> isize {
    let tasks = parse_input(input);
    let mut answer = 0;

    for task in tasks {
        answer += solve_sequence(&task);
    }

    answer
}

/// Predict the previous number
fn solve_sequence(sequence: &[isize]) -> isize {
    if sequence.iter().all(|i| *i == 0) {
        return 0;
    }

    let first_el = sequence.first().unwrap();
    let diff_sequence: Vec<isize> = sequence.windows(2).map(|window| window[1] - window[0]).collect();
    let value = solve_sequence(&diff_sequence);

    *first_el - value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            solve("0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45")
        )
    }
}
