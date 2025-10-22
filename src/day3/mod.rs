pub fn run() {
    part2();
}

fn part2() {
    let input = include_str!("in");

    let out: u32 = input
        .lines()
        .map(str::as_bytes)
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|chunks| {
            chunks[0]
                .iter()
                .find(|a| chunks[1].contains(a) && chunks[2].contains(a))
                .map(|a| {
                    let s = if (*a as char).is_ascii_lowercase() {
                        a - b'a' + 1
                    } else {
                        a - b'A' + 27
                    };
                    s as u32
                })
                .unwrap_or_default()
        })
        .sum();

    dbg!(out);
}

#[derive(Debug)]
struct RuckSack<'a> {
    fst: &'a [u8],
    snd: &'a [u8],
}

fn part1() {
    let input = include_str!("in");

    let sacks = input.lines().map(|l| {
        let l = l.as_bytes();
        let s = l.len() / 2;
        let (fst, snd) = l.split_at(s);
        RuckSack { fst, snd }
    });

    let mut out = 0;

    for sack in sacks {
        'outer: for &a in sack.fst {
            for &b in sack.snd {
                if a == b {
                    let s = if (a as char).is_ascii_lowercase() {
                        a - b'a' + 1
                    } else {
                        a - b'A' + 27
                    };

                    out += s as u32;

                    break 'outer;
                }
            }
        }
    }

    dbg!(out);
}
