fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input))
}

fn solve(input: &str) -> u32 {
    let (map, (animal_x, animal_y)) = parse_input(input);

    let cycle_len = find_cycle(&map, animal_x, animal_y);
    if cycle_len % 2 == 0 {
        cycle_len / 2
    } else {
        (cycle_len + 1) / 2
    }
}

fn find_cycle(map: &Vec<Vec<MapNode>>, animal_x: usize, animal_y: usize) -> u32 {
    // Double-edged graph. Yes
    // When graph is built, find all the connections which start at `Animal`, and follow them one-by-one.
    // When following, take the node at destination index `x`, `y` and check if it's an animal. Find the next connection.
    // Store connections on the map? I think yes

    let animal = &map[animal_y][animal_x];
    assert!(animal.is_animal);

    for animal_connection in animal.connections.iter() {
        let mut old_x = animal_x;
        let mut old_y = animal_y;
        let mut x = animal_connection.x;
        let mut y = animal_connection.y;
        let mut steps_traveled = 1;

        while let Some(current_node) = map.get(y).and_then(|inner| inner.get(x)) {
            // dbg!(x, y);
            if current_node.is_animal {
                return steps_traveled;
            }

            // Ground
            if current_node.connections.is_empty() {
                break;
            }

            for connection in current_node.connections.iter() {
                if !(connection.x == old_x && connection.y == old_y) {
                    // dbg!("Suitable found", connection.x, connection.y, x, y);
                    old_x = x;
                    old_y = y;
                    x = connection.x;
                    y = connection.y;
                    steps_traveled += 1;
                    break;
                }
            }
        }
    }

    0
}

struct Coordinates {
    x: usize,
    y: usize
}

struct MapNode {
    is_animal: bool,
    connections: Vec<Coordinates>
}

impl MapNode {
    fn from_char_and_coords(x: usize, y: usize, c: char) -> Self {
        let mut connections: Vec<Coordinates> = Vec::with_capacity(2);
        let mut is_animal = false;

        macro_rules! add_connection {
            ($delta_x: literal, $delta_y: literal) => {
                if let (Some(x), Some(y)) = (x.checked_add_signed($delta_x), y.checked_add_signed($delta_y)) {
                    connections.push(Coordinates { x, y })
                }
            };
        }

        match c {
            '|' => {
                add_connection!(0, -1);
                add_connection!(0, 1);
            }
            '-' => {
                add_connection!(-1, 0);
                add_connection!(1, 0);
            }
            'L' => {
                add_connection!(0, -1);
                add_connection!(1, 0);
            }
            'J' => {
                add_connection!(0, -1);
                add_connection!(-1, 0);
            }
            '7' => {
                add_connection!(0, 1);
                add_connection!(-1, 0);
            }
            'F' => {
                add_connection!(0, 1);
                add_connection!(1, 0);
            }
            '.' => {}
            'S' => {
                is_animal = true;
                add_connection!(0, 1);
                add_connection!(0, -1);
                add_connection!(1, 0);
                add_connection!(-1, 0);
            }
            _ => unreachable!(),
        }

        MapNode { is_animal, connections }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<MapNode>>, (usize, usize)) {
    let mut result = Vec::new();

    let mut animal_x = 0;
    let mut animal_y = 0;

    for (line_idx, line) in input.lines().enumerate() {
        let line = line.trim();
        let mut line_data = Vec::with_capacity(line.len());

        for (char_idx, c) in line.chars().enumerate() {
            let node = MapNode::from_char_and_coords(char_idx, line_idx, c);
            if node.is_animal {
                animal_x = char_idx;
                animal_y = line_idx;
            }

            line_data.push(node);
        }

        result.push(line_data);
    }

    (result, (animal_x, animal_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            4,
            solve(
                ".....
                 .S-7.
                 .|.|.
                 .L-J.
                 ....."
            )
        );
        assert_eq!(
            8,
            solve(
                "..F7.
                 .FJ|.
                 SJ.L7
                 |F--J
                 LJ..."
            )
        );
    }
}
