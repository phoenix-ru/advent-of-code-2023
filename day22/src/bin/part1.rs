use common::main;
use day22::{fall_and_compute_supports, parse_input};

main!();

fn solve(input: &str) -> usize {
    let mut blocks = parse_input(input);

    let (supports_map, supported_by_map) = fall_and_compute_supports(&mut blocks);

    // dbg!(&supports_map, &supported_by_map);

    blocks
        .iter()
        .enumerate()
        .filter(|(block_idx, _block)| {
            match supports_map.get(block_idx) {
                // All nodes it supports have other supports
                Some(v) => v.iter().all(|id| {
                    supported_by_map
                        .get(id)
                        .map_or(false, |supported_by| supported_by.len() > 1)
                }),
                None => true,
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            5,
            solve(
                "1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9"
            ),
        );
    }
}
