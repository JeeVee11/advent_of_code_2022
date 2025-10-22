pub fn run() {
    part2();
}

fn part2() {
    let input = include_str!("in");

    let out: u32 = input
        .lines()
        .map(|l| {
            let mut l = l.split(',');
            let r1 = parse_range(l.next().unwrap());
            let r2 = parse_range(l.next().unwrap());

            let overlap = (r1.0 <= r2.0 && r1.1 >= r2.0) || (r2.0 <= r1.0 && r2.1 >= r1.0);

            u32::from(overlap)
        })
        .sum();

    dbg!(out);
}

fn part1() {
    let input = include_str!("in");

    let out: u32 = input
        .lines()
        .map(|l| {
            let mut l = l.split(',');
            let r1 = parse_range(l.next().unwrap());
            let r2 = parse_range(l.next().unwrap());

            let overlap = (r1.0 <= r2.0 && r1.1 >= r2.1) || (r2.0 <= r1.0 && r2.1 >= r1.1);

            u32::from(overlap)
        })
        .sum();

    dbg!(out);
}

fn parse_range(s: &str) -> (u32, u32) {
    let mut s = s.split('-');
    let n1 = s.next().unwrap().parse().unwrap();
    let n2 = s.next().unwrap().parse().unwrap();

    (n1, n2)
}
