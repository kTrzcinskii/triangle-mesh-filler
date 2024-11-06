use nalgebra::Vector3;

use crate::{
    drawer::Drawer,
    mesh::{Points2DArr, PosIn2DArr},
};

pub struct PolygonFiller<'p, 'd, 'ep> {
    all_points: &'p Points2DArr,
    drawer: &'d Drawer<'ep>,
    aet: AET,
}

impl<'p, 'd, 'ep> PolygonFiller<'p, 'd, 'ep> {
    pub fn new(all_points: &'p Points2DArr, drawer: &'d Drawer<'ep>) -> Self {
        Self {
            all_points,
            drawer,
            aet: AET::new(),
        }
    }

    pub fn fill_polygon(&mut self, polygon: &[PosIn2DArr]) {
        self.aet = AET::new();
        let sorted_indicies = self.get_vertices_sorted_indicies(polygon);
        let y_min = self.get_point_at_index(polygon, &sorted_indicies, 0).y;
        let y_max = self
            .get_point_at_index(polygon, &sorted_indicies, sorted_indicies.len() - 1)
            .y;
        let mut y = y_min as i32;

        while y < y_max as i32 {
            for i in 0..sorted_indicies.len() {
                let p_index = sorted_indicies[i];
                let p = self.get_point_at_index(polygon, &sorted_indicies, i);

                // Sorted_indicies are sorted by y so we can break like this
                if p.y as i32 >= y {
                    break;
                }
                if p.y as i32 != y - 1 {
                    continue;
                }
                // At this point we know p was on previous scan line

                // Check previous
                let previous_index = match p_index == 0 {
                    true => polygon.len() - 1,
                    false => p_index - 1,
                };
                let previous_pos = polygon[previous_index];
                let previous_p = self.all_points.at_pos(previous_pos).after_rotation().p();
                self.check_point(p_index, p, previous_index, previous_p);

                // Check next
                let next_index = match p_index == polygon.len() - 1 {
                    true => 0,
                    false => p_index + 1,
                };
                let next_pos = polygon[next_index];
                let next_p = self.all_points.at_pos(next_pos).after_rotation().p();
                self.check_point(p_index, p, next_index, next_p);
            }
            self.aet.sort_by_x();
            self.aet.fill_line(self.drawer, y);
            self.aet.update_x();
            y += 1;
        }
    }

    fn check_point(
        &mut self,
        current_index: usize,
        current_p: Vector3<f32>,
        other_index: usize,
        other_p: Vector3<f32>,
    ) {
        if other_p.y >= current_p.y {
            self.aet
                .add_edge(current_p, other_p, current_index, other_index);
        } else {
            self.aet.remove_edge(current_index, other_index);
        }
    }

    fn get_vertices_sorted_indicies(&self, polygon: &[PosIn2DArr]) -> Vec<usize> {
        let mut indicies: Vec<_> = (0..polygon.len()).collect();
        indicies.sort_by(|&i, &j| {
            let a = polygon[i];
            let b = polygon[j];
            let ya = self.all_points.at_pos(a).after_rotation().p().y;
            let yb = self.all_points.at_pos(b).after_rotation().p().y;
            ya.partial_cmp(&yb)
                .expect("All floats in polygon should be comparable")
        });
        indicies
    }

    fn get_point_at_index(
        &self,
        polygon: &[PosIn2DArr],
        sorted_indicies: &[usize],
        index: usize,
    ) -> Vector3<f32> {
        let pos = polygon[sorted_indicies[index]];
        self.all_points.at_pos(pos).after_rotation().p()
    }
}

#[allow(clippy::upper_case_acronyms)]
struct AET {
    data: Vec<AETData>,
    same_y: Vec<SameY>,
}

impl AET {
    fn new() -> Self {
        AET {
            data: vec![],
            same_y: vec![],
        }
    }

    fn add_edge(&mut self, start: Vector3<f32>, end: Vector3<f32>, start_id: usize, end_id: usize) {
        let (start_y, end_y) = (start.y as i32, end.y as i32);
        if start_y == end_y {
            self.same_y.push(SameY {
                x_start: start.x.min(end.x),
                x_end: end.x.max(start.x),
            });
            return;
        }
        let (start, end) = match start_y < end_y {
            true => (start, end),
            false => (end, start),
        };
        let x = start.x;
        let x_diff = (end.x - start.x) / (end_y - start_y) as f32;
        let start_index = start_id.min(end_id);
        let end_index = end_id.max(start_id);
        self.data.push(AETData {
            x,
            x_diff,
            start: start_index,
            end: end_index,
        });
    }

    fn remove_edge(&mut self, start_id: usize, end_id: usize) {
        let start = start_id.min(end_id);
        let end = end_id.max(start_id);

        self.data.retain(|p| p.start != start || p.end != end);
    }

    fn update_x(&mut self) {
        for d in self.data.iter_mut() {
            d.update_x();
        }
    }

    fn sort_by_x(&mut self) {
        self.data.sort_by(|a, b| {
            a.x.partial_cmp(&b.x)
                .expect("All floats in polygon should be comparable")
        });
    }

    fn fill_line(&mut self, drawer: &Drawer<'_>, y: i32) {
        for same_y in self.same_y.iter() {
            for x in (same_y.x_start as i32)..(same_y.x_end as i32 + 1) {
                let pos = egui::Pos2 {
                    x: x as f32,
                    y: (y - 1) as f32,
                };
                drawer.paint_pixel(pos, egui::Color32::YELLOW);
            }
        }
        self.same_y.clear();
        if self.data.is_empty() {
            return;
        }
        for i in 0..(self.data.len() - 1) {
            let next = i + 1;
            let start = self.data[i].x;
            let end = self.data[next].x;
            for x in (start as i32)..(end as i32 + 1) {
                let pos = egui::Pos2 {
                    x: x as f32,
                    y: y as f32,
                };
                drawer.paint_pixel(pos, egui::Color32::YELLOW);
            }
        }
    }
}

struct AETData {
    x: f32,
    x_diff: f32,
    // Edge info (e.g. edge 1-2 => start=1, end=2), start < end
    start: usize,
    end: usize,
}

impl AETData {
    fn update_x(&mut self) {
        self.x += self.x_diff
    }
}

struct SameY {
    x_start: f32,
    x_end: f32,
}
