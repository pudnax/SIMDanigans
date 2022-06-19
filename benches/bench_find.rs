use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{prelude::SliceRandom, thread_rng};
use simdanigans::*;

fn prepare_input(length: usize) -> (Vec<u32>, u32) {
    let mut input: Vec<u32> = (0..length as u32).collect();
    let middle = input[length as usize / 2];
    let mut rng = thread_rng();
    input.shuffle(&mut rng);
    (input, middle)
}

fn bench_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("Find");
    for length in [10, 20, 50, 100, 500, 1000, 2500, 5000, 10_000] {
        group.throughput(Throughput::Elements(length as u64));
        group.bench_with_input(BenchmarkId::new("STD", length), &length, |b, &length| {
            let (input, middle) = prepare_input(length);
            b.iter(|| input.iter().position(|&x| x == middle))
        });
        group.bench_with_input(
            BenchmarkId::new("Handwritten", length),
            &length,
            |b, &length| {
                let (input, middle) = prepare_input(length);
                b.iter(|| simple_find(&input, middle))
            },
        );
        group.bench_with_input(BenchmarkId::new("SIMD 4", length), &length, |b, &length| {
            let (input, middle) = prepare_input(length);
            b.iter(|| simd_find::<_, 4>(&input, middle))
        });
        group.bench_with_input(BenchmarkId::new("SIMD 8", length), &length, |b, &length| {
            let (input, middle) = prepare_input(length);
            b.iter(|| simd_find::<_, 8>(&input, middle))
        });
        group.bench_with_input(
            BenchmarkId::new("SIMD 16", length),
            &length,
            |b, &length| {
                let (input, middle) = prepare_input(length);
                b.iter(|| simd_find::<_, 16>(&input, middle))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("SIMD 32", length),
            &length,
            |b, &length| {
                let (input, middle) = prepare_input(length);
                b.iter(|| simd_find::<_, 32>(&input, middle))
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_find);
criterion_main!(benches);
