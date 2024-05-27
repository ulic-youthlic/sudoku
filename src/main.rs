use std::time::Instant;

use sudoku::{Grid, Solver};

fn main() {
    let grid = Grid::build(60);
    let mut solver = Solver::new(&grid.0);
    let begin = Instant::now();
    solver.search();
    println!("{:?}", begin.elapsed());
    for raw in grid.0 {
        for node in raw {
            print!("{} ", node)
        }
        println!()
    }
    println!();
    for row in solver.grid {
        for node in row {
            print!("{} ", node.value)
        }
        println!()
    }
}
