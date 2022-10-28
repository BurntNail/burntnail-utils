use burntnail_utils::memcache::MemoryCacher;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn create_benches(c: &mut Criterion) {
    c.bench_function("create 20 u8", |b| {
        b.iter(|| MemoryCacher::<u8, 20>::new(None))
    });
    c.bench_function("create 20 u64", |b| {
        b.iter(|| MemoryCacher::<u64, 20>::new(None))
    });
    c.bench_function("create 500_000 u8", |b| {
        b.iter(|| MemoryCacher::<u8, 500_000>::new(None))
    });
}

fn fill_benches(c: &mut Criterion) {
    c.bench_function("create 20 u8, fill, get", |b| {
        b.iter(|| {
            let mut bn = MemoryCacher::<u8, 500_000>::new(None);
            for _ in 0..20 {
                bn.push(black_box(12));
            }
            black_box(bn.get_all())
        })
    });
    c.bench_function("create 500_000 u8, fill, get", |b| {
        b.iter(|| {
            let mut bn = MemoryCacher::<u8, 500_000>::new(None);
            for _ in 0..500_000 {
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
            for _ in 0..60 {
                bn.push(black_box(12));
            }
            black_box(bn.get_all())
        })
    });
    c.bench_function("create 500_000 u8, overfill 2x, get", |b| {
        b.iter(|| {
            let mut bn = MemoryCacher::<u8, 500_000>::new(None);
            for _ in 0..1_500_000 {
                bn.push(black_box(12));
            }
            black_box(bn.get_all())
        })
    });
}

criterion_group!(benches, create_benches, fill_benches, overfill_benches);
criterion_main!(benches);
