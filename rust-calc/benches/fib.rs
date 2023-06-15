use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci", |b| b.iter(|| fibonacci(40)));
    c.bench_function("fibonacci_blackbox", |b| {
        b.iter(|| fibonacci(black_box(40)))
    });
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
