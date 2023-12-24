use common::main;
use day23::{parse_input, RecursiveSolver, START, Node};

main!();

fn solve(input: &str) -> usize {
    let mut map = parse_input(input);

    // Part2 ignores slopes
    for row in map.iter_mut() {
        for node in row.iter_mut() {
            match node {
                Node::Obstacle | Node::Empty => {}
                _ => *node = Node::Empty
            }
        }
    }

    let solver = RecursiveSolver::new(map, true);
    solver.solve_recursive_smart(START, vec![])
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            154,
            solve(
                "#.#####################
                 #.......#########...###
                 #######.#########.#.###
                 ###.....#.>.>.###.#.###
                 ###v#####.#v#.###.#.###
                 ###.>...#.#.#.....#...#
                 ###v###.#.#.#########.#
                 ###...#.#.#.......#...#
                 #####.#.#.#######.#.###
                 #.....#.#.#.......#...#
                 #.#####.#.#.#########v#
                 #.#...#...#...###...>.#
                 #.#.#v#######v###.###v#
                 #...#.>.#...>.>.#.###.#
                 #####v#.#.###v#.#.###.#
                 #.....#...#...#.#.#...#
                 #.#########.###.#.#.###
                 #...###...#...#...#.###
                 ###.###.#.###v#####v###
                 #...#...#.#.>.>.#.>.###
                 #.###.###.#.###.#.#v###
                 #.....###...###...#...#
                 #####################.#"
            ),
        );
    }
}
