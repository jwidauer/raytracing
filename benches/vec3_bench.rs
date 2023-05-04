use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracing::vec3;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("vec3::dot", |b| {
        b.iter(|| {
            vec3::Vec3::new(black_box(0.0), black_box(0.0), black_box(0.0)).dot(black_box(
                vec3::Vec3::new(black_box(0.0), black_box(0.0), black_box(0.0)),
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
