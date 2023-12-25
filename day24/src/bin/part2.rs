use common::main;
use day24::{intersect, parse_input, Line};
use smallvec::SmallVec;

main!();

fn solve(input: &str) -> usize {
    let lines = parse_input(input);
    solve_for(&lines)
}

fn solve_for(lines: &[Line]) -> usize {
    let mut lines_to_check = SmallVec::<[&Line; 3]>::new();

    // Add non-parallel lines
    for line in lines.iter() {
        if lines_to_check
            .iter()
            .any(|l| l.m_y == line.m_y || l.m_z == line.m_z)
        {
            println!("Skipping line {:?}", line);
            continue;
        }

        lines_to_check.push(line);

        // We need just 3
        if lines_to_check.len() == 3 {
            break;
        }
    }

    // 4 non-parallel to each other lines
    // One is extra for checking
    dbg!(&lines_to_check);
    assert_eq!(3, lines_to_check.len());

    // Brute-force the normalizing vector which makes lines
    // Take the max absolute value of vector
    let max_v = lines
        .iter()
        .fold(0.0f64, |acc, line| {
            acc.max(line.dir.0.abs())
                .max(line.dir.1.abs())
                .max(line.dir.2.abs())
        })
        .round() as isize;

    // Cached variables
    let mut line1 = lines_to_check.pop().unwrap().to_owned();
    let mut line2 = lines_to_check.pop().unwrap().to_owned();
    let mut line3 = lines_to_check.pop().unwrap().to_owned();
    let mut intersect_at = (0.0f64, 0.0f64, 0.0f64);

    // Margin of error
    let delta_error = line1.x.log10();

    dbg!(-max_v..max_v);

    // Brute-force
    'outer: for v_x in -max_v..max_v {
        for v_y in -max_v..max_v {
            for v_z in -max_v..max_v {
                // println!("Checking {v_x} {v_y} {v_z}");
                // Debug for half
                if v_x == 0 && v_y == 0 && v_z == 0 {
                    println!("Half-done")
                }

                // Try correcting by these values
                let (v_x, v_y, v_z) = (v_x as f64, v_y as f64, v_z as f64);
                line1.m_y = (line1.dir.1 - v_y) / (line1.dir.0 - v_x);
                line1.m_z = (line1.dir.2 - v_z) / (line1.dir.0 - v_x);

                line2.m_y = (line2.dir.1 - v_y) / (line2.dir.0 - v_x);
                line2.m_z = (line2.dir.2 - v_z) / (line2.dir.0 - v_x);

                line3.m_y = (line3.dir.1 - v_y) / (line3.dir.0 - v_x);
                line3.m_z = (line3.dir.2 - v_z) / (line3.dir.0 - v_x);

                let Some(int1_2) = intersect(&line1, &line2) else {
                    continue;
                };

                let Some(int1_3) = intersect(&line1, &line3) else {
                    continue;
                };

                let Some(int2_3) = intersect(&line2, &line3) else {
                    continue;
                };

                macro_rules! is_eq {
                    ($first: ident, $second: ident) => {
                        ($first.0 - $second.0).abs() <= delta_error
                            && ($first.1 - $second.1).abs() <= delta_error
                            && ($first.2 - $second.2).abs() <= delta_error
                    };
                }

                // Intersection points
                if is_eq!(int1_2, int1_3) && is_eq!(int1_2, int2_3) {
                    // dbg!(v_x, v_y, v_z);
                    // dbg!(int1_2, int1_3, int2_3);

                    // Check first line against all others
                    let mut lines_to_check = lines.to_owned();
                    let is_ok = lines_to_check.iter_mut().all(|line| {
                        // Adjust
                        line.m_y = (line.dir.1 - v_y) / (line.dir.0 - v_x);
                        line.m_z = (line.dir.2 - v_z) / (line.dir.0 - v_x);

                        // Intersect and skip if parallel
                        let Some(int) = intersect(&line1, &line) else {
                            return true;
                        };

                        is_eq!(int1_2, int)
                    });

                    if is_ok {
                        intersect_at = int1_2;
                        break 'outer;
                    }
                }
            }
        }
    }

    dbg!(intersect_at);

    (intersect_at.0 + intersect_at.1 + intersect_at.2).round() as usize
}

#[cfg(test)]
mod tests {
    use crate::solve_for;
    use day24::parse_input;

    #[test]
    fn it_works() {
        assert_eq!(
            47,
            solve_for(&parse_input(
                "19, 13, 30 @ -2,  1, -2
                 18, 19, 22 @ -1, -1, -2
                 20, 25, 34 @ -2, -2, -4
                 12, 31, 28 @ -1, -2, -1
                 20, 19, 15 @  1, -5, -3"
            )),
        );
    }
}
