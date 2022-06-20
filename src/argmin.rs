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

#[cfg(test)]
mod test {
    use super::*;

    static TEST_CASE: &[u32] = &[
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    static TEST_ANS: usize = 19;

    #[test]
    fn empty_test() {
        assert_eq!(argmin(&[]), None);
    }

    #[test]
    fn basic_test() {
        assert_eq!(argmin(&TEST_CASE), Some(TEST_ANS));
    }

    #[test]
    fn basic_fold_test() {
        assert_eq!(argmin_fold(&TEST_CASE), Some(TEST_ANS));
    }
}
