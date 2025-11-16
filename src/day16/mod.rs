use std::{
    collections::{HashMap, HashSet},
    iter,
};

pub fn run() {
    part1();
}

const INPUT: &str = include_str!("in");

fn part1() {
    let dag = parse_input();
    let mut dist = HashMap::from([("AA", 0)]);
    let mut open = HashSet::new();

    // return;

    dfs("AA", 0, &dag, &mut open, &mut dist);
    dbg!(dist);
}

fn dfs(
    vert: Name,
    layer: usize,
    dag: &Dag,
    open: &mut HashSet<Name>,
    dist: &mut HashMap<Name, i32>,
) {
    if layer > 30 || vert == "END" {
        return;
    }

    dbg!(layer);

    let ns = &dag[layer][vert];
    for (name, cost) in ns {
        let cost = if open.contains(name) {
            0
        } else {
            *cost * (30 - (layer * 2) as u32)
        } as i32;

        if *dist.entry(name).or_insert(i32::MIN) < dist[vert] + cost {
            dist.insert(name, dist[vert] + cost);

            // if *name == vert {
            //     open.insert(name);
            // }

            dfs(name, layer + 1, dag, open, dist);
        }
    }
}

fn part2() {}

type Name = &'static str;
type Dag = Vec<Layer>;
type Layer = HashMap<Name, Vec<(Name, u32)>>;
type Graph = HashMap<Name, (u32, Vec<Name>)>;

fn parse_input() -> Dag {
    let graph: Graph = INPUT
        .lines()
        .map(|line| {
            let mut line = line.split("; ");
            let mut start = line.next().unwrap().split(" has flow rate=");

            let name = start.next().unwrap().split_at(6).1;
            let fr = start.next().unwrap().parse().unwrap();
            let connects = line
                .next()
                .unwrap()
                .split("to valve")
                .nth(1)
                .unwrap()
                .trim_start_matches(['s', ' '])
                .split(", ")
                .collect();

            (name, (fr, connects))
        })
        .collect();

    dbg!(&graph);

    let mut dag: Dag = vec![HashMap::new()];

    let start = "AA";
    let (fr, connects) = graph.get(start).unwrap();

    let edges = connects
        .iter()
        .map(|name| (*name, 0))
        .chain(iter::once((start, *fr)))
        .collect();

    dag[0].insert(start, edges);

    for i in 1..=30 {
        let mut layer = Layer::new();

        for name in dag[i - 1].values().flat_map(|v| v.iter().map(|(n, _)| *n)) {
            let edges = if i < 30 {
                let (fr, connects) = graph.get(name).unwrap();

                connects
                    .iter()
                    .map(|name| (*name, 0))
                    .chain(iter::once((name, *fr)))
                    .collect()
            } else {
                vec![("END", 0)]
            };

            layer.insert(name, edges);
        }

        dag.push(layer);
    }

    dbg!(&dag);

    for (i, l) in dag.iter().enumerate() {
        eprintln!("{i}: {}", l.len());
    }

    dag
}
