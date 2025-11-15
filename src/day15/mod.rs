use std::ops::RangeInclusive;

pub fn run() {
    part2();
}

const INPUT: &str = include_str!("in");

fn part1() {
    let y_check = 2000000;
    let input: Vec<_> = parse_input().collect();
    let ranges_union = calc_ranges(y_check, &input);
    let out: isize = ranges_union.into_iter().map(|r| r.end() - r.start()).sum();

    dbg!(out);
}

fn part2() {
    let mut input: Vec<_> = parse_input()
        .map(|(sc, bc)| {
            let dist = (sc.x.abs_diff(bc.x) + sc.y.abs_diff(bc.y)) as isize;
            (sc, dist)
        })
        .collect();

    input.sort_unstable_by_key(|(c, dist)| c.x - dist);

    let max = 4000000;

    for y_check in 0..=max {
        let mut ranges: Vec<_> = input
            .iter()
            .filter_map(|(sc, dist)| {
                let x_left = dist - (sc.y - y_check).abs();
                (x_left > 0).then(|| (sc.x - x_left, sc.x + x_left))
            })
            .collect();

        ranges.sort_unstable_by_key(|r| r.0);

        let (_, mut end) = ranges[0];

        for (s, e) in ranges.into_iter().skip(1) {
            if s > max {
                break;
            }

            if s > end {
                let x = s - 1;
                if x >= 0 {
                    dbg!(x * 4000000 + y_check);
                    return;
                }
            } else if e > end {
                end = e;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

fn calc_ranges<'a>(
    y_check: isize,
    coords: impl IntoIterator<Item = &'a (Coord, Coord)>,
) -> Vec<RangeInclusive<isize>> {
    let mut ranges: Vec<_> = coords
        .into_iter()
        .filter_map(|(sc, bc)| {
            let dist = (sc.x.abs_diff(bc.x) + sc.y.abs_diff(bc.y)) as isize;
            let x_left = dist - (sc.y - y_check).abs();
            (x_left > 0).then(|| sc.x - x_left..=sc.x + x_left)
        })
        .collect();

    ranges.sort_unstable_by_key(|r| *r.start());

    let mut ranges_union = vec![];
    let fst = ranges.remove(0);
    let mut start = *fst.start();
    let mut end = *fst.end();

    for range in ranges {
        if *range.start() > end {
            ranges_union.push(start..=end);
            start = *range.start();
            end = *range.end();
        } else if *range.end() > end {
            end = *range.end();
        }
    }

    ranges_union.push(start..=end);

    ranges_union
}

fn parse_input() -> impl Iterator<Item = (Coord, Coord)> {
    INPUT.lines().map(|line| {
        let mut line = line.split(": ");
        let mut sl = line
            .next()
            .unwrap()
            .split("Sensor at ")
            .nth(1)
            .unwrap()
            .split(", ");

        let sx = sl.next().unwrap().split_at(2).1.parse().unwrap();
        let sy = sl.next().unwrap().split_at(2).1.parse().unwrap();

        let mut bl = line
            .next()
            .unwrap()
            .split("closest beacon is at ")
            .nth(1)
            .unwrap()
            .split(", ");

        let bx = bl.next().unwrap().split_at(2).1.parse().unwrap();
        let by = bl.next().unwrap().split_at(2).1.parse().unwrap();

        (Coord::new(sx, sy), Coord::new(bx, by))
    })
}
