use criterion::{criterion_group, criterion_main, Criterion};
use backpack::misc::random::Xoshiro256ppGenerator;

fn bench_xoshiro256pp(c: &mut Criterion) {
    let mut g = Xoshiro256ppGenerator::new(123976);

    c.bench_function(
        "Xoshiro256pp::rand",
        |b| {
            b.iter(|| g.rand());
        }
    );
}

criterion_group!(benches, bench_xoshiro256pp);
criterion_main!(benches);
