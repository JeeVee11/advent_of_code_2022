pub fn run() {
    part1();
}

const INPUT: &str = include_str!("in");
const WIDTH: usize = 7;

fn part1() {
    use Tile::*;

    let mut jets = INPUT.trim().chars().cycle();
    let mut chamber: Vec<[Tile; WIDTH]> = vec![];

    for i in 0..2022 {
        let rock_top = chamber
            .iter()
            .enumerate()
            .rfind(|(_, line)| line.iter().any(|t| *t == Rock))
            .map(|(i, _)| i as isize)
            .unwrap_or(-1);

        let to_push = (chamber.len() as isize - 1) - (rock_top + 3);

        if to_push > 0 {
            for _ in 0..to_push {
                chamber.pop();
            }
        } else {
            for _ in to_push..0 {
                chamber.push(Layer::default());
            }
        }

        let (mut rock_b, mut rock_t) = match i % 5 {
            0 => {
                chamber.push([Air, Air, Falling, Falling, Falling, Falling, Air]);
                (chamber.len() - 1, chamber.len() - 1)
            }

            1 => {
                chamber.push([Air, Air, Air, Falling, Air, Air, Air]);
                chamber.push([Air, Air, Falling, Falling, Falling, Air, Air]);
                chamber.push([Air, Air, Air, Falling, Air, Air, Air]);
                (chamber.len() - 3, chamber.len() - 1)
            }

            2 => {
                chamber.push([Air, Air, Falling, Falling, Falling, Air, Air]);
                chamber.push([Air, Air, Air, Air, Falling, Air, Air]);
                chamber.push([Air, Air, Air, Air, Falling, Air, Air]);
                (chamber.len() - 3, chamber.len() - 1)
            }

            3 => {
                chamber.push([Air, Air, Falling, Air, Air, Air, Air]);
                chamber.push([Air, Air, Falling, Air, Air, Air, Air]);
                chamber.push([Air, Air, Falling, Air, Air, Air, Air]);
                chamber.push([Air, Air, Falling, Air, Air, Air, Air]);
                (chamber.len() - 4, chamber.len() - 1)
            }

            4 => {
                chamber.push([Air, Falling, Falling, Air, Air, Air, Air]);
                chamber.push([Air, Falling, Falling, Air, Air, Air, Air]);
                (chamber.len() - 2, chamber.len() - 1)
            }

            _ => unreachable!(),
        };

        let set_rock = |chamber: &mut [[Tile; 7]], rock_b: usize, rock_t: usize| {
            for line_i in rock_b..=rock_t {
                for ci in 0..WIDTH {
                    if chamber[line_i][ci] == Falling {
                        chamber[line_i][ci] = Rock;
                    }
                }
            }
        };

        'outer: loop {
            let jet = jets.next().unwrap();

            match jet {
                '>' => {
                    let can_push = chamber[rock_b..=rock_t].iter().all(|line| {
                        let fst = line
                            .iter()
                            .enumerate()
                            .rfind(|(_, t)| **t == Falling)
                            .unwrap()
                            .0;
                        fst != WIDTH - 1 && line[fst + 1] != Rock
                    });

                    if can_push {
                        for line_i in rock_b..=rock_t {
                            for i in (0..WIDTH - 1).rev() {
                                if chamber[line_i][i] == Falling {
                                    chamber[line_i][i + 1] = Falling;
                                    chamber[line_i][i] = Air;
                                }
                            }
                        }
                    }
                }

                '<' => {
                    let can_push = chamber[rock_b..=rock_t].iter().all(|line| {
                        let fst = line
                            .iter()
                            .enumerate()
                            .find(|(_, t)| **t == Falling)
                            .unwrap()
                            .0;
                        fst != 0 && line[fst - 1] != Rock
                    });

                    if can_push {
                        for line_i in rock_b..=rock_t {
                            for i in 1..WIDTH {
                                if chamber[line_i][i] == Falling {
                                    chamber[line_i][i - 1] = Falling;
                                    chamber[line_i][i] = Air;
                                }
                            }
                        }
                    }
                }

                _ => unreachable!(),
            }

            let can_fall = rock_b > 0
                && chamber[rock_b..=rock_t]
                    .iter()
                    .zip(&chamber[rock_b - 1..=rock_t - 1])
                    .all(|(line, line_b)| {
                        line.iter()
                            .zip(line_b)
                            .all(|(t, tb)| !(*t == Falling && *tb == Rock))
                    });

            if can_fall {
                for line_i in rock_b..=rock_t {
                    for ci in 0..WIDTH {
                        if chamber[line_i][ci] == Falling {
                            chamber[line_i][ci] = Air;
                            chamber[line_i - 1][ci] = Falling;
                        }
                    }
                }
            } else {
                set_rock(&mut chamber, rock_b, rock_t);
                break 'outer;
            }

            rock_b -= 1;
            rock_t -= 1;
        }
    }

    print_chamber(&chamber);

    let out = chamber
        .iter()
        .enumerate()
        .rfind(|(_, line)| line.iter().any(|t| *t == Rock))
        .map(|(i, _)| i as isize)
        .unwrap()
        + 1;

    dbg!(out);
}

fn part2() {}

type Layer = [Tile; 7];

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum Tile {
    #[default]
    Air,
    Rock,
    Falling,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Falling => write!(f, "@"),
        }
    }
}

fn print_chamber(ch: &[Layer]) {
    for (i, l) in ch.iter().enumerate().rev() {
        print!("{i}\t\t|");
        for t in l {
            print!("{t:?}");
        }
        println!("|");
    }

    println!("+-------+\n");
}
