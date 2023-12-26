use std::collections::HashSet;

use day10::{find_cycle, parse_input, Coordinates, MapNode, MapNodeType};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input))
}

fn solve(input: &str) -> usize {
    let (mut map, (animal_x, animal_y)) = parse_input(input);

    let cycle = find_cycle(&map, animal_x, animal_y);
    count_inside(cycle, &mut map)
}

// Raycast
fn count_inside(cycle: Vec<Coordinates>, map: &mut Vec<Vec<MapNode>>) -> usize {
    // Remap animal to a correct junction
    let animal = &cycle[cycle.len() - 1];
    let next_cell = &cycle[0];
    let last_cell = &cycle[cycle.len() - 2];
    let next_diff_x = next_cell.x as isize - animal.x as isize;
    let next_diff_y = next_cell.y as isize - animal.y as isize;
    let last_diff_x = last_cell.x as isize - animal.x as isize;
    let last_diff_y = last_cell.y as isize - animal.y as isize;
    let junction_typ = match (next_diff_x, next_diff_y, last_diff_x, last_diff_y) {
        (0, -1, 0, 1) | (0, 1, 0, -1) => MapNodeType::Utd,
        (-1, 0, 1, 0) | (1, 0, -1, 0) => MapNodeType::Ltr,
        (1, 0, 0, -1) | (0, -1, 1, 0) => MapNodeType::Utd,
        (0, -1, -1, 0) | (-1, 0, 0, -1) => MapNodeType::Utl,
        (0, 1, -1, 0) | (-1, 0, 0, 1) => MapNodeType::Ltd,
        (0, 1, 1, 0) | (1, 0, 0, 1) => MapNodeType::Rtd,
        _ => unreachable!()
    };
    map[animal.y][animal.x].typ = junction_typ;

    let cycle: HashSet<(usize, usize)> = HashSet::from_iter(cycle.into_iter().map(|c| (c.x, c.y)));

    let mut total = 0;
    for (y, row) in map.iter().enumerate() {
        let mut is_inside = false;

        for (x, cell) in row.iter().enumerate() {
            let is_part_of_cycle = cycle.contains(&(x, y));

            match cell.typ {
                MapNodeType::Utd | MapNodeType::Ltd | MapNodeType::Rtd if is_part_of_cycle => {
                    is_inside = !is_inside;
                    continue;
                }
                _ if is_inside && !is_part_of_cycle => {
                    // println!("Taking {x} {y}");
                    total += 1;
                }
                _ => {}
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            4,
            solve(
                "...........
                 .S-------7.
                 .|F-----7|.
                 .||.....||.
                 .||.....||.
                 .|L-7.F-J|.
                 .|..|.|..|.
                 .L--J.L--J.
                 ..........."
            )
        );
        assert_eq!(
            8,
            solve(
                ".F----7F7F7F7F-7....
                 .|F--7||||||||FJ....
                 .||.FJ||||||||L7....
                 FJL7L7LJLJ||LJ.L-7..
                 L--J.L7...LJS7F-7L7.
                 ....F-J..F7FJ|L7L7L7
                 ....L7.F7||L7|.L7L7|
                 .....|FJLJ|FJ|F7|.LJ
                 ....FJL-7.||.||||...
                 ....L---J.LJ.LJLJ..."
            )
        );
        assert_eq!(
            10,
            solve(
                "FF7FSF7F7F7F7F7F---7
                 L|LJ||||||||||||F--J
                 FL-7LJLJ||||||LJL-77
                 F--JF--7||LJLJ7F7FJ-
                 L---JF-JLJ.||-FJLJJ7
                 |F|F-JF---7F7-L7L|7|
                 |FFJF7L7F-JF7|JL---7
                 7-L-JL7||F7|L7F-7F7|
                 L.L7LFJ|||||FJL7||LJ
                 L7JLJL-JLJLJL--JLJ.L"
            )
        );
    }
}
