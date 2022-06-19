#![allow(incomplete_features)]
#![feature(portable_simd, slice_as_chunks, generic_const_exprs)]
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
    // const LANES: usize = 16;
    let (chunks, remainder) = array.as_chunks::<LANES>();
    for (i, &chunk) in chunks.iter().enumerate() {
        let chunk = Simd::<T, LANES>::from_array(chunk);
        let needle = Simd::<T, LANES>::splat(x);

        let mask = needle.lanes_eq(chunk);
        if mask.any() {
            for (pos, &check) in mask.to_array().iter().enumerate() {
                if check {
                    return Some(i * LANES + pos as usize);
                }
            }
            // return Some(i * LANES + mask.to_bitmask().trailing_zeros() as usize);
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
    use super::*;
    static TEST_CASE: &[u32] = &[
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];

    #[test]
    fn simple_find_test() {
        let target = 1;

        dbg!("boop");

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
}
