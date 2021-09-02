use std::hash::Hash;

use crate::{merge_2048, Placement};

use grid::*;
use rand::prelude::{SeedableRng, StdRng};
use rand::{seq::IteratorRandom, Rng};

#[derive(Debug)]
pub struct Game {
    pub rng: StdRng,
    pub grid: Grid<usize>,
    pub points: usize,
}

macro_rules! shift_fn {
    ($name:ident, $axis:ident, $placement:path) => {
        pub fn $name(&mut self) {
            paste::paste! {
                for i in 0..self.grid.[<$axis s>]() {
                    let axis: Vec<_> = self.grid.[<iter_ $axis>](i).map(|v| *v).collect();
                    let (merged, points) = merge_2048::<{ $placement }>(&axis);
                    self.points += points;
                    self.grid.[<replace_ $axis>](i, merged);
                }
            }
        }
    };
}

impl Game {
    pub fn new(width: usize, height: usize, seed: Option<u64>) -> Self {
        Game {
            rng: seed
                .map(SeedableRng::seed_from_u64)
                .unwrap_or_else(|| rand::SeedableRng::seed_from_u64(rand::random())),
            points: 0,
            grid: Grid::new(height, width),
        }
    }

    shift_fn!(shift_left, row, Placement::Start);
    shift_fn!(shift_right, row, Placement::End);
    shift_fn!(shift_top, col, Placement::Start);
    shift_fn!(shift_bottom, col, Placement::End);

    pub fn add_random_tile(&mut self) {
        let mut rng = &mut self.rng;
        self.grid
            .iter_mut()
            .filter(|v| **v == 0)
            .choose(&mut rng)
            .map(|v| {
                *v = match rng.gen_ratio(1, 10) {
                    false => 2,
                    true => 4,
                };
            });
    }

    pub fn is_game_over(&self) -> bool {
        !self.grid.iter().any(|&v| v == 0)
            && !self.grid.iter_with_index().any(|((row, col), val)| {
                self.grid.adjacent(row, col).iter().any(|&v| v == Some(val))
            })
    }
}

impl Hash for Game {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.grid.hash(state);
        self.points.hash(state);
    }
}
