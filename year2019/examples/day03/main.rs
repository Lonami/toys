use std::io::{stdin, BufRead};

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: i16
}

#[derive(PartialEq, Debug)]
enum Orientation {
    Horizontal,
    Vertical
}

// walked = distance to reach this
#[derive(Debug)]
struct HLine {
    x: i16,
    y: i16,
    len: i16,
    walked: u32
}

#[derive(Debug)]
struct VLine {
    x: i16,
    y: i16,
    len: i16,
    walked: u32
}

#[derive(Debug)]
struct Intersection {
    x: i16,
    y: i16,
    walked_h: u32,
    walked_v: u32
}

const TOO_MANY_ACTIONS: usize = 100;
const CLOSE_DISTANCE_PERCENT: f32 = 0.2;

impl Move {
    fn from_str(string: &str) -> Self {
        Self {
            amount: string[1..].parse::<i16>().expect("malformed input"),
            direction: match string.as_bytes()[0] {
                b'U' => Direction::Up,
                b'R' => Direction::Right,
                b'D' => Direction::Down,
                b'L' => Direction::Left,
                _ => unreachable!()
            }
        }
    }

    fn orientation(&self) -> Orientation {
        match self.direction {
            Direction::Up | Direction::Down => Orientation::Vertical,
            Direction::Right | Direction::Left => Orientation::Horizontal
        }
    }
}

impl Intersection {
    fn manhattan_from_origin(&self) -> i16 {
        self.x.abs() + self.y.abs()
    }

    fn walked_total(&self) -> u32 {
        self.walked_h + self.walked_v
    }
}

fn read_inputs() -> Vec<Vec<Move>> {
    stdin()
        .lock()
        .lines()
        .map(|line| line.expect("error while reading input file"))
        .map(|line| line.split(',').map(Move::from_str).collect())
        .collect()
}

fn path_to_lines(path: &Vec<Move>) -> (Vec<HLine>, Vec<VLine>) {
    let mut x = 0i16;
    let mut y = 0i16;
    let mut walked = 0u32;
    let mut hlines = vec![];
    let mut vlines = vec![];
    for mov in path {
        match mov.direction {
            Direction::Up => {
                vlines.push(VLine { x, y, walked, len: mov.amount });
                y += mov.amount;
            },
            Direction::Right => {
                hlines.push(HLine { x, y, walked, len: mov.amount });
                x += mov.amount;
            },
            Direction::Down => {
                vlines.push(VLine { x, y, walked, len: -mov.amount });
                y -= mov.amount;
            },
            Direction::Left => {
                hlines.push(HLine { x, y, walked, len: -mov.amount });
                x -= mov.amount;
            }
        };
        walked += mov.amount as u32;
    }

    (hlines, vlines)
}

fn intersect(horizontal: &HLine, vertical: &VLine) -> Option<Intersection> {
    let (x0, x1) = if horizontal.len > 0 {
        (horizontal.x, horizontal.x + horizontal.len)
    } else {
        (horizontal.x + horizontal.len, horizontal.x)
    };

    if x0 <= vertical.x && vertical.x <= x1 {
        let (y0, y1) = if vertical.len > 0 {
            (vertical.y, vertical.y + vertical.len)
        } else {
            (vertical.y + vertical.len, vertical.y)
        };

        if y0 <= horizontal.y && horizontal.y <= y1 {
            return Some(Intersection {
                x: vertical.x,
                y: horizontal.y,
                walked_h: horizontal.walked + (vertical.x - horizontal.x).abs() as u32,
                walked_v: vertical.walked + (horizontal.y - vertical.y).abs() as u32
            });
        }
    }

    None
}

fn main() {
    let mut inputs = read_inputs();
    let second = inputs.pop().expect("invalid empty input");
    let first = inputs.pop().expect("input is missing second line");

    let (mut first_h, mut first_v) = path_to_lines(&first);
    let (mut second_h, mut second_v) = path_to_lines(&second);

    // If we have far too many actions, keep only the closest percentage.
    //
    // Note that this if is completely irrelevant to the solution itself
    // and as such, removing it won't have any effect on the solution.
    if first.len() > TOO_MANY_ACTIONS {
        // First determine what the farthest point is.
        let mut max_x = 0;
        let mut max_y = 0;
        for h in first_h.iter().chain(second_h.iter()) {
            max_x = max_x.max(h.x.abs());
            max_y = max_y.max(h.y.abs());
        }
        for v in first_v.iter().chain(second_v.iter()) {
            max_x = max_x.max(v.x.abs());
            max_y = max_y.max(v.y.abs());
        }

        let max_dist_sq = (
            (CLOSE_DISTANCE_PERCENT * max_x as f32).powi(2)
            + (CLOSE_DISTANCE_PERCENT * max_y as f32).powi(2)
        ) as i32;

        // Then filter to keep Close Enough™
        first_h = first_h.into_iter().filter(|h| (h.x as i32).pow(2) + (h.y as i32).pow(2) < max_dist_sq).collect();
        first_v = first_v.into_iter().filter(|v| (v.x as i32).pow(2) + (v.y as i32).pow(2) < max_dist_sq).collect();
        second_h = second_h.into_iter().filter(|h| (h.x as i32).pow(2) + (h.y as i32).pow(2) < max_dist_sq).collect();
        second_v = second_v.into_iter().filter(|v| (v.x as i32).pow(2) + (v.y as i32).pow(2) < max_dist_sq).collect();
    }

    let intersections =
        first_h.iter().flat_map(
            |h| second_v.iter().flat_map(
                move |v| intersect(h, v)
            )
        )
        .chain(second_h.iter().flat_map(
            |h| first_v.iter().flat_map(
                move |v| intersect(h, v)
            )
        ));

    // Our real input starts parallel to each other.
    // All example inputs start perpendicular to each other.
    //
    // This is an issue because we only consider perpendicular
    // intersections. We need to skip (0, 0) iff it starts parallel.
    // There's probably more issues or elegant ways to fix it but
    // this works.
    let intersections: Vec<Intersection> = if first[0].orientation() == second[0].orientation() {
        intersections.collect()
    } else {
        intersections.filter(|i| i.x != 0 && i.y != 0).collect()
    };

    assert!(!intersections.is_empty());

    println!("{}", intersections
        .iter()
        .map(|i| i.manhattan_from_origin())
        .min()
        .unwrap());

    println!("{}", intersections
        .iter()
        .map(|i| i.walked_total())
        .min()
        .unwrap());
}
