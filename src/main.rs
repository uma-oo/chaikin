use macroquad::prelude::*;
mod utils;

use utils::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut points = Vec::new();
    let mut lines = Vec::new();
    let mut iteration = 0;
    let max_iterations = 7;
    let mut last_update_time = get_time();
    let mut pressed_enter_key = false;

    loop {
        clear_background(BLACK);
        if !pressed_enter_key {
            println!("enter not pressed yet");
            for Point(x, y) in &points {
                println!("heereee!!!");
                draw_circle_lines(*x, *y, 3.0, 1.0, WHITE);
            }
        }
        if is_key_pressed(KeyCode::Enter) && points.len() > 0 {
            pressed_enter_key = true;
            lines = draw_lines_points(points.clone());
            iteration = 0;
            last_update_time = get_time();
        }

        if iteration < max_iterations {
            let now = get_time();
            if now - last_update_time > 1.0 {
                let points_vec = Chaikin_iteration(lines.clone());
                lines = draw_lines_points(points_vec);
                iteration += 1;
                println!("{}", iteration);
                last_update_time = now;
            }


        } 
        // else if iteration == max_iterations {
        //     iteration = 0;
        //     last_update_time = get_time();
        // }

        for (p1, p2) in &lines {
            draw_line(p1.0, p1.1, p2.0, p2.1, 1.0, WHITE);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("hunaa");
            let (x_mouse_position, y_mouse_position) = mouse_position();
            points.push(Point(x_mouse_position, y_mouse_position));
            draw_circle_lines(x_mouse_position, y_mouse_position, 3.0, 0.05, WHITE);
        }

        next_frame().await;
    }
}

// use macroquad::prelude::*;
// mod utils;

// use utils::*;

// #[macroquad::main("BasicShapes")]
// async fn main() {
//     let mut points: Vec<Point> = Vec::new();
//     let mut original_lines = Vec::new();
//     let mut lines_to_draw = original_lines.clone();
//     let mut pressed_enter_key = false;
//     let mut line_is_drawn = false;
//     // let  animation_started = false;
//     loop {
//         if is_key_pressed(KeyCode::Enter) {
//             pressed_enter_key = true;
//             original_lines = draw_lines_points(points.clone());
//             lines_to_draw = original_lines.clone();
//         }

//         if !pressed_enter_key {
//             for Point(x, y) in &points {
//                 draw_circle_lines(*x, *y, 3.0, 1.0, WHITE);
//             }
//         } else if !line_is_drawn {
//             for (p1, p2) in &original_lines {
//                 draw_line(p1.0, p1.1, p2.0, p2.1, 1.0, WHITE);
//             }
//             line_is_drawn = true;
//         } else {
//             create_animation(7, lines_to_draw.clone());
//             for (p1, p2) in &lines_to_draw {
//                 draw_line(p1.0, p1.1, p2.0, p2.1, 1.0, WHITE);
//             }
//         }

//         if is_mouse_button_pressed(MouseButton::Left) {
//             let (x_mouse_position, y_mouse_position) = mouse_position();
//             points.push(Point(x_mouse_position, y_mouse_position));
//             draw_circle_lines(x_mouse_position, y_mouse_position, 3.0, 0.05, WHITE);
//         }

//         next_frame().await;
//     }
// }
