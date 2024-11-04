use egui::{Color32, Pos2};
use nalgebra::Vector3;

use crate::{
    mesh::{Mesh, Points2DArr},
    triangle::Triangle,
};

pub struct Drawer;

impl Drawer {
    fn point_to_screen(point: &Vector3<f32>, screen_center: &Pos2) -> Pos2 {
        let x = screen_center.x + point.x;
        let y = screen_center.y - point.y;
        Pos2 { x, y }
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
}
