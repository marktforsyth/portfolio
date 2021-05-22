use macroquad::*;

mod user_interface;

#[derive(Copy, Clone)]
pub struct StrokeSettings {
    color: Color,
    size: f32,
}

#[macroquad::main("Drawing App")]
async fn main() {
    let mut brush_settings = { StrokeSettings {
        color: RED,
        size: 10.0,
    } };

    let mut strokes: Vec<(StrokeSettings, Vec<[f32; 2]>)> = vec![(brush_settings, vec![])];
    let mut undone_strokes: Vec<(StrokeSettings, Vec<[f32; 2]>)> = vec![(brush_settings, vec![])];

    let mut currently_drawing = false;
    let mut current_stroke_index = 0;
    let mut first_stroke = true;

    loop {
        clear_background(WHITE);

        user_interface::ui_logic(&mut brush_settings);

        for (_s, stroke) in strokes.iter().enumerate() {
            for (p, point) in stroke.1.iter().enumerate() {
                draw_circle(point[0], point[1], stroke.0.size / 2.0, stroke.0.color);

                if p + 1 < stroke.1.len() {
                    draw_line(point[0], point[1], stroke.1[p + 1][0], stroke.1[p + 1][1], stroke.0.size, stroke.0.color);
                }
            }
            // BUG: when the mouse leaves the screen and re-enters in another place, it draws a connecting line between enter and end
            // should I end the stroke on-leave-screen and then start another strok on-enter-screen (but only if just_left = true)?
        }

        let (mouse_x, mouse_y) = mouse_position();

        if is_mouse_button_down(MouseButton::Left) {
            if !currently_drawing {
                strokes.append(&mut vec![(brush_settings, vec![])]);

                if !first_stroke {
                    current_stroke_index += 1;
                } else {
                    first_stroke = false;
                }

                currently_drawing = true;
            }
            
            if mouse_x > user_interface::SIDEBAR_WIDTH //+ brush_settings.size / 2.0
            && mouse_x < screen_width() - user_interface::BORDER_WIDTH
            && mouse_y > user_interface::BORDER_WIDTH
            && mouse_y < screen_height() - user_interface::BORDER_WIDTH {
                strokes[current_stroke_index].1.append(&mut vec![[mouse_x, mouse_y]]);
            } else {
                currently_drawing = false;
            }

            if mouse_x > 0.0 && mouse_x < user_interface::SIDEBAR_WIDTH && mouse_y > screen_height() - user_interface::SIDEBAR_WIDTH * 2.0 - 35.0 && mouse_y < screen_height() - user_interface::SIDEBAR_WIDTH * 2.0 + 5.0 {
                // Undo
                if strokes.len() > 0 {
                    /* This part is full of errors currently, so it is commented out */
                    //println!("{}", strokes[strokes.len() - 1].1[0][0]);
                    //undone_strokes.extend(&mut vec![(brush_settings, strokes[strokes.len() - 1])]);
                    //undone_strokes.append(&mut vec![(brush_settings, strokes[strokes.len() - 1]).1]);
                }
            } else if mouse_x > 0.0 && mouse_x < user_interface::SIDEBAR_WIDTH && mouse_y > screen_height() - user_interface::SIDEBAR_WIDTH * 2.0 - 35.0 && mouse_y < screen_height() - user_interface::SIDEBAR_WIDTH * 2.0 + 45.0 {
                // Redo
                if undone_strokes.len() > 0 {

                }
            }

            if mouse_x > 0.0 && mouse_x < user_interface::SIDEBAR_WIDTH && mouse_y > screen_height() - user_interface::SIDEBAR_WIDTH - 15.0 && mouse_y < screen_height() - 25.0 {
                strokes = vec![(brush_settings, vec![])];
                current_stroke_index = 0;
                first_stroke = true;
            }

            // eventually TODO: use a *subsurf*-like system to make the lines turn into smooth curves, to make the drawing less jagged.
        } else {
            currently_drawing = false;
        }

        user_interface::user_interface(&mut brush_settings);

        next_frame().await
    }
}