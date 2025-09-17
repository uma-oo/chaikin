use macroquad::prelude::*;
use std::{ thread, time::Duration };
// so we need to draw lines
#[derive(Clone, Copy, Debug)]
pub struct Point(pub f32, pub f32);

pub fn draw_lines_points(points: Vec<Point>) -> Vec<(Point, Point)> {
    let mut lines = Vec::new();
    for i in 0..points.len() {
        if i + 1 < points.len() {
            lines.push((points[i].clone(), points[i + 1].clone()));
        }
    }
    lines
}

// each times we need the current points
pub fn chaikin_iteration(points: Vec<(Point, Point)>) -> Vec<Point> {
    let mut lines_iteration = Vec::new();

    let (start, _) = points[0];
    let (_, end) = points[points.len() - 1];
    lines_iteration.push(start);
    let mut i = 0;
    for (p1, p2) in &points {
        if i == 0 {
            let (start, _) = points[0];
            lines_iteration.push(start);
        }

        let point_q = Point(
            ((3 as f32) / (4 as f32)) * p1.0 + ((1 as f32) / (4 as f32)) * p2.0,
            ((3 as f32) / (4 as f32)) * p1.1 + ((1 as f32) / (4 as f32)) * p2.1
        );
        let point_r = Point(
            ((1 as f32) / (4 as f32)) * p1.0 + ((3 as f32) / (4 as f32)) * p2.0,
            ((1 as f32) / (4 as f32)) * p1.1 + ((3 as f32) / (4 as f32)) * p2.1
        );
        lines_iteration.push(point_q);
        lines_iteration.push(point_r);

        if i == points.len() - 1 {
            let (_, end) = points[points.len() - 1];
            lines_iteration.push(end);
        }
        i += 1;
    }

    lines_iteration
}
