use nalgebra::Vector3;

use crate::{
    control_points::ControlPoints,
    point::{PData, Point},
    rotations::Rotations,
    triangle::Triangle,
    triangle_mesh_filler::ControlsState,
};

pub struct Mesh {
    triangles: Vec<Triangle>,
    points: Points2DArr,
}

// BINOMIAL[i] = Bin(3,i)
const N: usize = 3;
const BINOMIAL: [f32; 4] = [1.0, 3.0, 3.0, 1.0];

impl Mesh {
    pub fn triangulation(control_points: &ControlPoints, controls_state: &ControlsState) -> Self {
        let points = Self::generate_points(control_points, controls_state);
        let triangles = Self::generate_triangles(&points);
        Self { triangles, points }
    }

    pub fn triangles(&self) -> &[Triangle] {
        &self.triangles
    }

    pub fn points(&self) -> &Points2DArr {
        &self.points
    }

    fn generate_points(
        control_points: &ControlPoints,
        controls_state: &ControlsState,
    ) -> Points2DArr {
        let x_rotation = Rotations::create_x_rotation_matrix(controls_state.beta());
        let z_rotation = Rotations::create_z_rotation_matrix(controls_state.alfa());
        let rotation = x_rotation * z_rotation;
        let points_count = controls_state.triangulation_accuracy();
        let mut points = Points2DArr::new(points_count, points_count);
        for i in 0..points_count {
            let u = i as f32 / (points_count - 1) as f32;
            for j in 0..points_count {
                let v = j as f32 / (points_count - 1) as f32;
                let mut point = Self::generate_point(u, v, control_points);
                point.apply_rotation(&rotation);
                points.set_at(i, j, point);
            }
        }
        points
    }

    fn generate_point(u: f32, v: f32, control_points: &ControlPoints) -> Point {
        let mut p = Vector3::<f32>::new(0.0, 0.0, 0.0);
        for i in 0..(N + 1) {
            for j in 0..(N + 1) {
                p += control_points.at(i, j) * Self::calculate_B(i, u) * Self::calculate_B(j, v);
            }
        }
        let zero = Vector3::<f32>::new(0.0, 0.0, 0.0);
        let before_rotation = PData::new(p, zero, zero, zero);
        let after_rotation = PData::ZERO;
        Point::new(before_rotation, after_rotation, u, v)
    }

    #[allow(non_snake_case)]
    fn calculate_B(i: usize, u: f32) -> f32 {
        BINOMIAL[i] * u.powi(i as i32) * (1.0 - u).powi((N - i) as i32)
    }

    fn generate_triangles(points: &Points2DArr) -> Vec<Triangle> {
        let mut triangles = vec![];
        for i in 0..(points.rows() - 1) {
            for j in 1..points.cols() {
                let upper_triangle = Triangle::new([
                    PosIn2DArr { row: i, col: j },
                    PosIn2DArr { row: i, col: j - 1 },
                    PosIn2DArr {
                        row: i + 1,
                        col: j - 1,
                    },
                ]);
                let bottom_triangle = Triangle::new([
                    PosIn2DArr { row: i, col: j },
                    PosIn2DArr { row: i + 1, col: j },
                    PosIn2DArr {
                        row: i + 1,
                        col: j - 1,
                    },
                ]);
                triangles.push(upper_triangle);
                triangles.push(bottom_triangle);
            }
        }
        triangles
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
