#!/usr/bin/env bash
set -e

dir="src/$1"

mkdir $dir

touch "$dir/in"

cat <<EOF > "$dir/mod.rs"
pub fn run() {
    part1();
}

const INPUT: &str = include_str!("in");

fn part1() {
  
}

fn part2() {
  
}
EOF

echo "Created $1"
