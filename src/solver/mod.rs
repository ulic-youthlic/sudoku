use std::time::{Duration, Instant};

use rand::{
    rngs::ThreadRng,
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

use self::set::Set;

mod set;

#[derive(Default)]
pub struct Grid(pub [[u8; 9]; 9]);

pub struct Solver {
    pub grid: [[Node; 9]; 9],
}

impl Default for Solver {
    fn default() -> Self {
        let grid: [[Node; 9]; 9] =
            std::array::from_fn(|row| std::array::from_fn(|col| Node::new(row, col)));
        Self { grid }
    }
}

pub struct Node {
    pub value: u8,
    remind: Set,
    row: usize,
    col: usize,
    blk: usize,
}

impl From<Solver> for Grid {
    fn from(value: Solver) -> Self {
        let mut grid = Grid::default();
        for row in 0..9 {
            for col in 0..9 {
                grid.0[row][col] = value.grid[row][col].value;
            }
        }
        grid
    }
}

impl Grid {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut solver = Solver::default();
        solver.search();
        solver.into()
    }

    pub fn build(max_blank: usize) -> Self {
        let mut grid = Grid::new();
        let mut rng = thread_rng();
        let begin_time = Instant::now();
        for _ in 0..max_blank {
            loop {
                let row = (0..9_usize).choose(&mut rng).unwrap();
                let col = (0..9_usize).choose(&mut rng).unwrap();
                let temp = grid.0[row][col];
                if temp == 0 {
                    continue;
                }
                grid.0[row][col] = 0;
                let mut solver = Solver::new(&grid.0);
                if solver.only_solution().is_some() {
                    break;
                } else {
                    grid.0[row][col] = temp;
                }
                if begin_time.elapsed() > Duration::from_secs(1) {
                    break;
                }
            }
        }
        grid
    }
}

impl Node {
    fn new(row: usize, col: usize) -> Self {
        Self {
            value: 0,
            remind: Set::default(),
            row,
            col,
            blk: 3 * (row / 3) + col / 3,
        }
    }
}

impl Solver {
    fn next(&self) -> Option<&Node> {
        self.grid
            .iter()
            .flatten()
            .filter(|node| node.remind.size() != 1)
            .min_by_key(|node| (node.remind.size(), node.row, node.col))
    }
    pub fn new(grid: &[[u8; 9]; 9]) -> Self {
        let mut solver = Solver::default();
        let iter = grid
            .iter()
            .enumerate()
            .flat_map(|(row, col)| {
                col.iter()
                    .enumerate()
                    .map(move |(col, value)| (*value, row, col))
            })
            .filter(|(value, _row, _col)| *value > 0);
        iter.clone().for_each(|(value, row, col)| {
            solver.grid[row][col].remind.set(value);
        });
        iter.for_each(|(value, row, col)| {
            let mut steps = Vec::new();
            solver.set(row, col, value, &mut steps);
        });
        solver
    }

    #[allow(dead_code)]
    pub fn only_solution(&mut self) -> Option<()> {
        let mut solution_num_needed = 2;
        let mut rng = thread_rng();
        self.search_helper(&mut solution_num_needed, &mut rng);
        if solution_num_needed == 1 {
            Some(())
        } else {
            None
        }
    }
    pub fn search(&mut self) {
        let mut solution_num_needed = 1;
        let mut rng = thread_rng();
        self.search_helper(&mut solution_num_needed, &mut rng)
            .unwrap();
    }

    fn search_helper(
        &mut self,
        solution_num_needed: &mut isize,
        rng: &mut ThreadRng,
    ) -> Option<()> {
        if let Some(node) = self.next() {
            let (row, col) = (node.row, node.col);
            let mut list = node.remind.get();
            if list.is_empty() {
                None
            } else {
                list.shuffle(rng);
                let mut iter = list.iter();
                let res = loop {
                    if let Some(value) = iter.next() {
                        self.grid[row][col].remind.set(*value);
                        let mut steps = Vec::new();
                        self.set(row, col, *value, &mut steps);
                        if self.search_helper(solution_num_needed, rng).is_some()
                            && *solution_num_needed == 0
                        {
                            break Some(());
                        } else {
                            self.unset(&steps);
                        }
                    } else {
                        break None;
                    }
                };
                list.into_iter().for_each(|value| {
                    self.grid[row][col].remind.add(value);
                });
                res
            }
        } else {
            *solution_num_needed -= 1;
            Some(())
        }
    }

    fn unset(&mut self, steps: &[(u8, usize, usize)]) {
        steps.iter().for_each(|(value, row, col)| {
            self.grid[*row][*col].add(*value);
        });
    }

    fn set(&mut self, row: usize, col: usize, value: u8, steps: &mut Vec<(u8, usize, usize)>) {
        let mut list = vec![(value, row, col)];
        while !list.is_empty() {
            list = self.set_helper(&list, steps);
        }
    }
    fn set_helper(
        &mut self,
        list: &[(u8, usize, usize)],
        steps: &mut Vec<(u8, usize, usize)>,
    ) -> Vec<(u8, usize, usize)> {
        list.iter()
            .flat_map(|(value, row, col)| {
                let (mut node, mut grid): (Vec<_>, Vec<_>) = self
                    .grid
                    .iter_mut()
                    .flatten()
                    .partition(|node| node.row == *row && node.col == *col);
                node[0].set(*value, &mut grid, steps)
            })
            .collect()
    }
}

impl Node {
    fn set<'g>(
        &mut self,
        value: u8,
        grid: &'g mut [&'g mut Node],
        steps: &mut Vec<(u8, usize, usize)>,
    ) -> Vec<(u8, usize, usize)> {
        self.value = value;
        grid.iter_mut()
            .filter(|node| node.row == self.row || node.col == self.col || node.blk == self.blk)
            .filter_map(|node| node.remove(value, steps))
            .collect::<Vec<_>>()
    }

    fn remove(
        &mut self,
        value: u8,
        steps: &mut Vec<(u8, usize, usize)>,
    ) -> Option<(u8, usize, usize)> {
        if self.remind.remove(value).is_some() {
            steps.push((value, self.row, self.col));
            self.remind
                .find_only()
                .map(|value| (value, self.row, self.col))
        } else {
            None
        }
    }

    fn add(&mut self, value: u8) {
        self.remind.add(value);
        if self.value > 0 {
            self.value = 0;
        }
    }
}
