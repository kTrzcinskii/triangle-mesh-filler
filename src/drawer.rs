use egui::{Color32, Pos2};
use nalgebra::Vector3;

use crate::{
    control_points::{ControlPoints, CONTROL_POINT_COLS, CONTROL_POINT_ROWS},
    mesh::{Mesh, Points2DArr},
    rotations::Rotations,
    triangle::Triangle,
    triangle_mesh_filler::ControlsState,
};

pub struct Drawer;

impl Drawer {
    fn point_to_screen(point: &Vector3<f32>, screen_center: &Pos2) -> Pos2 {
        let x = screen_center.x + point.x;
        let y = screen_center.y - point.y;
        Pos2 { x, y }
    }

    fn pos_to_screen(pos: &Pos2, screen_center: &Pos2) -> Pos2 {
        let x = screen_center.x + pos.x;
        let y = screen_center.y - pos.y;
        Pos2 { x, y }
    }

    // We don't apply rotations to them, so we need to rotate them while drawing
    pub fn draw_control_points(
        painter: &egui::Painter,
        screen_center: &Pos2,
        control_points: &ControlPoints,
        controls_state: &ControlsState,
    ) {
        const WIDTH: f32 = 3.0;
        let x_rotation = Rotations::create_x_rotation_matrix(controls_state.beta());
        let z_rotation = Rotations::create_z_rotation_matrix(controls_state.alfa());
        let rotation = z_rotation * x_rotation;
        for i in 0..CONTROL_POINT_ROWS {
            for j in 0..CONTROL_POINT_COLS {
                let control_point = rotation * control_points.at(i, j);
                let position = Self::point_to_screen(&control_point, screen_center);
                painter.circle(
                    position,
                    WIDTH,
                    Color32::BLACK,
                    egui::Stroke {
                        color: Color32::BLACK,
                        width: WIDTH,
                    },
                );
            }
        }
    }

    pub fn draw_mesh(painter: &egui::Painter, screen_center: &Pos2, mesh: &Mesh) {
        for triangle in mesh.triangles() {
            Self::draw_triangle(painter, screen_center, triangle, mesh.points());
        }
    }

    fn draw_triangle(
        painter: &egui::Painter,
        screen_center: &Pos2,
        triangle: &Triangle,
        points: &Points2DArr,
    ) {
        let vertices = triangle.vertices();
        for id in 0..3 {
            let next_id = (id + 1) % 3;
            let start_ids = vertices[id];
            let end_ids = vertices[next_id];
            let start = points.at(start_ids.row, start_ids.col);
            let end = points.at(end_ids.row, end_ids.col);
            let start_position = Self::point_to_screen(&start.after_rotation().p(), screen_center);
            let end_position = Self::point_to_screen(&end.after_rotation().p(), screen_center);
            painter.line_segment(
                [start_position, end_position],
                egui::Stroke {
                    width: 1.5,
                    color: Color32::BLUE,
                },
            );
        }
    }

    pub fn paint_pixel(
        painter: &egui::Painter,
        screen_center: &Pos2,
        position: Pos2,
        color: Color32,
    ) {
        let pos = Self::pos_to_screen(&position, screen_center);
        let rect = egui::Rect::from_min_size(pos, egui::Vec2::new(1.0, 1.0));
        painter.rect_filled(rect, 0.0, color);
    }
}
