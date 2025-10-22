pub fn run() {
    part2();
}

type Stacks = Vec<Stack>;
type Stack = Vec<char>;

#[derive(Debug)]
struct Step {
    amount: usize,
    from: usize,
    to: usize,
}

fn part2() {
    let input = include_str!("in");
    let mut input = input.split("\n\n");

    let mut stacks = parse_stacks(input.next().unwrap());
    let steps = parse_steps(input.next().unwrap());

    for Step { amount, from, to } in steps {
        let i = stacks[from].len() - amount;
        let mut tmp = stacks[from].split_off(i);
        stacks[to].append(&mut tmp);
    }

    let out: String = stacks.into_iter().map(|mut s| s.pop().unwrap()).collect();
    dbg!(out);
}

fn part1() {
    let input = include_str!("in");
    let mut input = input.split("\n\n");

    let mut stacks = parse_stacks(input.next().unwrap());
    let steps = parse_steps(input.next().unwrap());

    for Step { amount, from, to } in steps {
        for _ in 0..amount {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }

    let out: String = stacks.into_iter().map(|mut s| s.pop().unwrap()).collect();
    dbg!(out);
}

fn parse_steps(s: &str) -> impl Iterator<Item = Step> {
    s.lines().map(|l| {
        let mut l = l.split_ascii_whitespace();

        let amount = l.by_ref().nth(1).unwrap().parse().unwrap();
        let from = l.by_ref().nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = l.by_ref().nth(1).unwrap().parse::<usize>().unwrap() - 1;

        Step { amount, from, to }
    })
}

fn parse_stacks(s: &str) -> Stacks {
    let stack_amount = (s.lines().next().unwrap().len() + 1) / 4;

    let mut stacks = vec![Vec::new(); stack_amount];

    for line in s.lines().rev().skip(1).map(str::as_bytes) {
        for i in 0..stack_amount {
            let c = line[i * 4 + 1] as char;
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    stacks
}
