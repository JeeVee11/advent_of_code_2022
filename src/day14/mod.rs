use std::{
    cmp::{max, min},
    fmt,
};

use aoc_util::{Coord, Dir::*, Grid};

pub fn run() {
    part2();
}

const INPUT: &str = include_str!("in");

fn part1() {
    let (mut grid, start) = create_grid_1();
    let source = Coord::new(start, 0);
    let mut out = 0;

    'spawn: loop {
        let mut sand = source;

        while grid.in_bounds(sand) {
            let (down, left, right) = (
                grid.get(sand.go_dir(Down)),
                grid.get(sand.go_dir(Down).go_dir(Left)),
                grid.get(sand.go_dir(Down).go_dir(Right)),
            );

            match (down, left, right) {
                (Some(&Tile::Air), _, _) => sand = sand.go_dir(Down),
                (None, _, _) => break 'spawn,

                (_, Some(&Tile::Air), _) => sand = sand.go_dir(Down).go_dir(Left),
                (_, None, _) => break 'spawn,

                (_, _, Some(&Tile::Air)) => sand = sand.go_dir(Down).go_dir(Right),
                (_, _, None) => break 'spawn,

                _ => {
                    grid[sand] = Tile::Sand;
                    out += 1;
                    continue 'spawn;
                }
            }
        }

        break;
    }

    dbg!(&grid);
    dbg!(out);
}

fn part2() {
    let (mut grid, start) = create_grid_2();
    let source = Coord::new(start, 0);
    let mut out = 0;

    'spawn: loop {
        let mut sand = source;
        out += 1;

        while grid.in_bounds(sand) {
            let (down, left, right) = (
                grid.get(sand.go_dir(Down)),
                grid.get(sand.go_dir(Down).go_dir(Left)),
                grid.get(sand.go_dir(Down).go_dir(Right)),
            );

            match (down, left, right) {
                (Some(&Tile::Air), _, _) => sand = sand.go_dir(Down),

                (_, Some(&Tile::Air), _) => sand = sand.go_dir(Down).go_dir(Left),

                (_, _, Some(&Tile::Air)) => sand = sand.go_dir(Down).go_dir(Right),

                _ if sand == source => break 'spawn,

                _ => {
                    grid[sand] = Tile::Sand;
                    continue 'spawn;
                }
            }
        }

        break;
    }

    // dbg!(&grid);
    dbg!(out);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "o"),
        }
    }
}

type Path = Vec<Coord>;

fn create_grid_1() -> (Grid<Tile>, usize) {
    let mut paths = parse_paths();

    let min_x = paths
        .iter()
        .flat_map(|p| p.iter().map(|c| c.x))
        .min()
        .unwrap();

    let max_x = paths
        .iter()
        .flat_map(|p| p.iter().map(|c| c.x))
        .max()
        .unwrap();

    let height = paths
        .iter()
        .flat_map(|p| p.iter().map(|c| c.y))
        .max()
        .unwrap()
        + 1;

    let width = max_x - min_x + 1;
    let start = 500 - min_x;

    for path in &mut paths {
        for c in path {
            c.x -= min_x;
        }
    }

    let mut grid = Grid::new_with(Tile::Air, width, height);

    for path in paths {
        for (c1, c2) in path.iter().zip(&path[1..]) {
            if c1.x == c2.x {
                for y in min(c1.y, c2.y)..=max(c1.y, c2.y) {
                    let c = Coord::new(c1.x, y);
                    grid[c] = Tile::Rock;
                }
            } else {
                for x in min(c1.x, c2.x)..=max(c1.x, c2.x) {
                    let c = Coord::new(x, c1.y);
                    grid[c] = Tile::Rock;
                }
            }
        }
    }

    (grid, start)
}

fn create_grid_2() -> (Grid<Tile>, usize) {
    let paths = parse_paths();

    let height = paths
        .iter()
        .flat_map(|p| p.iter().map(|c| c.y))
        .max()
        .unwrap()
        + 1
        + 2;

    let width = 1000;
    let start = 500;

    let mut grid = Grid::new_with(Tile::Air, width, height);

    for path in paths {
        for (c1, c2) in path.iter().zip(&path[1..]) {
            if c1.x == c2.x {
                for y in min(c1.y, c2.y)..=max(c1.y, c2.y) {
                    let c = Coord::new(c1.x, y);
                    grid[c] = Tile::Rock;
                }
            } else {
                for x in min(c1.x, c2.x)..=max(c1.x, c2.x) {
                    let c = Coord::new(x, c1.y);
                    grid[c] = Tile::Rock;
                }
            }
        }
    }

    for x in 0..width {
        let c = Coord::new(x, height - 1);
        grid[c] = Tile::Rock;
    }

    (grid, start)
}

fn parse_paths() -> Vec<Path> {
    INPUT
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|s| {
                    let mut si = s.split(',');
                    let x = si.next().unwrap().parse().unwrap();
                    let y = si.next().unwrap().parse().unwrap();
                    Coord::new(x, y)
                })
                .collect()
        })
        .collect()
}
