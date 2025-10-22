use std::cmp::max;

pub fn run() {
    part2();
}

fn part2() {
    let input = include_str!("in");

    let forest: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();

    let mut out = 0;

    for y in 1..forest.len() - 1 {
        let row = &forest[y];

        for x in 1..row.len() - 1 {
            let t = row[x];

            let mut score = 1;

            let mut d = 0;
            for i in (0..y).rev() {
                d += 1;
                if forest[i][x] >= t {
                    break;
                }
            }
            score *= d;

            let mut d = 0;
            for i in y + 1..forest.len() {
                d += 1;
                if forest[i][x] >= t {
                    break;
                }
            }
            score *= d;

            let mut d = 0;
            for j in (0..x).rev() {
                d += 1;
                if forest[y][j] >= t {
                    break;
                }
            }
            score *= d;

            let mut d = 0;
            for j in x + 1..row.len() {
                d += 1;
                if forest[y][j] >= t {
                    break;
                }
            }
            score *= d;

            out = max(out, score);
        }
    }

    dbg!(out);
}

fn part1() {
    let input = include_str!("in");

    let forest: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();

    let mut out = 0;

    for y in 1..forest.len() - 1 {
        let row = &forest[y];

        for x in 1..row.len() - 1 {
            let t = row[x];

            let mut blocked = 0;

            for i in 0..y {
                if forest[i][x] >= t {
                    blocked += 1;
                    break;
                }
            }

            for i in y + 1..forest.len() {
                if forest[i][x] >= t {
                    blocked += 1;
                    break;
                }
            }

            for j in 0..x {
                if forest[y][j] >= t {
                    blocked += 1;
                    break;
                }
            }

            for j in x + 1..forest[0].len() {
                if forest[y][j] >= t {
                    blocked += 1;
                    break;
                }
            }

            if blocked < 4 {
                out += 1;
            }
        }
    }

    out += 2 * forest.len() + 2 * (forest[0].len() - 2);

    dbg!(out);
}
