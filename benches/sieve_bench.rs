use criterion::{black_box, criterion_group, criterion_main, Criterion};
use backpack::math::sieve::Sieve;
use std::num::NonZeroUsize;

fn bench_build_sieve(c: &mut Criterion) {
    let maxn = NonZeroUsize::new(1e5 as usize);

    c.bench_function(
        "Sieve::new",
        |b| {
            b.iter(|| Sieve::new(black_box(maxn).unwrap()));
        }
    );
}

criterion_group!(benches, bench_build_sieve);
criterion_main!(benches);
