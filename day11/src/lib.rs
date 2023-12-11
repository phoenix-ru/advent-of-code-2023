#[derive(Debug)]
pub struct Galaxy {
    x: usize,
    y: usize,
}

pub fn solve_with_factor(input: &str, factor: usize) -> usize {
    let mut galaxies = parse_input(input);
    shift_coordinates(&mut galaxies, factor);

    let mut answer = 0;
    let galaxies_total = galaxies.len();

    for first in 0..galaxies_total {
        for second in (first + 1)..galaxies_total {
            let first_galaxy = &galaxies[first];
            let second_galaxy = &galaxies[second];

            let distance =
                first_galaxy.x.abs_diff(second_galaxy.x) + first_galaxy.y.abs_diff(second_galaxy.y);
            answer += distance;
        }
    }

    answer
}

fn shift_coordinates(galaxies: &mut Vec<Galaxy>, factor: usize) {
    let (bound_x, bound_y) = galaxies.iter().fold((0, 0), |(max_x, max_y), current| {
        (current.x.max(max_x), current.y.max(max_y))
    });

    // This iterates lots of times, but I do not care
    // Smarter way would be to use grid
    for y in (0..bound_y).rev() {
        let has_y = galaxies.iter().any(|galaxy| galaxy.y == y);
        if !has_y {
            for galaxy in galaxies.iter_mut() {
                if galaxy.y > y {
                    galaxy.y += factor - 1;
                }
            }
        }
    }
    for x in (0..bound_x).rev() {
        let has_x = galaxies.iter().any(|galaxy| galaxy.x == x);
        if !has_x {
            for galaxy in galaxies.iter_mut() {
                if galaxy.x > x {
                    galaxy.x += factor - 1;
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Galaxy> {
    let mut galaxies = Vec::new();

    for (line_idx, line) in input.lines().enumerate() {
        let line = line.trim();

        for (char_idx, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy {
                    x: char_idx,
                    y: line_idx,
                });
            }
        }
    }

    galaxies
}
