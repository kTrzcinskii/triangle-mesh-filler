use crate::{
    control_points::ControlPoints,
    point::{Point, Points2DArr, PosIn2DArr},
    rotations::Rotations,
    triangle::Triangle,
    triangle_mesh_filler::ControlsState,
};

pub struct Mesh {
    triangles: Vec<Triangle>,
    points: Points2DArr,
}

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
                let mut point = Point::from_control_points(u, v, control_points);
                point.apply_rotation(&rotation);
                points.set_at(i, j, point);
            }
        }
        points
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
