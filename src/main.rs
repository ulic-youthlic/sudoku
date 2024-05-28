use std::time::Instant;

use sudoku::{Grid, Solver};

fn main() {
    let mut solver = Solver::new(&[
        [0, 0, 0, 0, 0, 4, 0, 8, 0],
        [0, 0, 0, 1, 0, 0, 7, 0, 0],
        [0, 2, 0, 0, 0, 6, 0, 0, 0],
        [0, 6, 0, 0, 0, 0, 0, 9, 4],
        [0, 0, 2, 0, 0, 1, 0, 0, 5],
        [0, 0, 8, 7, 0, 0, 0, 0, 0],
        [3, 5, 0, 0, 8, 0, 0, 0, 0],
        [0, 0, 0, 0, 3, 0, 0, 4, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 6],
    ]);
    let begin = Instant::now();
    solver.search();
    println!("{:?}", begin.elapsed());
    for row in solver.grid {
        for node in row {
            print!("{} ", node.value)
        }
        println!()
    }
}
