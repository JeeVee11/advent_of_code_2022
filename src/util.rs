use std::{
    fmt::Debug,
    ops::{AddAssign, Index, IndexMut, SubAssign},
};

pub const MOVE_UP_AND_CLEAR: &str = "\x1b[H\x1b[J";

#[derive(Clone)]
pub struct Grid<T> {
    arr: Box<[T]>,
    width: usize,
}

impl<T: Copy> Grid<T> {
    pub fn new_with(v: T, width: usize, height: usize) -> Self {
        Self::new(vec![v; width * height].into_boxed_slice(), width)
    }
}

impl<T> Grid<T> {
    pub fn new(arr: Box<[T]>, width: usize) -> Self {
        Self { arr, width }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.arr.len() / self.width
    }

    pub fn in_bounds(&self, Coord { x, y }: Coord) -> bool {
        x < self.width && y < self.height()
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.arr.get(self.get_i(coord)?)
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        self.arr.get_mut(self.get_i(coord)?)
    }

    pub fn set(&mut self, coord: Coord, elem: T) {
        self.arr[self.get_i(coord).unwrap()] = elem;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter: self.arr.iter(),
            width: self.width,
            c: Coord::default(),
        }
    }

    pub fn neighbours(&self, coord: Coord) -> impl Iterator<Item = (Coord, &T)> {
        coord
            .neighbours()
            .filter_map(|c| self.get(c).map(|v| (c, v)))
    }

    pub fn neighbours_dir(&self, coord: Coord) -> impl Iterator<Item = (Coord, Dir, &T)> {
        coord
            .neighbours_with_dir()
            .filter_map(|(c, d)| self.get(c).map(|v| (c, d, v)))
    }

    fn get_i(&self, Coord { x, y }: Coord) -> Option<usize> {
        if x >= self.width || y >= self.height() {
            None
        } else {
            y.checked_mul(self.width).and_then(|i| i.checked_add(x))
        }
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.width;
        let end = start + self.width;
        &self.arr[start..end]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.width;
        let end = start + self.width;
        &mut self.arr[start..end]
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        self.get(index).expect("Index out of bounds")
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        self.get_mut(index).expect("Index out of bounds")
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            writeln!(f)?;
            for x in 0..self.width {
                write!(f, "{:?}", self[y][x])?;
            }
        }

        Ok(())
    }
}

pub struct Iter<'a, T> {
    iter: std::slice::Iter<'a, T>,
    width: usize,
    c: Coord,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.iter.next()?;
        let Coord { x, y } = self.c;

        self.c = Coord::new(x + 1, y);
        if self.c.x >= self.width {
            self.c.x = 0;
            self.c.y += 1;
        }

        Some((Coord::new(x, y), v))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn flip(self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    pub fn turn_cw(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    pub fn turn_ccw(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn go_dir(self, dir: Dir) -> Self {
        let Coord { x, y } = self;

        let (x, y) = match dir {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
        };

        Coord::new(x, y)
    }

    pub fn neighbours(self) -> impl Iterator<Item = Self> {
        self.neighbours_with_dir().map(|(c, _)| c)
    }

    pub fn neighbours_with_dir(self) -> impl Iterator<Item = (Self, Dir)> {
        let Coord { x, y } = self;
        [
            (x.wrapping_sub(1), y, Dir::Left),
            (x.wrapping_add(1), y, Dir::Right),
            (x, y.wrapping_sub(1), Dir::Up),
            (x, y.wrapping_add(1), Dir::Down),
        ]
        .into_iter()
        .map(|(x, y, d)| (Self::new(x, y), d))
    }
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Coord::new(x, y)
    }
}

#[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let g = Grid::new([1, 2, 3, 4, 5, 6, 7, 8, 9].into(), 3);

        dbg!(&g);
        dbg!(g[0][2]);
        dbg!(g[1][1]);
        dbg!(g[2][2]);
        dbg!(g[1][3]);
    }
}
