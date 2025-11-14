use std::collections::VecDeque;

pub fn run() {
    part2();
}

const INPUT: &str = include_str!("in");

fn part1() {
    let mut monkeys = parse_inputs(INPUT);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(mut worry) = monkeys[i].items.pop_front() {
                worry = monkeys[i].op.eval(worry);
                worry /= 3;

                let trown_to = monkeys[i].test.eval(worry);
                monkeys[trown_to].items.push_back(worry);
                monkeys[i].inspected += 1;
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspected);

    let out: u64 = monkeys
        .into_iter()
        .rev()
        .take(2)
        .map(|m| m.inspected)
        .product();

    dbg!(out);
}

fn part2() {
    let mut monkeys = parse_inputs(INPUT);

    let lcm: u64 = monkeys.iter().map(|m| m.test.div_by).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(mut worry) = monkeys[i].items.pop_front() {
                worry = monkeys[i].op.eval(worry);
                worry %= lcm;

                let trown_to = monkeys[i].test.eval(worry);
                monkeys[trown_to].items.push_back(worry);
                monkeys[i].inspected += 1;
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspected);

    let out: u64 = monkeys
        .into_iter()
        .rev()
        .take(2)
        .map(|m| m.inspected)
        .product();

    dbg!(out);
}

#[derive(Debug)]
struct Monkey {
    inspected: u64,
    items: VecDeque<u64>,
    op: Op,
    test: Test,
}

#[derive(Debug)]
struct Op {
    mul: bool,
    n: Option<u64>,
}

impl Op {
    fn eval(&self, n: u64) -> u64 {
        let m = self.n.unwrap_or(n);
        if self.mul { n * m } else { n + m }
    }
}

#[derive(Debug)]
struct Test {
    div_by: u64,
    t: usize,
    f: usize,
}

impl Test {
    fn eval(&self, n: u64) -> usize {
        if n % self.div_by == 0 { self.t } else { self.f }
    }
}

fn parse_inputs(s: &str) -> Vec<Monkey> {
    s.split("\n\n")
        .map(|s| {
            let mut ls = s.lines().skip(1);

            let items = ls
                .next()
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();

            let op = ls
                .next()
                .unwrap()
                .split("= old ")
                .nth(1)
                .map(|s| {
                    let mul = s.chars().next().unwrap() == '*';
                    let n = s.split_at(2).1.parse().ok();
                    Op { mul, n }
                })
                .unwrap();

            let div_by = ls
                .next()
                .unwrap()
                .split("  Test: divisible by ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            let t = ls
                .next()
                .unwrap()
                .split("    If true: throw to monkey ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            let f = ls
                .next()
                .unwrap()
                .split("    If false: throw to monkey ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            let test = Test { div_by, t, f };

            Monkey {
                inspected: 0,
                items,
                op,
                test,
            }
        })
        .collect()
}
