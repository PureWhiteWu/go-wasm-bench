use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn prime_factors(mut n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    if n > 2 {
        factors.push(n);
    }
    factors
}

fn bench_prime_factors(c: &mut Criterion) {
    c.bench_function("prime_factors", |b| b.iter(|| prime_factors(2310)));
    // c.bench_function("prime_factors_blackbox", |b| {
    //     b.iter(|| prime_factors(black_box(2310)))
    // });
}

criterion_group!(benches, bench_prime_factors);
criterion_main!(benches);
