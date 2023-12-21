use common::main;
use day20::{parse_input, cycle};


main!();

fn solve(input: &str) -> usize {
    let gates = parse_input(input);
    cycle(gates, 10000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            32000000,
            solve(
                "broadcaster -> a, b, c
                %a -> b
                %b -> c
                %c -> inv
                &inv -> a"
            )
        );

        assert_eq!(
            11687500,
            solve(
                "broadcaster -> a
                %a -> inv, con
                &inv -> b
                %b -> con
                &con -> output"
            )
        );
    }
}
