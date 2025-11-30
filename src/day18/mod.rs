use std::{
    collections::VecDeque,
    fmt,
    ops::{Index, IndexMut},
};

use aoc_util::{Coord, Grid};

pub fn run() {
    part2();
}

const INPUT: &str = include_str!("in");

fn part1() {
    let mut coords = parse_input();
    let width = coords.iter().map(|c| c.x).max().unwrap() + 3;
    let height = coords.iter().map(|c| c.y).max().unwrap() + 3;
    let depth = coords.iter().map(|c| c.z).max().unwrap() + 3;
    for c in &mut coords {
        let Coord3D { x, y, z } = c;
        *x += 1;
        *y += 1;
        *z += 1;
    }

    let mut grid = Grid3D(vec![Grid::new_with(Tile::E, width, height); depth].into_boxed_slice());
    let size = Coord3D::new(width, height, depth);

    for c in &coords {
        grid[*c] = Tile::R;
    }

    dbg!(&grid);

    let mut out = 0;

    for c in coords {
        for (dx, dy, dz) in [
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
        ] {
            if let Some(cc) = new_coord(c, dx, dy, dz, size) {
                if grid[cc] == Tile::E {
                    out += 1;
                }
            }
        }
    }

    dbg!(out);
}

fn part2() {
    let mut coords = parse_input();
    let width = coords.iter().map(|c| c.x).max().unwrap() + 3;
    let height = coords.iter().map(|c| c.y).max().unwrap() + 3;
    let depth = coords.iter().map(|c| c.z).max().unwrap() + 3;
    for c in &mut coords {
        let Coord3D { x, y, z } = c;
        *x += 1;
        *y += 1;
        *z += 1;
    }

    let mut grid = Grid3D(vec![Grid::new_with(Tile::E, width, height); depth].into_boxed_slice());
    let size = Coord3D::new(width, height, depth);

    for c in &coords {
        grid[*c] = Tile::R;
    }

    let start = Coord3D::new(0, 0, 0);
    let mut q = VecDeque::from([start]);

    while let Some(c) = q.pop_front() {
        for nc in c.neighbours(size) {
            if grid[nc] == Tile::E {
                grid[nc] = Tile::O;
                q.push_back(nc);
            }
        }
    }

    dbg!(&grid);

    let mut out = 0;

    for c in coords {
        for nc in c.neighbours(size) {
            if grid[nc] == Tile::O {
                out += 1;
            }
        }
    }

    dbg!(out);
}

fn parse_input() -> Vec<Coord3D> {
    INPUT
        .lines()
        .map(|l| {
            let mut l = l.split(',');
            let x = l.next().unwrap().parse().unwrap();
            let y = l.next().unwrap().parse().unwrap();
            let z = l.next().unwrap().parse().unwrap();
            Coord3D::new(x, y, z)
        })
        .collect()
}

fn new_coord(c: Coord3D, dx: isize, dy: isize, dz: isize, size: Coord3D) -> Option<Coord3D> {
    let Coord3D { x, y, z } = c;
    let nx = (x as isize + dx).try_into().ok()?;
    let ny = (y as isize + dy).try_into().ok()?;
    let nz = (z as isize + dz).try_into().ok()?;

    (nx < size.x && ny < size.y && nz < size.z).then_some(Coord3D::new(nx, ny, nz))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    E,
    R,
    O,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::E => write!(f, "@"),
            Self::R => write!(f, "#"),
            Tile::O => write!(f, "."),
        }
    }
}

struct Grid3D<T>(Box<[Grid<T>]>);

#[derive(Debug, Clone, Copy)]
struct Coord3D {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord3D {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    fn neighbours(self, size: Self) -> impl Iterator<Item = Self> {
        [
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
        ]
        .into_iter()
        .filter_map(move |(dx, dy, dz)| new_coord(self, dx, dy, dz, size))
    }
}

impl<T> Index<Coord3D> for Grid3D<T> {
    type Output = T;

    fn index(&self, index: Coord3D) -> &Self::Output {
        let c = Coord::new(index.x, index.y);
        &self.0[index.z][c]
    }
}

impl<T> IndexMut<Coord3D> for Grid3D<T> {
    fn index_mut(&mut self, index: Coord3D) -> &mut Self::Output {
        let c = Coord::new(index.x, index.y);
        &mut self.0[index.z][c]
    }
}

impl<T: fmt::Debug> fmt::Debug for Grid3D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for plane in &self.0 {
            writeln!(f, "{plane:?}")?;
        }

        Ok(())
    }
}
