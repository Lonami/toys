use std::io::{stdin, Read};
use std::ops::Index;
use std::iter::Iterator;

#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty,
    Asteroid
}

#[derive(Debug)]
struct CellMap {
    cells: Vec<Cell>,
    width: usize,
    height: usize
}

struct CellMapIter<'a> {
    map: &'a CellMap,
    index: usize
}

// I was never good with angles
fn angle((x, y): (i32, i32)) -> f32 {
    if y == 0 {
        // right (1/2 pi) or left (3/2 pi)
        (if x > 0 { 1f32 } else { 3f32 }) * std::f32::consts::FRAC_PI_2
    } else if x == 0 {
        // down (2/2 pi) or up (0/2 pi)
        if y > 0 { std::f32::consts::PI } else { 0f32 }
    } else {
        // random stuff until it works
        let angle = ((x as f32) / (y as f32)).atan();
        if y > 0 {
            std::f32::consts::PI - angle
        } else if x > 0 {
            -angle
        } else {
            2f32 * std::f32::consts::PI - angle
        }
    }
}

impl Cell {
    fn from_char(character: u8) -> Self {
        match character {
            b'.' => Cell::Empty,
            b'#' => Cell::Asteroid,
            _ => panic!(format!("don't know how to represent {}", character))
        }
    }
}

impl CellMap {
    fn in_bounds(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }

    fn trace(&self, (mut x, mut y): (i32, i32), (dx, dy): (i32, i32)) -> Option<(i32, i32)> {
        x += dx;
        y += dy;
        while self.in_bounds((x, y)) {
            match self[(x, y)] {
                Cell::Empty => {
                    x += dx;
                    y += dy;
                },
                Cell::Asteroid => {
                    return Some((x, y));
                }
            }
        }

        None
    }

    fn count_visible(&self, (sx, sy): (i32, i32)) -> usize {
        self.iter().map(|((tx, ty), target)| {
            match target {
                Cell::Empty => 0,
                Cell::Asteroid => {
                    let mut dx = tx - sx;
                    let mut dy = ty - sy;
                    if dx == 0 && dy == 0 {
                        0 // self, ignore (not visible)
                    } else {
                        let div = gcd(dx, dy);
                        dx /= div;
                        dy /= div;

                        // find resulting trace
                        if let Some((rx, ry)) = self.trace((sx, sy), (dx, dy)) {
                            (tx == rx && ty == ry) as usize // true of result = target
                        } else {
                            0
                        }
                    }
                }
            }
        }).sum::<usize>()
    }

    fn find_nth_visible(&self, (sx, sy): (i32, i32), n: usize) -> (i32, i32) {
        let mut deltas: Vec<(i32, i32)> = self.iter().filter_map(|((tx, ty), target)| {
            match target {
                Cell::Empty => None,
                Cell::Asteroid => {
                    let mut dx = tx - sx;
                    let mut dy = ty - sy;
                    if dx == 0 && dy == 0 {
                        None
                    } else {
                        let div = gcd(dx, dy);
                        dx /= div;
                        dy /= div;

                        // find resulting trace
                        if let Some((rx, ry)) = self.trace((sx, sy), (dx, dy)) {
                            if tx == rx && ty == ry {
                                Some((dx, dy))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                }
            }
        }).collect();
        deltas.sort_by(|a, b| angle(*a).partial_cmp(&angle(*b)).expect("cannot compare floats"));
        let (dx, dy) = deltas[n];
        (sx + dx, sy + dy)
    }

    fn iter(&self) -> CellMapIter {
        CellMapIter { map: self, index: 0 }
    }
}

impl Index<(i32, i32)> for CellMap {
    type Output = Cell;

    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        if self.in_bounds((x, y)) {
            &self.cells[y as usize * self.width + x as usize]
        } else {
            &Cell::Empty
        }
    }
}

impl Iterator for CellMapIter<'_> {
    type Item = ((i32, i32), Cell);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.map.cells.len() {
            return None;
        }

        let result = (
            ((self.index % self.map.width) as i32, (self.index / self.map.width) as i32),
            self.map.cells[self.index]
        );

        self.index += 1;
        Some(result)
    }
}

fn read_input() -> CellMap {
    let mut buffer = Vec::new();
    stdin()
        .lock()
        .read_to_end(&mut buffer)
        .expect("error while reading input file");

    let mut n = 0;
    let mut width = None;
    let mut cells = Vec::with_capacity(buffer.len());

    for character in buffer.iter() {
        if *character == b'\n' {
            if let Some(w) = width {
                if w != n {
                    panic!("malformed input map is not rectangular");
                }
            } else {
                width = Some(n);
            }

            n = 0;
            continue;
        }

        cells.push(Cell::from_char(*character));
        n += 1;
    }

    let width = width.expect("empty input");
    let height = cells.len() / width;
    CellMap { cells, width, height }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }

    if a == 0 || b == 0 {
        return a.max(b).max(1);
    }

    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
}

fn main() {
    let map = read_input();
    let (pos, visible) = map.iter().filter_map(|(pos, source)| {
        match source {
            Cell::Empty => None,
            Cell::Asteroid => Some((pos, map.count_visible(pos)))
        }
    }).max_by_key(|&(_, visible)| visible).expect("empty input");
    println!("{}", visible);

    // because we have to find the 200th and visible count > 200
    // we can do this in just one turn. simply find the 200th visible
    // one (sorted iteration by angle), no need to blast anything
    let (x, y) = map.find_nth_visible(pos, 199);
    println!("{}", x * 100 + y);
}
