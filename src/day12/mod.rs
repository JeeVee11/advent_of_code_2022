use std::collections::VecDeque;

use crate::util::{Coord, Grid};

pub fn run() {
    part2();
}

const INPUT: &str = include_str!("in");

fn part1() {
    let width = INPUT.lines().next().unwrap().len();
    let height = INPUT.lines().count();
    let mut grid = Grid::new_with('0', width, height);

    let mut start = Coord::default();
    let mut end = Coord::default();

    for (y, l) in INPUT.lines().enumerate() {
        for (x, c) in l.char_indices() {
            let coord = Coord::new(x, y);

            let c = match c {
                'S' => {
                    start = coord;
                    'a'
                }

                'E' => {
                    end = coord;
                    'z'
                }

                c => c,
            };

            grid.set(coord, c);
        }
    }

    let mut q = VecDeque::from([(start, 0)]);
    let mut visited = Grid::new_with(false, width, height);

    'outer: while let Some((coord, steps)) = q.pop_front() {
        for (nc, &nv) in grid.neighbours(coord) {
            let nv = nv as u8;
            let v = grid[coord] as u8;

            if (nv < v || nv - v < 2) && !visited[nc] {
                if nc == end {
                    dbg!(steps + 1);
                    break 'outer;
                }

                q.push_back((nc, steps + 1));
                visited[nc] = true;
            }
        }
    }
}

fn part2() {
    let width = INPUT.lines().next().unwrap().len();
    let height = INPUT.lines().count();
    let mut grid = Grid::new_with('0', width, height);

    let mut start = Coord::default();

    for (y, l) in INPUT.lines().enumerate() {
        for (x, c) in l.char_indices() {
            let coord = Coord::new(x, y);

            let c = match c {
                'S' => 'a',

                'E' => {
                    start = coord;
                    'z'
                }

                c => c,
            };

            grid.set(coord, c);
        }
    }

    let mut q = VecDeque::from([(start, 0)]);
    let mut visited = Grid::new_with(false, width, height);

    'outer: while let Some((coord, steps)) = q.pop_front() {
        for (nc, &nv) in grid.neighbours(coord) {
            let nv = nv as u8;
            let v = grid[coord] as u8;

            if (nv > v || v - nv < 2) && !visited[nc] {
                if nv == b'a' {
                    dbg!(steps + 1);
                    break 'outer;
                }

                q.push_back((nc, steps + 1));
                visited[nc] = true;
            }
        }
    }
}
