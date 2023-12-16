use common::main;
use day16::{parse_input, solve_for, Direction, Instruction};

main!();

fn solve(input: &str) -> usize {
    let nodes = parse_input(input);
    let dim_x = nodes.first().map_or(0, |r| r.len());
    let dim_y = nodes.len();

    // Solve across x
    let max_in_x = (0..dim_x).map(|x| {
        let down = solve_for(nodes.clone(), Instruction { x, y: 0, dir: Direction::Down });
        let up = solve_for(nodes.clone(), Instruction { x, y: dim_y - 1, dir: Direction::Up });
        down.max(up)
    }).max();

    let max_in_y = (0..dim_y).map(|y| {
        let right = solve_for(nodes.clone(), Instruction { x: 0, y, dir: Direction::Right });
        let left = solve_for(nodes.clone(), Instruction { x: dim_x - 1, y, dir: Direction::Left });
        right.max(left)
    }).max();

    max_in_x.unwrap_or(0).max(max_in_y.unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            51,
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
