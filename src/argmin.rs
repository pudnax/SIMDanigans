use std::simd::u32x16;

#[inline]
pub fn argmin(array: &[u32]) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let mut k = 0;
    for i in 0..array.len() {
        if array[i] < array[k] {
            k = i;
        }
    }
    Some(k)
}

#[inline]
pub fn argmin_fold(array: &[u32]) -> Option<usize> {
    array
        .iter()
        .enumerate()
        .min_by_key(|(_, &value)| value)
        .map(|(idx, _)| idx)
}

#[inline]
pub fn sind_argmin(array: &[u32]) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let curr = std::array::from_fn(|i| i as u32);
    let mut curr_idx = u32x16::from_array(curr);
    let step = u32x16::splat(u32x16::LANES as _);

    let mut min = u32x16::splat(u32::MAX);
    let mut argmin = u32x16::splat(0);

    let (chunks, remainder) = array.as_chunks();
    for &chunk in chunks {
        let undertest = u32x16::from_array(chunk);
        let mask = undertest.lanes_lt(min);
        argmin = mask.select(argmin, curr_idx);
        min = min.min(undertest);
        curr_idx += step;
    }

    let min_arr = min.to_array();
    let idx_arr = argmin.to_array();

    let mut k = 0;
    for i in 1..16 {
        if min_arr[i] < min_arr[k] {
            k = i;
        }

        if min_arr[i] == min_arr[k] && idx_arr[i] < idx_arr[k] {
            k = i;
        }
    }
    k = idx_arr[k] as usize;

    for (i, &elem) in remainder.iter().enumerate() {
        if elem < array[k] {
            k = chunks.len() * u32x16::LANES + i;
        }
    }

    Some(k)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_CASE: &[u32] = &[
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    static TEST_ANS: usize = 19;

    #[test]
    fn empty_argmin() {
        assert_eq!(argmin(&[]), None);
    }

    #[test]
    fn basic_argmin() {
        assert_eq!(argmin(&TEST_CASE), Some(TEST_ANS));
    }

    #[test]
    fn fold_argmin() {
        assert_eq!(argmin_fold(&TEST_CASE), Some(TEST_ANS));
    }

    #[test]
    fn simd_argmin_16() {
        assert_eq!(sind_argmin(&TEST_CASE), Some(TEST_ANS));
    }
}
