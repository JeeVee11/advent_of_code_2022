pub fn run() {
    part2();
}

fn part2() {
    let input = include_bytes!("in");

    for i in 0..input.len() - 1 {
        if i >= 14 && !contains_dup(&input[i - 14..i]) {
            dbg!(i);
            return;
        }
    }
}

fn part1() {
    let input = include_bytes!("in");

    let mut window = [0; 4];
    let mut i = 0;

    for (j, &c) in input[0..input.len() - 1].iter().enumerate() {
        window[i] = c;

        i += 1;
        if i > 3 {
            i = 0;
        }

        if j >= 4 && !contains_dup(&window) {
            dbg!(j + 1);
            return;
        }
    }
}

fn contains_dup(w: &[u8]) -> bool {
    for (i, c) in w[0..w.len() - 1].iter().enumerate() {
        if w[i + 1..].contains(c) {
            return true;
        }
    }

    false
}
