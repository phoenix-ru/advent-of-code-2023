use common::main;
use day16::{parse_input, solve_for, Direction, Instruction};

main!();

fn solve(input: &str) -> usize {
    let nodes = parse_input(input);

    // Initial: start from top-left going right
    let start_from = Instruction {
        x: 0,
        y: 0,
        dir: Direction::Right,
    };

    solve_for(nodes, start_from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            46,
            solve(
                r".|...\....
                  |.-.\.....
                  .....|-...
                  ........|.
                  ..........
                  .........\
                  ..../.\\..
                  .-.-/..|..
                  .|....-|.\
                  ..//.|...."
            )
        );
    }
}
