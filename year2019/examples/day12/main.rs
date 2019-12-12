use std::io::{stdin, BufRead};
use std::str::FromStr;
use std::num::ParseIntError;
use std::cmp::Ordering;
use std::fmt;

const STEP_COUNT: usize = 1000;

#[derive(PartialEq, Debug)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32
}

#[derive(PartialEq, Debug)]
struct Body {
    pos: Vec3,
    vel: Vec3
}

#[derive(Debug)]
struct System {
    bodies: Vec<Body>,
    step: usize
}

impl Vec3 {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0
        }
    }

    fn add(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={: >3}, y={: >3}, z={: >3}>", self.x, self.y, self.z)
    }
}

impl FromStr for Vec3 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.trim_matches(|p| p == '<' || p == '>' ).split(',');

        let x = it.next().expect("empty input")
            .split('=').skip(1).next().expect("malformed x coord").parse::<i32>()?;

        let y = it.next().expect("need 2 more coords")
            .split('=').skip(1).next().expect("malformed y coord").parse::<i32>()?;

        let z = it.next().expect("need 1 more coord")
            .split('=').skip(1).next().expect("malformed z coord").parse::<i32>()?;

        Ok(Self { x, y, z })
    }
}

impl Body {
    fn new(pos: Vec3) -> Self {
        Self {
            pos,
            vel: Vec3::new()
        }
    }

    fn pull_with(&mut self, other: &mut Body) {
        fn pull_value(p0: i32, p1: i32, v0: &mut i32, v1: &mut i32) {
            let (d0, d1) = match p0.cmp(&p1) {
                Ordering::Less => (1, -1),
                Ordering::Equal => return,
                Ordering::Greater => (-1, 1)
            };

            *v0 += d0;
            *v1 += d1;
        }

        pull_value(self.pos.x, other.pos.x, &mut self.vel.x, &mut other.vel.x);
        pull_value(self.pos.y, other.pos.y, &mut self.vel.y, &mut other.vel.y);
        pull_value(self.pos.z, other.pos.z, &mut self.vel.z, &mut other.vel.z);
    }

    fn apply_vel(&mut self) {
        self.pos.add(&self.vel);
    }

    fn potential_energy(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos={}, vel={}", self.pos, self.vel)
    }
}

impl System {
    fn step(&mut self) {
        assert!(self.bodies.len() > 1); // Cannot simulate less than 2 bodies

        // Pull pairs together to update velocities
        for i in 1..self.bodies.len() {
            let (left, right) = self.bodies.split_at_mut(i);
            let b0 = &mut left[i - 1];
            for mut b1 in right {
                b0.pull_with(&mut b1);
            }
        }

        // Apply velocities
        for body in self.bodies.iter_mut() {
            body.apply_vel();
        }

        // Another step taken
        self.step += 1;
    }

    fn total_energy(&self) -> i32 {
        self.bodies.iter().map(|body| body.total_energy()).sum()
    }
}

impl fmt::Display for System {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "After {} step{}:\n", self.step, if self.step == 1 { "" } else { "s" })?;
        for body in self.bodies.iter() {
            body.fmt(&mut f)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn read_input() -> System {
    let bodies = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("failed to read input"))
        .map(|line| Body::new(Vec3::from_str(&line).expect("malformed input")))
        .collect();

    System { bodies, step: 0 }
}

fn main() {
    let mut system = read_input();
    for _ in 0..STEP_COUNT {
        system.step();
    }

    println!("{}", system.total_energy());
}
