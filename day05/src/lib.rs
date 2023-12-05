use std::io::Write;

#[derive(Debug)]
struct CorrespondenceRange {
    target_start: usize,
    dest_start: usize,
    range_len: usize
}

#[derive(Debug, Default)]
pub struct CorrespondenceMap {
    ranges: Vec<CorrespondenceRange>
}

pub fn parse_maps<'i>(lines: impl Iterator<Item = &'i str>) -> Vec<CorrespondenceMap> {
    let mut lines = lines.peekable();
    let mut maps: Vec<CorrespondenceMap> = Vec::new();

    while let Some(line) = lines.peek() {
        // Ignore empty lines before map blocks
        if line.trim().is_empty() {
            lines.next();
            continue;
        }

        maps.push(parse_map(&mut lines));
    }

    maps
}

/// Solves the day for both part1 and part2
pub fn solve_for(seeds: impl IntoIterator<Item = usize>, maps: Vec<CorrespondenceMap>) -> usize {
    let mut min_result = usize::MAX;

    // Track progress
    let seeds = seeds.into_iter();
    let size_hint = seeds.size_hint().0;
    let mut prev_progress_bars = 0;
    print_progress(0);

    for (idx, seed) in seeds.enumerate() {
        let progress_bars = map_to_scale(idx, size_hint);
        if progress_bars != prev_progress_bars {
            prev_progress_bars = progress_bars;
            print_progress(progress_bars);
        }

        let mut current = seed;

        for map in maps.iter() {
            current = map.correspond(current);
        }
        min_result = min_result.min(current);
    }

    // Clear progress
    print!("\r{}\r", " ".repeat(22));

    min_result
}

#[inline]
pub fn print_progress(bars: usize) {
    print!("\r[{}{}]", "#".repeat(bars), " ".repeat(20 - bars));
    std::io::stdout().flush().unwrap();
}

fn parse_map<'i>(lines: &mut impl Iterator<Item = &'i str>) -> CorrespondenceMap {
    let mut map = CorrespondenceMap::default();

    // Read and check header
    let header = lines.next().expect("Must have header").trim();
    const MAP_SUFFIX: &'static str = " map:";
    assert!(header.ends_with(MAP_SUFFIX));

    println!("Parsing map {}", &header[..header.len() - MAP_SUFFIX.len()]);

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let mut nums = line.split_whitespace();
        let dest_start = nums.next().unwrap().parse().unwrap();
        let target_start = nums.next().unwrap().parse().unwrap();
        let range_len = nums.next().unwrap().parse().unwrap();

        map.add_range(target_start, dest_start, range_len);
    }

    map
}

fn map_to_scale(current: usize, total: usize) -> usize {
    // Ensure total is not zero to avoid division by zero
    if total == 0 {
        return 0; // Or handle this case as needed
    }

    // Calculate the ratio of current to total as a floating-point number
    let ratio = current as f64 / total as f64;

    // Map the ratio to a scale of 0 to 20
    let scaled_value = (ratio * 20.0).round() as usize;

    // Ensure the result is within the valid scale (0 to 20)
    scaled_value.clamp(0, 20)
}

impl CorrespondenceMap {
    fn add_range(&mut self, target_start: usize, dest_start: usize, range_len: usize) {
        self.ranges.push(CorrespondenceRange { target_start, dest_start, range_len });
    }

    /// Correspond `target` to `dest`
    pub fn correspond(&self, target: usize) -> usize {
        for range in self.ranges.iter() {
            // Check lower bound
            let range_target_start = range.target_start;
            if target < range_target_start {
                continue;
            }

            // Check upper bound
            let range_diff = target - range_target_start;
            if range_diff > range.range_len {
                continue;
            }

            // Map
            return range.dest_start + range_diff;
        }

        target
    }
}
