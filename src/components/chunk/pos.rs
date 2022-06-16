use std::{
    cmp::Ordering,
    ops::{Add, Sub},
};

use bevy::prelude::*;

#[derive(Component, Eq, Debug, Clone, Copy)]
pub struct PosComponent {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl PosComponent {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn mul_scalar(&self, val: i64) -> Self {
        PosComponent::new(self.x * val, self.y * val, self.z * val)
    }

    pub fn get_east_neighbor(&self) -> PosComponent {
        PosComponent::new(self.x + 1, self.y, self.z)
    }
    pub fn get_west_neighbor(&self) -> PosComponent {
        PosComponent::new(self.x - 1, self.y, self.z)
    }
    pub fn get_up_neighbor(&self) -> PosComponent {
        PosComponent::new(self.x, self.y + 1, self.z)
    }
    pub fn get_down_neighbor(&self) -> PosComponent {
        PosComponent::new(self.x, self.y - 1, self.z)
    }
    pub fn get_north_neighbor(&self) -> PosComponent {
        PosComponent::new(self.x, self.y, self.z + 1)
    }
    pub fn get_south_neighbor(&self) -> PosComponent {
        PosComponent::new(self.x, self.y, self.z - 1)
    }

    pub fn iter_around(&self, radius: usize) -> PosComponentAroundIterator {
        PosComponentAroundIterator::new(self.clone(), radius)
    }
    pub fn iter_neighbors(&self, include_self: bool) -> PosComponentIterNeighbors {
        PosComponentIterNeighbors::new(self.clone(), include_self)
    }
}

impl Ord for PosComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x > other.x {
            return Ordering::Greater;
        }
        if self.x < other.x {
            return Ordering::Less;
        }
        if self.y > other.y {
            return Ordering::Greater;
        }
        if self.y < other.y {
            return Ordering::Less;
        }
        if self.z > other.z {
            return Ordering::Greater;
        }
        if self.z < other.z {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

impl Add for PosComponent {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for PosComponent {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl PartialOrd for PosComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PosComponent {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[derive(Clone, Copy)]
pub struct PosComponentIterNeighbors {
    x: i64,
    y: i64,
    z: i64,
    pos: PosComponent,
    include_self: bool,
}

impl PosComponentIterNeighbors {
    pub fn new(pos: PosComponent, include_self: bool) -> Self {
        Self {
            x: -1,
            y: -1,
            z: -1,
            pos,
            include_self,
        }
    }
}

impl Iterator for PosComponentIterNeighbors {
    type Item = PosComponent;

    fn next(&mut self) -> Option<PosComponent> {
        if self.z > 1 {
            return None;
        }
        let result = PosComponent::new(
            self.pos.x + self.x,
            self.pos.y + self.y,
            self.pos.z + self.z,
        );

        self.x += 1;
        if self.x > 1 {
            self.x = -1;
            self.y += 1;
            if self.y > 1 {
                self.y = -1;
                self.z += 1;
            }
        } else if !self.include_self && self.x == 0 && self.y == 0 && self.z == 0 {
            self.x += 1;
        }

        return Some(result);
    }
}

#[derive(Clone, Copy)]
pub struct PosComponentAroundIterator {
    start: PosComponent,
    current: PosComponent,
    current_radius: i64,
    radius: i64,
}

impl PosComponentAroundIterator {
    pub fn new(start: PosComponent, radius: usize) -> Self {
        Self {
            radius: radius as i64,
            start,
            current_radius: 0,
            current: PosComponent::new(0, -(radius as i64), 0),
        }
    }
}

impl Iterator for PosComponentAroundIterator {
    type Item = PosComponent;

    fn next(&mut self) -> Option<PosComponent> {
        let r = self.current_radius;
        if self.radius == r {
            return None;
        }

        let y_r = self.radius - r + 1;

        let new_pos = match self.current {
            p if p.y < y_r => p.get_up_neighbor(),
            mut p if p.z == r && p.x == -r => {
                p.y = -y_r + 1;
                self.current_radius += 1;
                p.get_north_neighbor()
            }

            mut p if p.x < r && p.z == r => {
                p.y = -y_r;
                p.get_east_neighbor()
            }

            mut p if p.z > -r && p.x == r => {
                p.y = -y_r;
                p.get_south_neighbor()
            }

            mut p if p.x > -r && p.z == -r => {
                p.y = -y_r;
                p.get_west_neighbor()
            }

            mut p if p.z < r && p.x == -r => {
                p.y = -y_r;
                p.get_north_neighbor()
            }

            _ => {
                panic!("unreachable");
            }
        };

        // new_pos.y = 0;

        self.current = new_pos;

        return Some(new_pos + self.start);
    }
}
