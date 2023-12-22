use rustc_hash::FxHashMap;

pub type Coord = (usize, usize, usize);
pub type SupportMap = FxHashMap::<usize, Vec<usize>>;

// Minimum level at where the blocks stop falling
const MIN_Z: usize = 1;

#[derive(Debug)]
pub struct Block {
    pub start_coord: Coord,
    pub end_coord: Coord,
}

/// Returns `supports_map` and `supported_by_map`. First shows what a node supports,
/// second shows what a node is supported by.
pub fn fall_and_compute_supports(blocks: &mut Vec<Block>) -> (SupportMap, SupportMap) {
    // Sort blocks. It doesn't seem to work otherwise
    blocks.sort_by(|a, b| {
        let a_z = a.start_coord.2.min(a.end_coord.2);
        let b_z = b.start_coord.2.min(b.end_coord.2);
        a_z.cmp(&b_z)
    });

    // Build a map of (x, y, z) -> idx
    // It will be heavily updated as blocks are falling
    let mut blocks_map = FxHashMap::<Coord, usize>::default();

    // Build a map of supports. Idx of node -> what nodes it supports
    let mut supports_map = SupportMap::default();

    // Build a map of supported by. Idx of node -> what node supports it
    let mut supported_by_map = SupportMap::default();

    for (block_idx, block) in blocks.iter_mut().enumerate() {
        // Check supports for each x and y coordinate, lower z
        let starting_z = block.start_coord.2.min(block.end_coord.2);
        let mut current_z = starting_z;

        while current_z > MIN_Z {
            // Check support for each x and y
            let mut is_supported = false;
            for x in block.start_coord.0..=block.end_coord.0 {
                for y in block.start_coord.1..=block.end_coord.1 {
                    // dbg!(x, y, current_z);
                    if let Some(idx) = blocks_map.get(&(x, y, current_z - 1)) {
                        is_supported = true;

                        // Update supports map of block `idx` (the one which supports)
                        let entry = supports_map.entry(*idx).or_insert_with(|| vec![]);
                        if !entry.contains(&block_idx) {
                            entry.push(block_idx);
                        }

                        // Update supports map of block `idx` (the one which supports)
                        let entry = supported_by_map.entry(block_idx).or_insert_with(|| vec![]);
                        if !entry.contains(&idx) {
                            entry.push(*idx);
                        }
                    }
                }
            }

            if !is_supported {
                current_z -= 1;
            } else {
                break;
            }
        }

        let z_diff = starting_z - current_z;
        block.start_coord.2 -= z_diff;
        block.end_coord.2 -= z_diff;

        // println!(
        //     "Inserting {block_idx} at {:?} - {:?}",
        //     block.start_coord, block.end_coord
        // );
        // dbg!()

        // Insert cubes
        for x in block.start_coord.0..=block.end_coord.0 {
            for y in block.start_coord.1..=block.end_coord.1 {
                for z in block.start_coord.2..=block.end_coord.2 {
                    // println!("Inserting {block_idx} at {x} {y} {z}");
                    blocks_map.insert((x, y, z), block_idx);
                }
            }
        }
    }

    (supports_map, supported_by_map)
}

pub fn parse_input(input: &str) -> Vec<Block> {
    let mut result = Vec::<Block>::new();

    for line in input.lines().map(str::trim) {
        let (start_raw, end_raw) = line.split_once('~').unwrap();
        let start_coord = parse_coord(start_raw);
        let end_coord = parse_coord(end_raw);
        result.push(Block {
            start_coord,
            end_coord,
        })
    }

    result
}

fn parse_coord(input: &str) -> Coord {
    let mut split = input.split(',');
    let x = split.next().unwrap().parse().unwrap();
    let y = split.next().unwrap().parse().unwrap();
    let z = split.next().unwrap().parse().unwrap();
    (x, y, z)
}
