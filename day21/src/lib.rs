#[derive(Debug)]
pub enum Node {
    Available,
    Obstacle,
}

#[derive(Default)]
pub struct Input {
    pub nodes: Vec<Vec<Node>>,
    pub starting_x: usize,
    pub starting_y: usize,
}

// Where moving is allowed
pub const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn parse_input(input: &str) -> Input {
    let mut result = Input::default();

    for (y, line) in input.lines().map(str::trim).enumerate() {
        let mut new_row = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => new_row.push(Node::Available),
                '#' => new_row.push(Node::Obstacle),
                'S' => {
                    result.starting_x = x;
                    result.starting_y = y;
                    new_row.push(Node::Available);
                }
                _ => unreachable!(),
            }
        }
        result.nodes.push(new_row);
    }

    result
}
