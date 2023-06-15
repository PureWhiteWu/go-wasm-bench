use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn n_queens(n: usize) -> usize {
    let mut solutions = 0;
    let mut board = vec![0; n];
    place_queen(&mut board, 0usize, &mut solutions);
    solutions
}

fn place_queen(board: &mut Vec<usize>, current_row: usize, solutions: &mut usize) {
    let n = board.len();
    if current_row == n {
        *solutions += 1;
    } else {
        for col in 0..n {
            if can_place_queen(&board, current_row, col) {
                board[current_row] = col;
                // unsafe { *board.get_unchecked_mut(current_row) = col }
                place_queen(board, current_row + 1, solutions);
            }
        }
    }
}

fn can_place_queen(board: &[usize], row: usize, col: usize) -> bool {
    for i in 0..row {
        if board[i] == col || (row - i) == (col as isize - board[i] as isize).abs() as usize {
            return false;
        }
    }
    true
}

fn bench_n_queens(c: &mut Criterion) {
    c.bench_function("n_queens", |b| {
        b.iter(|| n_queens(12));
    });

    // c.bench_function("n_queens_blackbox", |b| {
    //     b.iter(|| n_queens(black_box(12)));
    // });
}

criterion_group!(benches, bench_n_queens);
criterion_main!(benches);
