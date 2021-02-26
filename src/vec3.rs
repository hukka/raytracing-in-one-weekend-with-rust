use std::convert::Into;
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[test]
fn scalar_multiplication() {
    let a = Vec3 {x: 1.0, y: 0.0, z: 0.0};
    assert_eq!(a * 2 as u8, Vec3 {x: 2.0, y: 0.0, z: 0.0});
    assert_eq!(a * 2.0, Vec3 {x: 2.0, y: 0.0, z: 0.0});
}

#[test]
fn cross_product() {
    let x = Vec3 {x: 1.0, y: 0.0, z: 0.0};
    let y = Vec3 {x: 0.0, y: 1.0, z: 0.0};
    let z = Vec3 {x: 0.0, y: 0.0, z: 1.0};

    // Right hand rules
    assert_eq!(x.cross(y), z);
    assert_eq!(y.cross(x), -z);
    assert_eq!(y.cross(z), x);
    assert_eq!(z.cross(y), -x);
    assert_eq!(z.cross(x), y);
    assert_eq!(x.cross(z), -y);

    // Cross product of vectors on the same line is zero
    assert_eq!(x.cross(x), Vec3 {x: 0.0, y: 0.0, z: 0.0});
    assert_eq!(x.cross(-x), Vec3 {x: 0.0, y: 0.0, z: 0.0});
}

#[test]
fn dot_product() {
    let a = Vec3 {x: 1.0, y: 0.0, z: 0.0};
    assert_eq!(a.dot(Vec3 {x: 1.0, y: 0.0, z: 0.0}), 1.0);
    assert_eq!(a.dot(Vec3 {x: 1.0, y: 1.0, z: 1.0}), 1.0);
    assert_eq!(a.dot(Vec3 {x: 0.0, y: 1.0, z: 0.0}), 0.0);

    let b = Vec3 {x: 2.0, y: 0.0, z: 0.0};
    assert_eq!(b.dot(Vec3 {x: 10.0, y: 0.0, z: 0.0}), 20.0);
    assert_eq!(b.dot(Vec3 {x: 20.0, y: 20.0, z: 20.0}), 40.0);
    assert_eq!(b.dot(Vec3 {x: 0.0, y: 1.0, z: 0.0}), 0.0);
}

#[test]
fn length() {
    assert_eq!((Vec3 {x: 10.0, y: 0.0, z: 0.0}).length(), 10.0);
    assert_eq!((Vec3 {x: 3.0, y: 4.0, z: 0.0}).length(), 5.0);
}

// Floats can be NaN, and NaN != NaN, so they cannot fulfill Eq but only PartialEq,
// and nothing prevents one of the components from being NaN.
impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x+other.x, y: self.y+other.y, z: self.z+other.z }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, other: Vec3) -> Vec3 {
        self + (-other)
    }
}

impl<T> ops::Mul<T> for Vec3
where
T: Into<f32>,
T: Copy,
{
    type Output = Self;
    fn mul(self, other: T) -> Vec3 {
        Vec3 { x: self.x*other.into(), y: self.y*other.into(), z: self.z*other.into() }
    }
}

impl<T> ops::Div<T> for Vec3
where
T: Into<f32>,
T: Copy,
{
    type Output = Self;
    fn div(self, other: T) -> Vec3 {
        Vec3 { x: self.x/other.into(), y: self.y/other.into(), z: self.z/other.into() }
    }
}

impl Vec3 {
    pub fn new<T>(x: T, y: T, z: T) -> Vec3 where T: Into<f32> {
        Vec3 { x: x.into(), y: y.into(), z: z.into() }
    }

    pub fn length(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y*other.z - other.y*self.z,
            y: self.z*other.x - other.z*self.x,
            z: self.x*other.y - other.x*self.y,
        }
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

