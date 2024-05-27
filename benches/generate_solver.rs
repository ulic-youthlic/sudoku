use sudoku::{Grid, Solver};

use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_solver(c: &mut Criterion) {
    c.bench_function("generate solution", |b| {
        b.iter(|| {
            let _ = Grid::new();
        })
    });
    let gird = Grid([
        [0, 6, 0, 8, 0, 0, 0, 0, 5],
        [0, 0, 9, 0, 0, 2, 0, 0, 0],
        [0, 8, 0, 0, 6, 4, 0, 0, 7],
        [0, 0, 0, 4, 7, 0, 8, 0, 0],
        [0, 0, 2, 0, 0, 0, 1, 0, 6],
        [0, 5, 0, 2, 0, 0, 0, 0, 0],
        [0, 0, 0, 5, 0, 0, 0, 0, 0],
        [0, 0, 3, 0, 0, 8, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 9, 2, 0],
    ]);
    let mut solver = Solver::new(&gird.0);
    c.bench_function("solve sudoku", |b| {
        b.iter(|| {
            solver.search();
        })
    });
}

criterion_group!(benches, benchmark_solver);
criterion_main!(benches);
