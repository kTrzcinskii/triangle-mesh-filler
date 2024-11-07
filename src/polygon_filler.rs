use egui::{Color32, Pos2};
use nalgebra::{Vector2, Vector3};

use crate::{
    drawer::Drawer,
    light_source::LightSource,
    point::{PData, Points2DArr, PosIn2DArr},
};

pub struct PolygonFiller<'p, 'd, 'ep, 'l> {
    all_points: &'p Points2DArr,
    drawer: &'d Drawer<'ep>,
    light_source: &'l LightSource,
    base_color: Color32,
    kd: f32,
    ks: f32,
    m: u8,
}

impl<'p, 'd, 'ep, 'l> PolygonFiller<'p, 'd, 'ep, 'l> {
    pub fn new(
        all_points: &'p Points2DArr,
        drawer: &'d Drawer<'ep>,
        light_source: &'l LightSource,
        base_color: Color32,
        kd: f32,
        ks: f32,
        m: u8,
    ) -> Self {
        Self {
            all_points,
            drawer,
            light_source,
            base_color,
            kd,
            ks,
            m,
        }
    }

    pub fn fill_polygon(&mut self, polygon: &[PosIn2DArr]) {
        let mut aet = AET::new();
        let sorted_indicies = self.get_vertices_sorted_indicies(polygon);
        let y_min = self.get_point_at_index(polygon, &sorted_indicies, 0).y;
        let y_max = self
            .get_point_at_index(polygon, &sorted_indicies, sorted_indicies.len() - 1)
            .y;
        let mut y = y_min as i32;

        while y <= y_max as i32 {
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
                self.check_point(&mut aet, p_index, p, previous_index, previous_p);

                // Check next
                let next_index = match p_index == polygon.len() - 1 {
                    true => 0,
                    false => p_index + 1,
                };
                let next_pos = polygon[next_index];
                let next_p = self.all_points.at_pos(next_pos).after_rotation().p();
                self.check_point(&mut aet, p_index, p, next_index, next_p);
            }
            aet.sort_by_x();
            let f = |x: i32, y: i32| {
                self.paint_pixel(x, y, polygon);
            };
            aet.fill_line(y, f);
            aet.update_x();
            y += 1;
        }
    }

    fn paint_pixel(&self, x: i32, y: i32, polygon: &[PosIn2DArr]) {
        let pos = Pos2 {
            x: x as f32,
            y: y as f32,
        };
        let bar_coords = self.get_barycentric_coords(polygon, Vector2::<f32>::new(pos.x, pos.y));
        let p = self.point_from_barycentric_coords(polygon, bar_coords);
        let color = self.color_in_point(p);
        self.drawer.paint_pixel(pos, color);
    }

    fn check_point(
        &mut self,
        aet: &mut AET,
        current_index: usize,
        current_p: Vector3<f32>,
        other_index: usize,
        other_p: Vector3<f32>,
    ) {
        if other_p.y >= current_p.y {
            aet.add_edge(current_p, other_p, current_index, other_index);
        } else {
            aet.remove_edge(current_index, other_index);
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

    /// Returns vector where i-th element is barycentric coord of i-th point from polygon
    fn get_barycentric_coords(&self, polygon: &[PosIn2DArr], p: Vector2<f32>) -> Vec<f32> {
        // TODO: for now only works for triangles, maybe should be generalized for every polygon?
        let v0 = self.all_points.at_pos(polygon[1]).after_rotation().p().xy()
            - self.all_points.at_pos(polygon[0]).after_rotation().p().xy();
        let v1 = self.all_points.at_pos(polygon[2]).after_rotation().p().xy()
            - self.all_points.at_pos(polygon[0]).after_rotation().p().xy();
        let v2 = p - self.all_points.at_pos(polygon[0]).after_rotation().p().xy();
        let d00 = v0.dot(&v0);
        let d01 = v0.dot(&v1);
        let d11 = v1.dot(&v1);
        let d20 = v2.dot(&v0);
        let d21 = v2.dot(&v1);
        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;
        vec![u, v, w]
    }

    fn point_from_barycentric_coords(&self, polygon: &[PosIn2DArr], bars: Vec<f32>) -> PData {
        let mut p = PData::ZERO;
        for i in 0..polygon.len() {
            p += *self.all_points.at_pos(polygon[i]).after_rotation() * bars[i];
        }
        p.normalize_all();
        p
    }

    fn color_in_point(&self, point: PData) -> Color32 {
        let r = self.base_color.r() as f32 / u8::MAX as f32;
        let light_r = self.light_source.color().r() as f32 / u8::MAX as f32;
        let g = self.base_color.g() as f32 / u8::MAX as f32;
        let light_g = self.light_source.color().g() as f32 / u8::MAX as f32;
        let b = self.base_color.b() as f32 / u8::MAX as f32;
        let light_b = self.light_source.color().b() as f32 / u8::MAX as f32;
        let new_r = self.calculate_color_component(point, r, light_r);
        let new_g = self.calculate_color_component(point, g, light_g);
        let new_b = self.calculate_color_component(point, b, light_b);
        Color32::from_rgb(new_r, new_g, new_b)
    }

    fn calculate_color_component(
        &self,
        point: PData,
        base_color_component: f32,
        light_color_component: f32,
    ) -> u8 {
        let l = (self.light_source.position() - point.p()).normalize();
        let mut cos_n_l = point.n().dot(&l);
        if cos_n_l < 0.0 {
            cos_n_l = 0.0;
        }
        let lhs = self.kd * light_color_component * base_color_component * cos_n_l;
        let v = Vector3::<f32>::new(0.0, 0.0, 1.0);
        let r = 2.0 * cos_n_l * point.n() - l;
        let mut cos_m_v_r = v.dot(&r).powi(self.m as i32);
        if cos_m_v_r < 0.0 {
            cos_m_v_r = 0.0;
        }
        let rhs = self.ks * light_color_component * base_color_component * cos_m_v_r;
        let mut sum = lhs + rhs;
        if sum > 1.0 {
            sum = 1.0;
        }
        (sum * 255.0) as u8
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

    fn fill_line<F>(&mut self, y: i32, pixel_callback: F)
    where
        F: Fn(i32, i32),
    {
        for same_y in self.same_y.iter() {
            for x in (same_y.x_start as i32)..(same_y.x_end as i32 + 1) {
                pixel_callback(x, y - 1);
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
                pixel_callback(x, y);
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
