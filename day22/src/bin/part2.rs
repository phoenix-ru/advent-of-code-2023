use std::collections::VecDeque;

use common::main;
use day22::{fall_and_compute_supports, parse_input};
use rustc_hash::FxHashSet;

main!();

fn solve(input: &str) -> usize {
    let mut blocks = parse_input(input);

    let (supports_map, supported_by_map) = fall_and_compute_supports(&mut blocks);

    // dbg!(&supports_map, &supported_by_map);

    let mut answer = 0;
    let mut queue = VecDeque::<usize>::default();
    let mut fallen_blocks = FxHashSet::<usize>::default();
    for (block_idx, _) in blocks.iter().enumerate() {
        // If block does not support anything, it's not interesting
        let Some(block_supports_these) = supports_map.get(&block_idx) else {
            continue;
        };

        // "Fall" the selected block
        fallen_blocks.insert(block_idx);

        // Only operate on nodes supported by this single block. Others would not fall.
        queue.extend(block_supports_these.iter());

        while let Some(maybe_falling_block) = queue.pop_front() {
            let Some(maybe_falling_block_supported_by) = supported_by_map.get(&maybe_falling_block) else {
                continue;
            };

            // Supported by something else, would not fall
            if maybe_falling_block_supported_by.iter().any(|support| !fallen_blocks.contains(support)) {
                continue;
            }

            // This one is certainly falling
            // println!("This is falling {maybe_falling_block}");
            fallen_blocks.insert(maybe_falling_block);

            let Some(falling_block_supports_these) = supports_map.get(&maybe_falling_block) else {
                continue;
            };

            // Trigger chain reaction
            for may_fall_next in falling_block_supports_these.iter() {
                queue.push_back(*may_fall_next);
            }
        }

        answer += fallen_blocks.len() - 1;
        fallen_blocks.clear();
    }

    answer
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(
            7,
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
