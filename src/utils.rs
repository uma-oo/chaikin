use macroquad::prelude::*;

// so we need to draw lines
#[derive(Clone, Copy)]
pub struct Point(pub f32, pub f32);

pub fn draw_lines_points(points: &Vec<Point>) -> Vec<(Point, Point)> {
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
    for i in 0..points.len() {
        if i + 1 < points.len() {
            let (p1, p2) = points[i];
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
    }
    lines_iteration
}

pub fn create_animation(
    steps: usize,
    animation_duration: f32,
    original_lines: Vec<(Point, Point)>
) {

 
    // get_time 

    let last_time = get_time()
    for _ in 0..7 {

        i
        let lines = Chaikin_iteration(original_lines);


    }
}
