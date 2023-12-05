use std::ops::Range;

use day05::{parse_maps, solve_for};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let seeds = parse_seeds(lines.next().expect("Must have seeds"));
    let maps = parse_maps(lines);

    solve_for(seeds, maps)
}

#[derive(Default)]
struct RangedSeedsIterator {
    ranges: Vec<Range<usize>>,
    current_range: Range<usize>,
    current_range_idx: usize,
}

/// Seeds parser for part2.
/// This is memory-heavy for the given input
fn parse_seeds<'i>(line: &str) -> RangedSeedsIterator {
    const HEADER: &'static str = "seeds: ";

    assert!(
        line.starts_with("seeds: "),
        "Does not start with `{HEADER}`"
    );

    let line = &line[HEADER.len()..];

    let raw_nums: Vec<usize> = line.split(' ').map(|num| num.parse().unwrap()).collect();

    // Part 2 is pairs (seed_start, length)
    let mut result = RangedSeedsIterator::default();
    let mut raw_nums_iter = raw_nums.into_iter();
    while let (Some(seed_start), Some(length)) = (raw_nums_iter.next(), raw_nums_iter.next()) {
        println!("Adding seed range {seed_start} {}", seed_start + length);
        result.add_range(seed_start, seed_start + length);
    }

    result
}

impl RangedSeedsIterator {
    pub fn add_range(&mut self, range_start: usize, range_end: usize) {
        // Set this range as current if it is the first
        if self.ranges.is_empty() {
            self.current_range = range_start..range_end;
        }

        self.ranges.push(range_start..range_end);
    }
}

impl Iterator for RangedSeedsIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // Try current range
        let next_in_current_range = self.current_range.next();
        if next_in_current_range.is_some() {
            return next_in_current_range;
        }

        // Try creating next range
        self.current_range_idx += 1;
        if let Some(next_range) = self.ranges.get(self.current_range_idx) {
            self.current_range = next_range.to_owned();
        }

        self.current_range.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut size = 0;

        for range in self.ranges.iter() {
            size += range.size_hint().0;
        }

        (size, Some(size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            46,
            solve(
                "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48
            
            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15
            
            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4
            
            water-to-light map:
            88 18 7
            18 25 70
            
            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13
            
            temperature-to-humidity map:
            0 69 1
            1 0 69
            
            humidity-to-location map:
            60 56 37
            56 93 4"
            )
        )
    }
}
