use day12::{State, Solver};
use common::main;

main!();

fn solve(input: &str) -> usize {
    let mut data: Vec<Vec<State>> = Vec::new();
    let mut nums: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        let (cells, numbers) = line.split_once(' ').unwrap();
        data.push(cells.chars().map(|c| State::from(c)).collect());
        nums.push(numbers.split(',').map(|n| n.parse().unwrap()).collect());
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
        assert_eq!(4, solve(".??..??...?##. 1,1,3"));
        assert_eq!(1, solve("?#?#?#?#?#?#?#? 1,3,1,6"));
        assert_eq!(1, solve("????.#...#... 4,1,1"));
        assert_eq!(4, solve("????.######..#####. 1,6,5"));
        assert_eq!(10, solve("?###???????? 3,2,1"));
    }
}
