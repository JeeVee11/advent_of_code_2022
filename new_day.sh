#!/usr/bin/env bash
set -e

dir="src/$1"

mkdir -p $dir

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

head -n -3 "src/main.rs" > tmp
sed -i "2i mod $1;" tmp
echo -e "fn main() {\n    $1::run();\n}" >> tmp
mv tmp "src/main.rs"

echo "Created $1"
