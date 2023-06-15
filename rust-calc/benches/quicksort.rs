use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::Uniform;
use rand::prelude::*;

fn quicksort(x: &mut [i32], low: isize, high: isize) {
    if low < high {
        let p = partition(x, low, high);
        quicksort(x, low, p - 1);
        quicksort(x, p + 1, high);
    }
}

fn partition(x: &mut [i32], low: isize, high: isize) -> isize {
    let pivot = x[high as usize];
    // let pivot = unsafe { *x.get_unchecked(high as usize) };
    let mut i = low - 1;
    for j in low..high {
        if x[j as usize] < pivot {
            // if unsafe { *x.get_unchecked(j as usize) } < pivot {
            i += 1;
            x.swap(i as usize, j as usize);
        }
    }
    x.swap((i + 1) as usize, high as usize);
    i + 1
}

fn bench_quicksort(c: &mut Criterion) {
    let rng = rand::thread_rng();
    let array: Vec<i32> = rng
        .sample_iter(Uniform::new(0, 100000))
        .take(10000)
        .collect();

    c.bench_function("quicksort", |b| {
        b.iter(|| {
            let mut array = array.clone();
            quicksort(&mut array, 0, 9999);
        })
    });

    // c.bench_function("quicksort_blackbox", |b| {
    //     b.iter(|| quicksort(black_box(&mut array.clone()), black_box(0), black_box(9999)))
    // });
}

criterion_group!(benches, bench_quicksort);
criterion_main!(benches);
