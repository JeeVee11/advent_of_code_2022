use std::collections::{HashMap, VecDeque};

use aoc_util::Grid;

pub fn run() {
    part2();
}

const INPUT: &str = include_str!("in");

fn part1() {
    let (graph, mapping) = parse_input();
    let dist = find_distances(&graph);

    let mut mem = HashMap::new();
    let i = mapping["AA"];
    let t = 31;
    let o: Vec<_> = graph.iter().map(|(fr, _)| *fr == 0).collect();

    let out = flow_rate(&graph, &dist, &mut mem, i, t, o);

    dbg!(out);
}

fn part2() {
    let (graph, mapping) = parse_input();
    let dist = find_distances(&graph);

    let mut mem = HashMap::new();
    let i = mapping["AA"];
    let e = i;
    let t = 27;
    let u = t;
    let o: Vec<_> = graph.iter().map(|(fr, _)| *fr == 0).collect();

    let out = flow_rate_elephant(&graph, &dist, &mut mem, i, e, t, u, o);

    dbg!(out);
}

type Name = &'static str;
type Mapping = HashMap<Name, usize>;
type Graph = Vec<(u32, Vec<usize>)>;

/*
    E(i, e, t, u, o) = 0 if (t = 0 and u == 0) or o = G
    E(i, e, t, u, o) = F(i, t, o) if u == 0
    E(i, e, t, u, o) = F(e, u, o) if t == 0
    E(i, e, t, u, o) = max (v, w in G \ o, v != w) { F(v, w, t - D(i, v) - 1, u - D(e, w) - 1, o U i U e) } + R(i) * (t - 1) + R(e) * (u - 1)
*/
fn flow_rate_elephant(
    graph: &Graph,
    dist: &Grid<u32>,
    mem: &mut HashMap<(usize, usize, u32, u32, Vec<bool>), u32>,
    i: usize,
    e: usize,
    t: u32,
    u: u32,
    mut o: Vec<bool>,
) -> u32 {
    // eprintln!("{} {}", mem.len(), o.iter().filter(|b| **b).count());
    if (t as i32 <= 0) && (u as i32 <= 0) {
        return 0;
    } else if t as i32 <= 0 {
        let mut mem = HashMap::new();
        return flow_rate(graph, dist, &mut mem, e, u, o);
    } else if u as i32 <= 0 {
        let mut mem = HashMap::new();
        return flow_rate(graph, dist, &mut mem, i, t, o);
    }

    if let Some(fr) = mem.get(&(i, e, t, u, o.clone())) {
        return *fr;
    }

    o[i] = true;
    o[e] = true;

    let reachable_i: Vec<_> = dist[i]
        .iter()
        .copied()
        .enumerate()
        .filter(|(v, d)| !o[*v] && *d < t)
        .collect();

    let reachable_e: Vec<_> = dist[i]
        .iter()
        .copied()
        .enumerate()
        .filter(|(v, d)| !o[*v] && *d < u)
        .collect();

    let fr = reachable_i
        .into_iter()
        .flat_map(|(v, d1)| {
            reachable_e
                .clone()
                .into_iter()
                .map(move |(w, d2)| (v, d1, w, d2))
        })
        .filter(|(v, _, w, _)| v != w)
        .map(|(v, d1, w, d2)| {
            flow_rate_elephant(graph, dist, mem, v, w, t - d1 - 1, u - d2 - 1, o.clone())
        })
        .max()
        .unwrap_or_default();

    let fr = fr + graph[i].0 * (t - 1) + graph[e].0 * (u - 1);
    mem.insert((i, e, t, u, o), fr);

    fr
}

/*
    F(i, t, o) = 0 if t = 0 or o = G
    F(i, t, o) = max v in G \ o { F(v, t - D(i, v) - 1, o U i) } + R(i) * (t - 1)
*/
fn flow_rate(
    graph: &Graph,
    dist: &Grid<u32>,
    mem: &mut HashMap<(usize, u32, Vec<bool>), u32>,
    i: usize,
    t: u32,
    mut o: Vec<bool>,
) -> u32 {
    // dbg!(mem.len(), t);
    if (t as i32) <= 0 {
        return 0;
    }

    if let Some(fr) = mem.get(&(i, t, o.clone())) {
        return *fr;
    }

    o[i] = true;

    let fr = dist[i]
        .iter()
        .enumerate()
        .filter(|(v, d)| !o[*v] && **d < t)
        .map(|(v, d)| flow_rate(graph, dist, mem, v, t - *d - 1, o.clone()))
        .max()
        .unwrap_or_default();

    let fr = fr + graph[i].0 * (t - 1);
    mem.insert((i, t, o), fr);

    fr
}

fn parse_input() -> (Graph, Mapping) {
    let g: HashMap<Name, (usize, u32, Vec<Name>)> = INPUT
        .lines()
        .enumerate()
        .map(|(i, line)| {
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

            (name, (i, fr, connects))
        })
        .collect();

    let mut graph = vec![(0, vec![]); g.len()];

    for (i, fr, ns) in g.values() {
        let ns = ns.iter().map(|n| g[n].0).collect();
        graph[*i] = (*fr, ns);
    }

    (graph, g.into_iter().map(|(k, v)| (k, v.0)).collect())
}

fn find_distances(graph: &Graph) -> Grid<u32> {
    let mut dist = Grid::new_with(u32::MAX, graph.len(), graph.len());

    for (i, _) in graph.iter().enumerate() {
        let mut q = VecDeque::from([i]);
        dist[i][i] = 0;

        while let Some(j) = q.pop_front() {
            for n in &graph[j].1 {
                if dist[i][*n] == u32::MAX {
                    dist[i][*n] = dist[i][j] + 1;
                    q.push_back(*n);
                }
            }
        }
    }

    dist
}
