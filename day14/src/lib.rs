use std::fmt::{Debug, Write};
use itertools::Either;
use rayon::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum State {
    Ground,
    Obstacle,
    RollingStone,
}

pub enum Gravity {
    /// Items fall towards the lesser index
    Positive,
    /// Items fall towards the bigger index
    Negative,
}

pub fn apply_vertical_gravity(
    map: &mut Vec<State>,
    total_rows: usize,
    row_len: usize,
    gravity: Gravity,
) {
    let is_gravity_positive = matches!(gravity, Gravity::Positive);

    for col_idx in 0..row_len {
        let mut last_free_idx: Option<usize> = None;
        let row_iter = if is_gravity_positive {
            Either::Left(0..total_rows)
        } else {
            Either::Right((0..total_rows).rev())
        };

        for row_idx in row_iter {
            let item_idx = row_idx * row_len + col_idx;
            match map[item_idx] {
                State::Ground => {
                    if last_free_idx.is_none() {
                        last_free_idx = Some(item_idx);
                    }
                }
                State::Obstacle => {
                    last_free_idx = None;
                }
                State::RollingStone => {
                    if let Some(free_idx) = last_free_idx {
                        map.swap(item_idx, free_idx);
                        last_free_idx = Some(if is_gravity_positive {
                            free_idx + row_len
                        } else {
                            free_idx - row_len
                        });
                    }
                }
            }
        }
    }
}

pub fn apply_horizontal_gravity(
    map: &mut Vec<State>,
    _total_rows: usize,
    row_len: usize,
    gravity: Gravity,
) {
    let is_gravity_positive = matches!(gravity, Gravity::Positive);

    map.par_chunks_mut(row_len)
        .for_each(|chunk| {
            let mut last_free_idx: Option<usize> = None;
            let col_iter = if is_gravity_positive {
                Either::Left(0..row_len)
            } else {
                Either::Right((0..row_len).rev())
            };
    
            for col_idx in col_iter {
                match chunk[col_idx] {
                    State::Ground => {
                        if last_free_idx.is_none() {
                            last_free_idx = Some(col_idx);
                        }
                    }
                    State::Obstacle => {
                        last_free_idx = None;
                    }
                    State::RollingStone => {
                        if let Some(free_idx) = last_free_idx {
                            chunk.swap(col_idx, free_idx);
                            last_free_idx = Some(if is_gravity_positive {
                                free_idx + 1
                            } else {
                                free_idx - 1
                            });
                        }
                    }
                }
            }
        });
}

pub fn calculate_weighted_sum(map: &[State], row_len: usize, total_rows: usize) -> usize {
    let mut sum = 0;
    for (idx, item) in map.iter().enumerate() {
        if !matches!(item, State::RollingStone) {
            continue;
        }

        let item_row = idx / row_len;
        let row_value = total_rows - item_row;
        sum += row_value;
    }

    sum
}

pub fn show_map(map: &[State], row_len: usize) {
    let mut result = String::with_capacity(map.len());
    for row in map.chunks(row_len) {
        for it in row.iter() {
            let _ = write!(&mut result, "{:?}", it);
        }
        result.push('\n');
    }
    println!("{result}");
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ground,
            '#' => Self::Obstacle,
            'O' => Self::RollingStone,
            _ => unreachable!(),
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => write!(f, "."),
            Self::Obstacle => write!(f, "#"),
            Self::RollingStone => write!(f, "O"),
        }
    }
}
