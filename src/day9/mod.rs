use std::collections::HashSet;

pub fn run() {
    part2();
}

const INPUT: &str = include_str!("in");

fn part2() {
    let mut coords: [(i32, i32); _] = [(0, 0); 10];
    let mut visited = HashSet::from([(0, 0)]);

    for l in INPUT.lines() {
        let dir = l.chars().next().unwrap();
        let n = l.split_whitespace().last().unwrap().parse().unwrap();

        for _ in 0..n {
            match dir {
                'U' => coords[0].1 += 1,
                'D' => coords[0].1 -= 1,
                'R' => coords[0].0 += 1,
                'L' => coords[0].0 -= 1,
                _ => unreachable!(),
            }

            for i in 1..coords.len() {
                let x_dist = (coords[i - 1].0 - coords[i].0).abs();
                let y_dist = (coords[i - 1].1 - coords[i].1).abs();

                if x_dist > 1 || y_dist > 1 || x_dist + y_dist > 2 {
                    coords[i].0 += (coords[i - 1].0 - coords[i].0).signum();
                    coords[i].1 += (coords[i - 1].1 - coords[i].1).signum();
                }

                if i == coords.len() - 1 {
                    visited.insert(coords[i]);
                }
            }
        }
    }

    dbg!(visited.len());
}

fn part1() {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited = HashSet::from([(0, 0)]);

    for l in INPUT.lines() {
        let dir = l.chars().next().unwrap();
        let n = l.split_whitespace().last().unwrap().parse().unwrap();

        for _ in 0..n {
            match dir {
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                'R' => head.0 += 1,
                'L' => head.0 -= 1,
                _ => unreachable!(),
            }

            let x_dist = (head.0 - tail.0).abs();
            let y_dist = (head.1 - tail.1).abs();

            if x_dist > 1 || y_dist > 1 || x_dist + y_dist > 2 {
                tail.0 += (head.0 - tail.0).signum();
                tail.1 += (head.1 - tail.1).signum();
                visited.insert(tail);
            }
        }
    }

    dbg!(visited.len());
}
