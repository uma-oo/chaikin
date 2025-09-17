use macroquad::prelude::*;
mod utils;

use utils::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    // setup for the project 

    // points + lines + defaults 
    let mut points = Vec::new();
    let mut lines = Vec::new();
    let mut iteration = 0;
    let max_iterations = 7;
    let mut last_update_time = get_time();
    let mut pressed_enter_key = false;
    let mut show_error_message = false;

    loop {
        clear_background(BLACK);
       // exiting the window if escaped !!
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
        
        // clearing the window if the delete is pressed !! 
        if is_key_pressed(KeyCode::Delete) {
            points = Vec::new();
            lines = Vec::new();
            iteration = 0;
            pressed_enter_key = false;
            last_update_time = get_time();
            show_error_message = false;
        }

        if points.len() > 1 && show_error_message {
            show_error_message = false;
        }

        // for the error message display 
        if (is_key_pressed(KeyCode::Enter) && points.len() <= 1) || show_error_message {
            draw_text(
                "please try to add more than one point!",
                20.0,
                screen_height() / 2.0,
                30.0,
                YELLOW
            );
            show_error_message = true;
        }

        for Point(x, y) in &points {
            draw_circle_lines(*x, *y, 3.0, 1.0, WHITE);
        }

        if (is_key_pressed(KeyCode::Enter) && points.len() > 1) || iteration == max_iterations {
            pressed_enter_key = true;
            lines = draw_lines_points(points.clone());
            iteration = 0;
            last_update_time = get_time();
        }
        
        // we only start if the lines we have is => 2
        if iteration < max_iterations && lines.len() >= 2 {
            let now = get_time();
            if now - last_update_time > 2.0 {
                let points_vec = chaikin_iteration(lines.clone());
                lines = draw_lines_points(points_vec);
                iteration += 1;
                last_update_time = now;
            }
        }

        for (p1, p2) in &lines {
            draw_line(p1.0, p1.1, p2.0, p2.1, 2.0, PINK);
        }

        if is_mouse_button_pressed(MouseButton::Left) && !pressed_enter_key {
            let (x_mouse_position, y_mouse_position) = mouse_position();
            points.push(Point(x_mouse_position, y_mouse_position));
            draw_circle_lines(x_mouse_position, y_mouse_position, 3.0, 0.05, WHITE);
        }

        next_frame().await;
    }
}
