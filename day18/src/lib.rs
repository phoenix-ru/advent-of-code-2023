#[derive(Debug)]
pub struct Instruction {
    pub dir: Direction,
    pub steps: isize,
}

#[derive(Debug)]
pub enum Direction {
    X,
    Y,
}

pub fn solve_for(instructions: &[Instruction]) -> usize {
    let mut curr_x = 0;
    let mut curr_y = 0;
    let mut perimeter: usize = 0;
    let mut points: Vec<(isize, isize)> = Vec::with_capacity(instructions.len());

    for instruction in instructions {
        perimeter += instruction.steps.abs() as usize;

        match instruction.dir {
            Direction::X => {
                curr_x += instruction.steps;
            }
            Direction::Y => {
                curr_y += instruction.steps;
            }
        }
        points.push((curr_x, curr_y));
    }

    // Calculate shoelace plus perimeter and compensate by a duplicated border
    let shoelace_area = shoelace(&points);
    let full_area = shoelace_area + (perimeter / 2) + 1;

    full_area
}

fn shoelace(points: &[(isize, isize)]) -> usize {
    let mut sum1: isize = 0;
    let mut sum2: isize = 0;

    for window in points.windows(2) {
        let p1 = window[0];
        let p2 = window[1];
        sum1 += p1.0 * p2.1;
        sum2 += p1.1 * p2.0;
    }

    sum1.abs_diff(sum2) / 2
}
