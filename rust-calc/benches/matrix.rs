use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn create_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    (0..rows)
        .map(|i| (0..cols).map(|j| (i * j) as i32).collect())
        .collect()
}

fn matrix_multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>, result: &mut Vec<Vec<i32>>) {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                // unsafe {
                //     *result.get_unchecked_mut(i).get_unchecked_mut(j) +=
                //         a.get_unchecked(i).get_unchecked(k) * b.get_unchecked(k).get_unchecked(j)
                // }
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
}

fn bench_matrix_multiplication(c: &mut Criterion) {
    let rows = 100;
    let cols = 100;

    let m1 = create_matrix(rows, cols);
    let m2 = create_matrix(rows, cols);

    let rows_a = m1.len();
    let cols_b = m2[0].len();

    let mut result = vec![vec![0; cols_b]; rows_a];

    c.bench_function("matrix_multiplication", |b| {
        b.iter(|| matrix_multiply(&m1, &m2, &mut result))
    });

    // c.bench_function("matrix_multiplication_blackbox", |b| {
    //     b.iter(|| matrix_multiply(black_box(&m1), black_box(&m2), black_box(&mut result)))
    // });
}

criterion_group!(benches, bench_matrix_multiplication);
criterion_main!(benches);
