pub fn mirror_vertical(input: &Vec<Vec<bool>>, allowed_smudges: usize) -> Option<usize> {
    let row_len = input[0].len();
    let middle_point = row_len / 2;

    // Optimize by doing a pendulum
    let mut iterator_to_right = middle_point..row_len;
    let mut iterator_to_left = (1..middle_point).rev();

    loop {
        let left_idx = iterator_to_left.next();
        let right_idx = iterator_to_right.next();
        if left_idx.is_none() && right_idx.is_none() {
            break;
        }

        macro_rules! check {
            ($idx: ident) => {
                if let Some(idx) = $idx {
                    if input.iter().map(|row| check_row(row, idx)).sum::<usize>() == allowed_smudges {
                        return Some(idx);
                    }
                }
            };
        }

        check!(left_idx);
        check!(right_idx);
    }

    None
}

pub fn mirror_horizontal(cols: &Vec<Vec<bool>>, allowed_smudges: usize) -> Option<usize> {
    let cols_len = cols.len();
    let row_len = cols[0].len();
    let middle_point = cols_len / 2;

    // Optimize by doing a pendulum
    let mut iterator_to_right = middle_point..cols_len;
    let mut iterator_to_left = (1..middle_point).rev();

    loop {
        let left_idx = iterator_to_left.next();
        let right_idx = iterator_to_right.next();
        if left_idx.is_none() && right_idx.is_none() {
            break;
        }

        macro_rules! check {
            ($idx: ident) => {
                if let Some(idx) = $idx {
                    if (0..row_len).map(|col_idx| check_col(cols, col_idx, idx)).sum::<usize>() == allowed_smudges {
                        return Some(idx * 100);
                    }
                }
            };
        }

        check!(left_idx);
        check!(right_idx);
    }

    None
}

#[inline]
fn check_row(row: &[bool], split_idx: usize) -> usize {
    let mut smudges_cnt = 0;
    for (left_idx, right_idx) in (split_idx..row.len()).zip((0..split_idx).rev()) {
        if row[left_idx] != row[right_idx] {
            smudges_cnt += 1;
        }
    }

    smudges_cnt
}

#[inline]
fn check_col(cols: &Vec<Vec<bool>>, col_idx: usize, split_idx: usize) -> usize {
    let mut smudges_cnt = 0;
    for (left_idx, right_idx) in (split_idx..cols.len()).zip((0..split_idx).rev()) {
        if cols[left_idx][col_idx] != cols[right_idx][col_idx] {
            smudges_cnt += 1;
        }
    }

    smudges_cnt
}
