use std::cmp::Ordering;

pub mod game;

/// ```
/// # use a2048::*;
/// let mut arr = [0, 2, 1, 8, 0, 0, 3, 4, 0];
/// sort_zeros::<true>(&mut arr);
/// assert_eq!(arr.to_vec(), vec![0, 0, 0, 0, 2, 1, 8, 3, 4])
/// ```
/// ```
/// # use a2048::*;
/// # let mut arr = [0, 2, 1, 8, 0, 0, 3, 4, 0];
/// sort_zeros::<false>(&mut arr);
/// assert_eq!(arr.to_vec(), vec![2, 1, 8, 3, 4, 0, 0, 0, 0])
/// ```
pub fn sort_zeros<const DIR: bool>(arr: &mut [usize]) {
    arr.sort_by(|&a, &b| {
        let is_zero = if DIR { a == 0 } else { b == 0 };
        if is_zero {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    })
}

/// A simple algorithm for merging the way 2048 does.
/// `DIR: true` is merge at the end of the `Vec`, `DIR: false` is merge at the start of the `vec`
/// ```
/// # use a2048::*;
/// let mut arr = [2, 2, 0, 2, 2, 8, 0, 0, 8, 4, 0, 4, 8];
/// assert_eq!(merge_2048::<true>(&arr), (vec![0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 16, 8, 8], 32));
/// assert_eq!(merge_2048::<false>(&arr),(vec![4, 4, 16, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0], 32))
/// ```
pub fn merge_2048<const DIR: bool>(arr: &[usize]) -> (Vec<usize>, usize) {
    let mut sorted = Vec::from(arr);
    let mut points: usize = 0;

    if !DIR {
        sorted.reverse();
    }

    sort_zeros::<DIR>(&mut sorted);

    for i in (0..sorted.len()).rev() {
        let n = sorted[i];

        if n == 0 {
            continue;
        }

        if let Some(pos) = i.checked_sub(1) {
            if sorted[pos] == n {
                sorted[pos] = 0;
                sorted[i] *= 2;
                points += n * 2;
            }
        }

        if let Some(zero_count) = sorted.iter().skip(i).rposition(|a| a == &0) {
            let pos = i + zero_count;
            if pos < sorted.len() {
                sorted.swap(i, pos)
            }
        }
    }

    if !DIR {
        sorted.reverse()
    }

    (sorted, points)
}
