fn run() {
    let input = include_str!("in");

    let mut tree = FileTree::Dir {
        name: "/",
        content: vec![],
    };

    let mut path = vec![];

    for cmd in input.split('$').skip(1) {
        let mut words = cmd.split_ascii_whitespace();
        match words.next().unwrap() {
            "cd" => {
                let next = words.next().unwrap();
                if next == ".." {
                    path.pop();
                } else {
                    path.push(next);
                }
            }

            "ls" => {
                for line in cmd.lines().skip(1) {
                    let words: Vec<_> = line.split_ascii_whitespace().collect();

                    let ft = match words.as_slice() {
                        &["dir", name] => FileTree::Dir {
                            name,
                            content: vec![],
                        },

                        &[size, name] => FileTree::File {
                            name,
                            size: size.parse().unwrap(),
                        },
                        _ => unreachable!(),
                    };

                    tree.add(&path, ft);
                }
            }

            _ => unreachable!(),
        }
    }

    let mut out = vec![];

    // dbg!(&tree);
    let space_used = calc_size(&tree, &mut out);
    let unused_space = 70000000 - space_used;
    let space_needed = 30000000 - unused_space;

    let size = out
        .into_iter()
        .filter(|&s| s >= space_needed)
        .min()
        .unwrap();

    dbg!(size);
}

fn calc_size(tree: &FileTree, total: &mut Vec<usize>) -> usize {
    match tree {
        FileTree::Dir { content, .. } => {
            let size = content.iter().map(|ft| calc_size(ft, total)).sum();
            total.push(size);
            size
        }
        FileTree::File { size, .. } => *size,
    }
}

type Path = Vec<&'static str>;

#[derive(Debug)]
enum FileTree {
    Dir {
        name: &'static str,
        content: Vec<FileTree>,
    },

    File {
        name: &'static str,
        size: usize,
    },
}

impl FileTree {
    fn add(&mut self, path: &Path, ft: FileTree) {
        let mut cur = self;

        for dir in path.iter().skip(1) {
            let FileTree::Dir { content, .. } = cur else {
                unreachable!();
            };

            let sub_dir = content
                .iter_mut()
                .find(|sub| matches!(sub, FileTree::Dir { name, .. } if name == dir))
                .unwrap();

            cur = sub_dir;
        }

        let FileTree::Dir { name, content } = cur else {
            unreachable!();
        };

        assert_eq!(name, path.last().unwrap());

        content.push(ft);
    }
}
