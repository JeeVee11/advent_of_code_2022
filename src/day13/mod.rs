use std::{cmp::Ordering, iter, str::Chars};

pub fn run() {
    part2();
}

const INPUT: &str = include_str!("in");

fn part1() {
    let out: usize = parse_lists()
        .map(|(l1, l2)| check_lists(&l1, &l2).unwrap())
        .enumerate()
        .filter_map(|(i, in_order)| in_order.then_some(i + 1))
        .sum();

    dbg!(out);
}

fn part2() {
    let mut lists: Vec<_> = parse_lists().flat_map(|(l1, l2)| [l1, l2]).collect();

    let div1 = vec![ListItem::List(vec![ListItem::Int(6)])];
    let div2 = vec![ListItem::List(vec![ListItem::Int(2)])];

    lists.push(div1.clone());
    lists.push(div2.clone());

    lists.sort_by(|l1, l2| match check_lists(l1, l2) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    });

    let div1_i = lists
        .iter()
        .enumerate()
        .find_map(|(i, l)| (l == &div1).then_some(i + 1))
        .unwrap();

    let div2_i = lists
        .iter()
        .enumerate()
        .find_map(|(i, l)| (l == &div2).then_some(i + 1))
        .unwrap();

    dbg!(div1_i * div2_i);
}

fn check_lists(l1: &List, l2: &List) -> Option<bool> {
    for (li1, li2) in l1.into_iter().zip(l2) {
        match (li1, li2) {
            (ListItem::Int(a), ListItem::Int(b)) => {
                if a < b {
                    return Some(true);
                } else if a > b {
                    return Some(false);
                }
            }

            (ListItem::Int(n), ListItem::List(l2)) => {
                if let Some(in_order) = check_lists(&vec![ListItem::Int(*n)], l2) {
                    return Some(in_order);
                }
            }

            (ListItem::List(l1), ListItem::Int(n)) => {
                if let Some(in_order) = check_lists(l1, &vec![ListItem::Int(*n)]) {
                    return Some(in_order);
                }
            }

            (ListItem::List(l1), ListItem::List(l2)) => {
                if let Some(in_order) = check_lists(l1, l2) {
                    return Some(in_order);
                }
            }
        }
    }

    if l1.len() < l2.len() {
        return Some(true);
    } else if l1.len() > l2.len() {
        return Some(false);
    }

    None
}

fn parse_lists() -> impl Iterator<Item = (List, List)> {
    INPUT.split("\n\n").map(|pair| {
        let mut ps = pair.lines();
        let l1 = ListParser::new(ps.next().unwrap()).parse_all();
        let l2 = ListParser::new(ps.next().unwrap()).parse_all();
        (l1, l2)
    })
}

struct ListParser {
    chars: Chars<'static>,
}

impl ListParser {
    fn new(s: &'static str) -> Self {
        let mut chars = s.chars();
        assert!(chars.next() == Some('['));
        Self { chars }
    }

    fn parse_all(mut self) -> List {
        let l = self.parse_list();
        assert!(self.chars.next().is_none());
        l
    }

    fn parse_list(&mut self) -> List {
        let mut list = vec![];
        let mut comma = true;

        while comma {
            comma = false;

            match self.chars.next().unwrap() {
                c @ '0'..='9' => {
                    let n = iter::once(c)
                        .chain(self.chars.by_ref().take_while(|c| {
                            if *c == ',' {
                                comma = true;
                            }

                            c.is_ascii_digit()
                        }))
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    list.push(ListItem::Int(n));
                }

                '[' => {
                    let l = self.parse_list();
                    list.push(ListItem::List(l));
                    comma = self.chars.next() == Some(',');
                }

                ']' => {}

                _ => unreachable!(),
            }
        }

        list
    }
}

type List = Vec<ListItem>;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListItem {
    Int(u32),
    List(List),
}
