pub fn run() {
    part2();
}

const INPUT: &str = include_str!("in");

fn part1() {
    let mut out = 0;
    let mut x = 1;
    let mut clock = 0;
    let times = [20, 60, 100, 140, 180, 220];

    for l in INPUT.lines() {
        let (instr, ns) = l.split_at(4);
        // eprintln!("clk: {clock}, x: {x}");

        let check = move |clock, out: &mut i32| {
            if times.contains(&clock) {
                *out += clock * x;
                // dbg!(clock, out, x);
            }
        };

        match instr {
            "addx" => {
                let n: i32 = ns[1..].parse().unwrap();
                x += n;
                clock += 1;
                check(clock, &mut out);
                clock += 1;
            }

            "noop" => clock += 1,

            _ => unreachable!(),
        }

        check(clock, &mut out);
    }

    dbg!(out);
}

fn part2() {
    const W: i32 = 40;
    const H: i32 = 6;

    let mut x = 1;
    let mut clk = 0;

    for l in INPUT.lines() {
        let (instr, ns) = l.split_at(4);

        let (i, j) = (clk % W, clk / W);

        if i == x - 1 || i == x || i == x + 1 {
            print!("#");
        } else {
            print!(".");
        }

        if i == W - 1 {
            println!();
        }

        if j == H - 1 {
            println!();
        }

        match instr {
            "addx" => {
                // let n: i32 = ns[1..].parse().unwrap();
                // x += n;
                clk += 2;
            }

            "noop" => clk += 1,

            _ => unreachable!(),
        }
    }
}
