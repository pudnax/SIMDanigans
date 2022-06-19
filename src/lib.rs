#![allow(incomplete_features)]
#![feature(portable_simd, array_chunks, slice_as_chunks, generic_const_exprs)]
use std::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

pub fn simple_find(array: &[u32], x: u32) -> Option<usize> {
    array.iter().enumerate().fold(None, |mut acc, (i, &elem)| {
        if elem == x {
            acc = Some(i)
        }
        acc
    })
}

pub fn simd_find<T: SimdElement + Eq, const LANES: usize>(array: &[T], x: T) -> Option<usize>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let (chunks, remainder) = array.as_chunks::<LANES>();
    for (i, &chunk) in chunks.iter().enumerate() {
        let chunk = Simd::<T, LANES>::from_array(chunk);
        let needle = Simd::<T, LANES>::splat(x);

        let mask = needle.lanes_eq(chunk).to_int().cast::<u8>().to_array();
        let (chunks, remainder) = mask.as_chunks();
        assert!(remainder.is_empty());
        for (j, &chunk) in chunks.iter().enumerate() {
            let bitmask = u128::from_le_bytes(chunk);
            if bitmask != 0 {
                return Some(i * LANES + j * chunk.len() + bitmask.trailing_zeros() as usize / 8);
            }
        }
    }

    for (i, &elem) in remainder.iter().enumerate() {
        if elem == x {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use rand::{prelude::SliceRandom, thread_rng};

    use super::*;
    static TEST_CASE: &[u32] = &[
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];

    fn prepare_input(length: usize) -> (Vec<u32>, u32) {
        let mut input: Vec<u32> = (0..length as u32).collect();
        let middle = input[length as usize / 2];
        let mut rng = thread_rng();
        input.shuffle(&mut rng);
        (input, middle)
    }

    #[test]
    fn simple_find_test() {
        let target = 1;

        assert_eq!(
            simple_find(&TEST_CASE, target),
            TEST_CASE.iter().position(|&x| x == target)
        );
    }

    #[test]
    fn simd_find_test() {
        let target = 1;

        assert_eq!(
            simd_find::<_, 16>(&TEST_CASE, target),
            TEST_CASE.iter().position(|&x| x == target)
        );
    }

    #[test]
    fn simd_find_4_10() {
        let (input, middle) = prepare_input(10);
        assert_eq!(
            simd_find::<_, 4>(&input, middle),
            input.iter().position(|&x| x == middle)
        );
    }

    #[test]
    fn simd_find_16_10() {
        let (input, middle) = prepare_input(10);
        assert_eq!(
            simd_find::<_, 16>(&input, middle),
            input.iter().position(|&x| x == middle)
        );
    }
}
