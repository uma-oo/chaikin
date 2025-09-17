use macroquad::prelude::*;

// so we need to draw lines
#[derive(Clone, Copy, Debug)]
pub struct Point(pub f32, pub f32);



//  this is for the combinaison of the points into lines 
pub fn draw_lines_points(points: Vec<Point>) -> Vec<(Point, Point)> {
    let mut lines = Vec::new();
    for i in 0..points.len() {
        if i + 1 < points.len() {
            lines.push((points[i].clone(), points[i + 1].clone()));
        }
    }
    lines
}

// each times we need the current points (so we'll be passing the lines each times to get the q and r between each line)
pub fn chaikin_iteration(points: Vec<(Point, Point)>) -> Vec<Point> {
    let mut lines_iteration = Vec::new();
    let (start, _) = points[0];
    lines_iteration.push(start);
    for (p1, p2) in &points {
        let point_q = Point(0.75 * p1.0 + 0.25 * p2.0, 0.75 * p1.1 + 0.25 * p2.1);
        let point_r = Point(0.25 * p1.0 + 0.75 * p2.0, 0.25 * p1.1 + 0.75 * p2.1);
        lines_iteration.push(point_q);
        lines_iteration.push(point_r);
    }
    let (_, end) = points[points.len() - 1];
    lines_iteration.push(end);
    lines_iteration
}
