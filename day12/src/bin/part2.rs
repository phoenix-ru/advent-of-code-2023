use common::main;
use day12::{State, Solver};

main!();

fn solve(input: &str) -> usize {
    let mut data: Vec<Vec<State>> = Vec::new();
    let mut nums: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        let (cells, numbers) = line.split_once(' ').unwrap();

        // Part2: duplicate input
        let mut new_cells = String::with_capacity(cells.len() * 5);
        for idx in 0..5 {
            if idx != 0 {
                new_cells.push('?');
            }
            new_cells.push_str(cells);
        }
        let mut new_numbers = String::with_capacity(numbers.len() * 5);
        for idx in 0..5 {
            if idx != 0 {
                new_numbers.push(',');
            }
            new_numbers.push_str(numbers);
        }

        let cells: Vec<State> = new_cells.chars().map(|c| State::from(c)).collect();
        let numbers: Vec<usize> = new_numbers.split(',').map(|n| n.parse().unwrap()).collect();
        data.push(cells);
        nums.push(numbers);
    }

    let mut sum = 0;
    for (states, nums) in data.iter().zip(nums.iter()) {
        let mut solver = Solver::default();
        sum += solver.solve_dynamic(&states, 0, &nums);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, solve("???.### 1,1,3"));
        assert_eq!(16384, solve(".??..??...?##. 1,1,3"));
        assert_eq!(1, solve("?#?#?#?#?#?#?#? 1,3,1,6"));
        assert_eq!(16, solve("????.#...#... 4,1,1"));
        assert_eq!(2500, solve("????.######..#####. 1,6,5"));
        assert_eq!(506250, solve("?###???????? 3,2,1"));
    }
}
