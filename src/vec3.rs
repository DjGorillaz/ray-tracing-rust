use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

type Point3 = Vec3;
type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{x, y, z}
    }

    pub fn len(&self) -> f64 {
        Self::len_squared(self).sqrt()
    }

    fn len_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

#[test]
fn add_test() {
    let v1 = Vec3::new(1.5, 2.0, 3.0);
    let v2 = Vec3::new(-1.5, 3.0, 100.0);
    assert_eq!(v1 + v2, Vec3::new(0.0, 5.0, 103.0));
}

#[test]
fn sub_test() {
    let v1 = Vec3::new(1.5, 2.0, 3.0);
    let v2 = Vec3::new(-1.5, 3.0, 100.0);
    assert_eq!(v1 - v2, Vec3::new(3.0, -1.0, -97.0));
}

#[test]
fn mul_test() {
    let v1 = Vec3::new(1.5, 2.0, 3.0);
    assert_eq!(v1 * 2.0, Vec3::new(3.0, 4.0, 6.0));
}

#[test]
fn dib_test() {
    let v1 = Vec3::new(3.0, 2.0, 1.0);
    assert_eq!(v1 / 2.0, Vec3::new(1.5, 1.0, 0.5));
}

#[test]
fn len_squared_test() {
    let v1 = Vec3::new(3.0, 2.0, 1.0);
    assert_eq!(v1.len_squared(), 14.0);
}

#[test]
fn len_test() {
    let v1 = Vec3::new(3.0, 4.0, 0.0);
    assert_eq!(v1.len(), 5.0);
}
