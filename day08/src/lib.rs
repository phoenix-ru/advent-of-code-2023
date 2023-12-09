/// Converts a three-letter sequence into a number
#[inline]
pub fn get_idx(seq: &str) -> usize {
    usize::from_str_radix(seq, 36).unwrap()
}

/// Maximum index of a radix 36 number with three letters
pub const fn max_idx() -> usize {
    36 * 36 * 36
}

pub enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Connections {
    pub left: usize,
    pub right: usize
}

pub fn parse_input(input: &str) -> (Box<[Connections; max_idx()]>, Vec<Instruction>) {
    let mut lines = input.lines();
    let instructions = parse_instructions(lines.next().unwrap());
    lines.next();

    // Map of node connections (~730KB)
    let mut map = Box::new([Connections::default(); max_idx()]);

    for line in lines {
        let line = line.trim();
        let (source, dests) = line.split_once(" = ").expect("Should split");
        let dests = &dests[1..dests.len()-1];
        let (dest_l, dest_r) = dests.split_once(", ").expect("Should split");

        // Add connections
        let src_idx = get_idx(source);
        let dest_l_idx = get_idx(dest_l);
        let dest_r_idx = get_idx(dest_r);
        let item = &mut map[src_idx];
        item.left = dest_l_idx;
        item.right = dest_r_idx;
    }

    (map, instructions)
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut result = Vec::with_capacity(input.len());

    for c in input.chars() {
        result.push(if c == 'L' {
            Instruction::Left
        } else {
            Instruction::Right
        });
    }

    result
}
