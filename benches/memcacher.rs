use burntnail_utils::memcache::MemoryCacher;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fill_benches(c: &mut Criterion) {
    c.bench_function("create 20 u8, fill, get", |b| {
        b.iter(|| {
            let mut bn = MemoryCacher::<u8, 500_000>::new(None);
            for _ in 0..black_box(20) {
                bn.push(black_box(12));
            }
            black_box(bn.get_all())
        })
    });
    c.bench_function("create 500_000 u8, fill, get", |b| {
        b.iter(|| {
            let mut bn = MemoryCacher::<u8, 500_000>::new(None);
            for _ in 0..black_box(500_000) {
                bn.push(black_box(12));
            }
            black_box(bn.get_all())
        })
    });
}

fn overfill_benches(c: &mut Criterion) {
    c.bench_function("create 20 u8, overfill 2x, get", |b| {
        b.iter(|| {
            let mut bn = MemoryCacher::<u8, 500_000>::new(None);
            for _ in 0..black_box(20 * 3) {
                bn.push(black_box(12));
            }
            black_box(bn.get_all())
        })
    });
    c.bench_function("create 500_000 u8, overfill 2x, get", |b| {
        b.iter(|| {
            let mut bn = MemoryCacher::<u8, 500_000>::new(None);
            for _ in 0..black_box(500_000 * 3) {
                bn.push(black_box(12));
            }
            black_box(bn.get_all())
        })
    });
}

criterion_group!(benches, fill_benches, overfill_benches);
criterion_main!(benches);
