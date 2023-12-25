use common::main;
use day24::{Line, parse_input, intersect};

main!();

fn solve(input: &str) -> usize {
    let lines = parse_input(input);
    solve_for(&lines, 200000000000000.0, 400000000000000.0)
}

fn solve_for(lines: &[Line], lower_bound: f64, upper_bound: f64) -> usize {
    let mut total_intersect = 0;
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            let line1 = &lines[i];
            let line2 = &lines[j];
            let Some((x_int, y_int, _z_int)) = intersect(line1, line2) else {
                continue;
            };

            // Check if in the past, these should be discarded
            let is_past =
                (x_int - line1.x) / line1.dir.0 < 0.0 || (x_int - line2.x) / line2.dir.0 < 0.0;

            if !is_past
                && x_int >= lower_bound
                && x_int <= upper_bound
                && y_int >= lower_bound
                && y_int <= upper_bound
            {
                // println!("Lines {i} and {j} intersect at {x_int} {y_int} {z_int}");
                total_intersect += 1;
            }
        }
    }

    total_intersect
}

#[cfg(test)]
mod tests {
    use day24::parse_input;
    use crate::solve_for;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            solve_for(
                &parse_input(
                    "19, 13, 30 @ -2,  1, -2
                     18, 19, 22 @ -1, -1, -2
                     20, 25, 34 @ -2, -2, -4
                     12, 31, 28 @ -1, -2, -1
                     20, 19, 15 @  1, -5, -3"
                ),
                7.0,
                27.0
            ),
        );

        // Normalized, all hit the same spot 24 13
        assert_eq!(
            3 + 3 + 2 + 1,
            solve_for(
                &parse_input(
                    "19, 13, 30 @ 1, 0, -4
                     18, 19, 22 @ 2, -2, -4
                     20, 25, 34 @ 1, -3, -6
                     12, 31, 28 @ 2, -3, -3
                     20, 19, 15 @ 4, -6, -5"
                ),
                7.0,
                27.0
            ),
        );

        // Normalized, all hit the same spot 24 13
        assert_eq!(
            3 + 3 + 2 + 1,
            solve_for(
                &parse_input(
                    "19, 13, 30 @  2, -3, -3
                     18, 19, 22 @  3, -5, -3
                     20, 25, 34 @  2, -6, -5
                     12, 31, 28 @  3, -6, -2
                     20, 19, 15 @  5, -9, -4"
                ),
                0.0,
                270.0
            ),
        );
    }
}
