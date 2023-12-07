pub fn solve_races(times: Vec<usize>, distances: Vec<usize>) -> usize {
    let mut result = 1.0;
    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        // Solve the quadratic equation with parameters T and D (time and distance)
        let t = time as f64;
        let d = distance as f64;

        // Center of parabola `- x^2 + tx - d`
        let x0 = t / 2.0;

        // Find lesser solution to `- x^2 + tx - d = 0`
        let discriminant = t * t - 4.0 * d;
        let x1 = (t - discriminant.sqrt()) / 2.0;

        // Check integer numbers, they change calculation a bit
        let is_x0_int = x0.fract() == 0.0;
        let is_x1_int = x1.fract() == 0.0;

        // Number of ways is (integer difference between floored x0 and ceiled x1 plus 1) * 2.
        // And minus integers if any (2 ints for x1 and 1 int for x0)
        let correction = (if is_x0_int { 1 } else { 0 }) + (if is_x1_int { 2 } else { 0 });
        let num_of_ways = (x0.floor() - x1.ceil() + 1.0) * 2.0 - correction as f64;

        dbg!(time, distance, x0, x1, x0.floor(), x1.ceil(), correction, num_of_ways);

        result *= num_of_ways;
    }

    result as usize
}
