use macroquad::prelude::*;
mod utils;

use utils::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut points: Vec<Point> = Vec::new();
    let mut original_lines = Vec::new();
    let mut lines_to_draw = original_lines.clone();
    let mut pressed_enter_key = false;
    let mut line_is_drawn = false;
    let mut animation_started = false;
    let animation_duration = 3.0;
    loop {

        println!("{:?}", get_time());
        if is_key_pressed(KeyCode::Enter) {
            pressed_enter_key = true;
            original_lines = draw_lines_points(&points);
        }

        if animation_started {
            let first_one = start_animation(lines_to_draw.clone());
            let second_lines = draw_lines_points(&first_one);
        }

        if line_is_drawn && pressed_enter_key {
            // println!("huuuuuuuna");
            // let first_one = start_animation(lines_to_draw.clone());
            // let second_lines = draw_lines_points(&first_one);
            animation_started = true;
        }

        if !pressed_enter_key {
            for Point(x, y) in &points {
                draw_circle_lines(*x, *y, 3.0, 1.0, WHITE);
            }
        } else if !line_is_drawn {
            for (p1, p2) in &original_lines {
                draw_line(p1.0, p1.1, p2.0, p2.1, 1.0, WHITE);
            }
            line_is_drawn = true;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x_mouse_position, y_mouse_position) = mouse_position();
            points.push(Point(x_mouse_position, y_mouse_position));
            draw_circle_lines(x_mouse_position, y_mouse_position, 3.0, 0.05, WHITE);
        }

        next_frame().await;
    }
}
