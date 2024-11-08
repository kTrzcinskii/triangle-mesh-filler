use std::ops::{AddAssign, Mul};

use nalgebra::{Matrix3, Vector3};

use crate::control_points::ControlPoints;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    before_rotation: PData,
    after_rotation: PData,
    u: f32,
    v: f32,
}

impl Point {
    // BINOMIAL[i] = Bin(3,i)
    const N: usize = 3;
    const BINOMIAL: [f32; 4] = [1.0, 3.0, 3.0, 1.0];
    const BINOMIAL_1: [f32; 3] = [1.0, 2.0, 1.0];

    pub const ZERO: Self = Self {
        before_rotation: PData::ZERO,
        after_rotation: PData::ZERO,
        u: 0.0,
        v: 0.0,
    };

    pub fn from_control_points(u: f32, v: f32, control_points: &ControlPoints) -> Self {
        let mut p = Vector3::<f32>::new(0.0, 0.0, 0.0);
        for i in 0..(Self::N + 1) {
            for j in 0..(Self::N + 1) {
                p += control_points.at(i, j) * Self::calculate_B(i, u) * Self::calculate_B(j, v);
            }
        }

        let mut pu = Vector3::<f32>::new(0.0, 0.0, 0.0);
        for i in 0..Self::N {
            for j in 0..(Self::N + 1) {
                pu += (control_points.at(i + 1, j) - control_points.at(i, j))
                    * Self::calculate_B_1(i, u)
                    * Self::calculate_B(j, v);
            }
        }
        pu *= Self::N as f32;
        pu = pu.normalize();

        let mut pv = Vector3::<f32>::new(0.0, 0.0, 0.0);
        for i in 0..(Self::N + 1) {
            for j in 0..Self::N {
                pv += (control_points.at(i, j + 1) - control_points.at(i, j))
                    * Self::calculate_B(i, u)
                    * Self::calculate_B_1(j, v);
            }
        }
        pv *= Self::N as f32;
        pv = pv.normalize();

        let n = pu.cross(&pv);
        let n = n.normalize();

        let before_rotation = PData::new(p, pu, pv, n);
        let after_rotation = before_rotation;
        Self {
            before_rotation,
            after_rotation,
            u,
            v,
        }
    }

    #[allow(non_snake_case)]
    fn calculate_B(i: usize, u: f32) -> f32 {
        Self::BINOMIAL[i] * u.powi(i as i32) * (1.0 - u).powi((Self::N - i) as i32)
    }

    #[allow(non_snake_case)]
    fn calculate_B_1(i: usize, u: f32) -> f32 {
        Self::BINOMIAL_1[i] * u.powi(i as i32) * (1.0 - u).powi((Self::N - 1 - i) as i32)
    }

    pub fn before_rotation(&self) -> &PData {
        &self.before_rotation
    }

    pub fn after_rotation(&self) -> &PData {
        &self.after_rotation
    }

    pub fn u(&self) -> f32 {
        self.u
    }

    pub fn v(&self) -> f32 {
        self.v
    }

    pub fn apply_rotation(&mut self, rotation: &Matrix3<f32>) {
        let p: Vector3<f32> = rotation * self.before_rotation().p;
        let pu: Vector3<f32> = rotation * self.before_rotation().pu;
        let pv: Vector3<f32> = rotation * self.before_rotation().pv;
        let n: Vector3<f32> = rotation * self.before_rotation().n;
        self.after_rotation = PData::new(p, pu, pv, n);
        self.after_rotation.normalize_all();
    }

    pub fn normalize_all(&mut self) {
        self.before_rotation.normalize_all();
        self.after_rotation.normalize_all();
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        Point {
            after_rotation: self.after_rotation * rhs,
            before_rotation: self.before_rotation * rhs,
            u: self.u * rhs,
            v: self.v * rhs,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.after_rotation += rhs.after_rotation;
        self.before_rotation += rhs.before_rotation;
        self.u += rhs.u;
        self.v += rhs.v;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PData {
    /// Point
    p: Vector3<f32>,
    /// Tangent vector u
    pu: Vector3<f32>,
    /// Tangent vector v
    pv: Vector3<f32>,
    /// Normal vector
    n: Vector3<f32>,
}

impl PData {
    pub const ZERO: Self = Self {
        p: Vector3::new(0.0, 0.0, 0.0),
        pu: Vector3::new(0.0, 0.0, 0.0),
        pv: Vector3::new(0.0, 0.0, 0.0),
        n: Vector3::new(0.0, 0.0, 0.0),
    };

    pub fn new(p: Vector3<f32>, pu: Vector3<f32>, pv: Vector3<f32>, n: Vector3<f32>) -> Self {
        Self { p, pu, pv, n }
    }

    pub fn p(&self) -> Vector3<f32> {
        self.p
    }

    pub fn pu(&self) -> Vector3<f32> {
        self.pu
    }

    pub fn pv(&self) -> Vector3<f32> {
        self.pv
    }

    pub fn n(&self) -> Vector3<f32> {
        self.n
    }

    pub fn normalize_all(&mut self) {
        self.pu = self.pu.normalize();
        self.pv = self.pv.normalize();
        self.n = self.n.normalize();
    }
}

impl AddAssign<PData> for PData {
    fn add_assign(&mut self, rhs: PData) {
        self.p += rhs.p;
        self.pu += rhs.pu;
        self.pv += rhs.pv;
        self.n += rhs.n;
    }
}

impl Mul<f32> for PData {
    type Output = PData;

    fn mul(self, rhs: f32) -> Self::Output {
        PData {
            p: self.p * rhs,
            pu: self.pu * rhs,
            pv: self.pv * rhs,
            n: self.n * rhs,
        }
    }
}

pub struct Points2DArr {
    data: Vec<Point>,
    rows: usize,
    cols: usize,
}

impl Points2DArr {
    pub fn new(height: usize, width: usize) -> Self {
        let vec = vec![Point::ZERO; width * height];
        Self {
            data: vec,
            rows: height,
            cols: width,
        }
    }

    pub fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.cols + column
    }

    pub fn at(&self, row: usize, column: usize) -> &Point {
        &self.data[self.get_index(row, column)]
    }

    pub fn at_pos(&self, pos: PosIn2DArr) -> &Point {
        self.at(pos.row, pos.col)
    }

    pub fn set_at(&mut self, row: usize, column: usize, point: Point) {
        let id = self.get_index(row, column);
        self.data[id] = point;
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

#[derive(Clone, Copy)]
pub struct PosIn2DArr {
    pub row: usize,
    pub col: usize,
}
