use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub enum State {
    // #
    Broken,
    // .
    Fine,
    // ?
    Unknown,
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Broken,
            '.' => Self::Fine,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
pub struct Solver {
    cache: FxHashMap<(usize, usize, usize), usize>,
}

impl Solver {
    pub fn solve_dynamic(
        &mut self,
        current_states: &[State],
        current_broken: usize,
        current_remaining: &[usize],
    ) -> usize {
        // dbg!(&current_states, current_broken, &current_remaining);

        // EOL
        if current_states.is_empty() {
            return match (current_broken, current_remaining.len()) {
                (0, 0) => 1,
                (n, 1) if n == current_remaining[0] => 1,
                _ => 0,
            };
        }

        if current_broken != 0 && current_remaining.is_empty() {
            return 0;
        }

        let cache_key = (
            current_states.len(),
            current_broken,
            current_remaining.len(),
        );
        if let Some(cached) = self.cache.get(&cache_key) {
            return *cached;
        }

        let answer = match (&current_states[0], current_broken) {
            // Sequence was finished, but does not match the expected
            (State::Fine, x) if x > 0 && x != current_remaining[0] => 0,

            // Sequence was finished and it does match the expected
            (State::Fine, 0) => self.solve_dynamic(&current_states[1..], 0, current_remaining),
            (State::Fine, _) => {
                self.solve_dynamic(&current_states[1..], 0, &current_remaining[1..])
            }

            // In the middle of the sequence
            (State::Broken, x) => {
                self.solve_dynamic(&current_states[1..], x + 1, current_remaining)
            }

            // In an unknown state...

            // ... and not in sequence yet
            (State::Unknown, 0) => {
                self.solve_dynamic(&current_states[1..], 0, current_remaining)
                    + self.solve_dynamic(&current_states[1..], 1, current_remaining)
            }

            // ... and in sequence already
            (State::Unknown, x) => {
                // if x == current_remaining[0] {
                //     self.solve_dynamic(&current_states[1..], 0, &current_remaining[1..])
                // } else {
                //     self.solve_dynamic(&current_states[1..], x + 1, current_remaining) +
                //     self.solve_dynamic(&current_states[1..], 0, &current_remaining[1..])
                // }
                let mut ans = self.solve_dynamic(&current_states[1..], x + 1, &current_remaining);
                if x == current_remaining[0] {
                    ans += self.solve_dynamic(&current_states[1..], 0, &current_remaining[1..])
                }
                ans
            }
        };

        self.cache.insert(cache_key, answer);
        answer
    }
}
