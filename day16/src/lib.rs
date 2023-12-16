use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Direction {
    Up = 1,
    Down = 2,
    Right = 4,
    Left = 8,
}

pub struct Instruction {
    pub x: usize,
    pub y: usize,
    pub dir: Direction,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    /// `|`
    SplitterVertical,
    /// `-`
    SplitterHorizontal,
    /// /
    MirrorForward,
    /// `\`
    MirrorBackward,
    /// `.`
    EmptySpace,
}

#[derive(Debug, Clone)]
pub struct Node {
    typ: NodeType,
    energized: bool,
    visited_from: u8
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        let typ: NodeType = match value {
            '|' => NodeType::SplitterVertical,
            '-' => NodeType::SplitterHorizontal,
            '/' => NodeType::MirrorForward,
            '\\' => NodeType::MirrorBackward,
            '.' => NodeType::EmptySpace,
            _ => unreachable!(),
        };

        Self {
            typ,
            energized: false,
            visited_from: 0
        }
    }
}

pub fn solve_for(mut nodes: Vec<Vec<Node>>, initial: Instruction) -> usize {
    let mut queue = VecDeque::<Instruction>::new();
    queue.push_back(initial);

    // Dimensions for checks
    let dim_x = nodes.first().map_or(0, |v| v.len());
    let dim_y = nodes.len();

    // Follow the path
    while let Some(next_instruction) = queue.pop_front() {
        // Get item at path and check that it was already visited
        let curr_x = next_instruction.x;
        let curr_y = next_instruction.y;
        let direction_from = next_instruction.dir as u8;
        let node = &mut nodes[curr_y][curr_x];
        if node.visited_from & direction_from != 0 {
            continue;
        }
        node.energized = true;
        node.visited_from |= direction_from;

        macro_rules! next {
            ($dir: ident, $delta_x: expr, $delta_y: expr) => {
                if !(curr_x == 0 && $delta_x == -1
                    || curr_y == 0 && $delta_y == -1
                    || curr_x + 1 == dim_x && $delta_x == 1
                    || curr_y + 1 == dim_y && $delta_y == 1)
                {
                    let x = curr_x.wrapping_add_signed($delta_x);
                    let y = curr_y.wrapping_add_signed($delta_y);
                    queue.push_back(Instruction {
                        x,
                        y,
                        dir: Direction::$dir,
                    })
                }
            };
        }

        match (&node.typ, next_instruction.dir) {
            (NodeType::SplitterVertical, Direction::Up) => next!(Up, 0, -1),
            (NodeType::SplitterVertical, Direction::Down) => next!(Down, 0, 1),
            (NodeType::SplitterVertical, Direction::Right | Direction::Left) => {
                next!(Up, 0, -1);
                next!(Down, 0, 1);
            }
            (NodeType::SplitterHorizontal, Direction::Up | Direction::Down) => {
                next!(Right, 1, 0);
                next!(Left, -1, 0);
            }
            (NodeType::SplitterHorizontal, Direction::Right) => next!(Right, 1, 0),
            (NodeType::SplitterHorizontal, Direction::Left) => next!(Left, -1, 0),
            (NodeType::MirrorForward, Direction::Up) => next!(Right, 1, 0),
            (NodeType::MirrorForward, Direction::Down) => next!(Left, -1, 0),
            (NodeType::MirrorForward, Direction::Right) => next!(Up, 0, -1),
            (NodeType::MirrorForward, Direction::Left) => next!(Down, 0, 1),
            (NodeType::MirrorBackward, Direction::Up) => next!(Left, -1, 0),
            (NodeType::MirrorBackward, Direction::Down) => next!(Right, 1, 0),
            (NodeType::MirrorBackward, Direction::Right) => next!(Down, 0, 1),
            (NodeType::MirrorBackward, Direction::Left) => next!(Up, 0, -1),
            (NodeType::EmptySpace, Direction::Up) => next!(Up, 0, -1),
            (NodeType::EmptySpace, Direction::Down) => next!(Down, 0, 1),
            (NodeType::EmptySpace, Direction::Right) => next!(Right, 1, 0),
            (NodeType::EmptySpace, Direction::Left) => next!(Left, -1, 0),
        }

        // Debug
        // for row in nodes.iter() {
        //     for node in row.iter() {
        //         print!("{}", if node.energized { '#' } else { '.' })
        //     }
        //     println!();
        // }
        // println!()
    }

    // Compute the answer as a count of `energized` Nodes
    nodes
        .iter()
        .map(|row| row.iter().filter(|n| n.energized).count())
        .sum()
} 

pub fn parse_input(input: &str) -> Vec<Vec<Node>> {
    let mut result = Vec::new();

    for line in input.lines().map(str::trim) {
        let mut line_nodes: Vec<Node> = Vec::with_capacity(line.len());
        for c in line.chars() {
            line_nodes.push(c.into())
        }
        result.push(line_nodes);
    }

    result
}
