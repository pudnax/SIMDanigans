use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{prelude::SliceRandom, thread_rng};
use simdanigans::find::*;

fn prepare_input(length: usize) -> (Vec<u32>, u32) {
    let mut input: Vec<u32> = (0..length as u32).collect();
    let middle = input[length as usize / 2];
    let mut rng = thread_rng();
    input.shuffle(&mut rng);
    (input, middle)
}

fn bench_find_smol(c: &mut Criterion) {
    let mut group = c.benchmark_group("Find (Alt)");
    for length in [50, 100, 1000, 2500, 5000, 6000, 7000, 8000, 9000, 10_000] {
        group.throughput(Throughput::Elements(length as u64));
        group.bench_with_input(BenchmarkId::new("STD", length), &length, |b, &length| {
            let (input, middle) = prepare_input(length);
            b.iter(|| input.iter().position(|&x| x == middle))
        });
        group.bench_with_input(
            BenchmarkId::new("SIMD 16", length),
            &length,
            |b, &length| {
                let (input, middle) = prepare_input(length);
                b.iter(|| simd_find_16(&input, &middle))
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_find_smol);
criterion_main!(benches);
