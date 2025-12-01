use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut, Not},
};

use itertools::Itertools;

pub struct Grid<const N: usize, T> {
    grid: [[T; N]; N],
}

impl<const N: usize, T> Grid<N, T> {
    pub fn iter(&self) -> GridIter<'_, N, T> {
        GridIter {
            grid: self,
            pos: (0, 0),
        }
    }

    pub fn get(&self, pos: (usize, usize)) -> Option<&T> {
        self.grid.get(pos.1)?.get(pos.0)
    }
}

impl<const N: usize, T> Display for Grid<N, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self
            .grid
            .iter()
            .map(|line| line.iter().map(|tile| tile.to_string()).collect::<String>())
            .join("\n");
        writeln!(f, "\n{}", grid)
    }
}

impl<const N: usize, T> Debug for Grid<N, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<const N: usize, T> Index<(usize, usize)> for Grid<N, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.grid.index(index.1).index(index.0)
    }
}

impl<const N: usize, T> IndexMut<(usize, usize)> for Grid<N, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.grid.index_mut(index.1).index_mut(index.0)
    }
}

impl<const N: usize, T> Default for Grid<N, T>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            grid: [[T::default(); N]; N],
        }
    }
}

pub struct GridIter<'a, const N: usize, T> {
    grid: &'a Grid<N, T>,
    pos: (usize, usize),
}

impl<'a, const N: usize, T> Iterator for GridIter<'a, N, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        let tile = self.grid.get(pos)?;
        self.pos = match self.pos {
            (x, y) if x + 1 < N => (x + 1, y),
            (_, y) => (0, y + 1),
        };
        Some((pos, tile))
    }
}

use Direction::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn all() -> [Direction; 8] {
        [
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ]
    }

    pub fn right_turn(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
            NorthEast => SouthEast,
            SouthEast => SouthWest,
            SouthWest => NorthWest,
            NorthWest => NorthEast,
        }
    }

    pub fn cardinal() -> [Direction; 4] {
        [North, East, South, West]
    }

    pub fn step(&self, pos: (usize, usize), bound: usize) -> Option<(usize, usize)> {
        match self {
            North => Some((pos.0, pos.1.checked_sub(1)?)),
            South => {
                let added = pos.1 + 1;
                if added < bound {
                    Some((pos.0, added))
                } else {
                    None
                }
            }
            East => {
                let added = pos.0 + 1;
                if added < bound {
                    Some((added, pos.1))
                } else {
                    None
                }
            }
            West => Some((pos.0.checked_sub(1)?, pos.1)),
            NorthEast => {
                let added = pos.0 + 1;
                if added < bound {
                    Some((added, pos.1.checked_sub(1)?))
                } else {
                    None
                }
            }
            SouthEast => {
                let added = pos.0 + 1;
                let added2 = pos.1 + 1;
                if added < bound && added2 < bound {
                    Some((added, added2))
                } else {
                    None
                }
            }
            SouthWest => {
                let added = pos.0.checked_sub(1)?;
                let added2 = pos.1 + 1;
                if added2 < bound {
                    Some((added, added2))
                } else {
                    None
                }
            }
            NorthWest => {
                let added = pos.0.checked_sub(1)?;
                if added < bound {
                    Some((added, pos.1.checked_sub(1)?))
                } else {
                    None
                }
            }
        }
    }
}

impl Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
            NorthEast => SouthWest,
            SouthWest => NorthEast,
            SouthEast => NorthWest,
            NorthWest => SouthEast,
        }
    }
}
