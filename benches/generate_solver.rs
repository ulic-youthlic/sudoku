use sudoku::{Grid, Solver};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_solver(c: &mut Criterion) {
    c.bench_function("generate solution", |b| {
        b.iter(|| {
            let _ = Grid::new();
        })
    });
    let grid = Grid([
        [5, 0, 0, 0, 0, 0, 3, 0, 0],
        [0, 2, 0, 1, 0, 0, 0, 7, 0],
        [0, 0, 8, 0, 0, 0, 0, 0, 9],
        [0, 4, 0, 0, 0, 7, 0, 0, 0],
        [0, 0, 0, 8, 2, 1, 0, 0, 0],
        [0, 0, 0, 6, 0, 0, 0, 1, 0],
        [3, 0, 0, 0, 0, 0, 8, 0, 0],
        [0, 6, 0, 0, 0, 4, 0, 2, 0],
        [0, 0, 9, 0, 0, 0, 0, 0, 5],
    ]);
    c.bench_function("solve sudoku", |b| {
        b.iter_batched(
            || Solver::new(black_box(&grid.0)),
            |mut solver| {
                solver.search();
            },
            criterion::BatchSize::PerIteration,
        )
    });
}

criterion_group!(benches, benchmark_solver);
criterion_main!(benches);
