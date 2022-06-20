#![allow(incomplete_features)]
#![feature(portable_simd, array_chunks, slice_as_chunks, generic_const_exprs)]
mod find;

pub use find::{simd_find, simd_find_16, simple_find};
