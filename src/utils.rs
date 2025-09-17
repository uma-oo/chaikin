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
pub fn Chaikin_iteration(points: Vec<(Point, Point)>) -> Vec<Point> {
    let mut lines_iteration = Vec::new();
    for (p1, p2) in points {
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
    }
    lines_iteration
}



// it does not work OMG we wan't do this this way
pub fn create_animation(steps: usize, mut original_lines: Vec<(Point, Point)>) {
    let mut current_lines = original_lines;
    let mut current_points = Vec::new();
    let mut i = 1;

    while i <= steps {
        println!("step :{:?}", i);

        thread::sleep(Duration::from_millis(1000));

        current_points = Chaikin_iteration(current_lines.to_vec());
       
        current_lines = draw_lines_points(current_points);
        original_lines = current_lines.clone();
        i += 1;
    }
}
