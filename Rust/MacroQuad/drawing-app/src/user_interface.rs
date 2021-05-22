use macroquad::*;

use crate::StrokeSettings;

pub struct Button<'a> {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    on_click: &'a str,
    new_color: Color,
    new_size: f32,
}

pub const SIDEBAR_WIDTH: f32 = 65.0;
pub const BORDER_WIDTH: f32 = 15.0;

const COLOR_BUTTON_SIZE: f32 = 25.0;
const COLOR_BUTTON_BORDER_WIDTH: f32 = 4.0;

const COLOR_OPTIONS: [Color; 5] = [BLACK, BLUE, GREEN, RED, ORANGE];

pub fn user_interface(brush_settings: &mut StrokeSettings) {
    draw_rectangle(0.0, 0.0, SIDEBAR_WIDTH, screen_height(), LIGHTGRAY);
    /*draw_rectangle(screen_width() - BORDER_WIDTH, 0.0, BORDER_WIDTH, screen_height(), LIGHTGRAY);
    draw_rectangle(0.0, 0.0, screen_width(), BORDER_WIDTH, LIGHTGRAY);
    draw_rectangle(0.0, screen_height() - BORDER_WIDTH, screen_width(), BORDER_WIDTH, LIGHTGRAY);*/

    /*if DrawContext::UI.button(32.0, 55.0, "Red") {
        brush_settings.color = BLUE;
    }*/

    for (c, color) in COLOR_OPTIONS.iter().enumerate() {
        if *color == brush_settings.color {
            draw_rectangle(
                0.0,
                SIDEBAR_WIDTH * (c as f32 + 1.0) - SIDEBAR_WIDTH / 2.0 - 15.0 - COLOR_BUTTON_BORDER_WIDTH / 2.0,
                SIDEBAR_WIDTH,
                SIDEBAR_WIDTH,
                WHITE
            );
        } else {
            draw_rectangle(
                20.0 - COLOR_BUTTON_BORDER_WIDTH / 2.0,
                SIDEBAR_WIDTH * (c as f32 + 1.0) - COLOR_BUTTON_BORDER_WIDTH / 2.0 - 30.0,
                COLOR_BUTTON_SIZE + COLOR_BUTTON_BORDER_WIDTH,
                COLOR_BUTTON_SIZE + COLOR_BUTTON_BORDER_WIDTH,
                WHITE
            );
        }
        draw_rectangle(20.0, SIDEBAR_WIDTH * (c as f32 + 1.0) - 30.0, COLOR_BUTTON_SIZE, COLOR_BUTTON_SIZE, *color);
    }

    let (mouse_x, mouse_y) = mouse_position();
    let obscuring_undo_circle: Color;
    let obscuring_redo_circle: Color;

    if is_mouse_button_down(MouseButton::Left) && mouse_x > 0.0 && mouse_x < SIDEBAR_WIDTH && mouse_y > screen_height() - SIDEBAR_WIDTH * 2.0 - 35.0 && mouse_y < screen_height() - SIDEBAR_WIDTH * 2.0 + 5.0 {
        draw_rectangle(10.0, screen_height() - SIDEBAR_WIDTH * 2.0 - 35.0, SIDEBAR_WIDTH - 20.0, 40.0, WHITE);
        obscuring_undo_circle = WHITE;
        obscuring_redo_circle = LIGHTGRAY;
    } else if is_mouse_button_down(MouseButton::Left) && mouse_x > 0.0 && mouse_x < SIDEBAR_WIDTH && mouse_y > screen_height() - SIDEBAR_WIDTH * 2.0 - 35.0 && mouse_y < screen_height() - SIDEBAR_WIDTH * 2.0 + 45.0 {
        draw_rectangle(10.0, screen_height() - SIDEBAR_WIDTH * 2.0 + 5.0, SIDEBAR_WIDTH - 20.0, 40.0, WHITE);
        obscuring_redo_circle = WHITE;
        obscuring_undo_circle = LIGHTGRAY;
    } else {
        obscuring_undo_circle = LIGHTGRAY;
        obscuring_redo_circle = LIGHTGRAY;
    }

    // Undo Button
    draw_circle_lines(SIDEBAR_WIDTH / 2.0, screen_height() - SIDEBAR_WIDTH * 2.0 - 15.0, 10.0, 2.0, BLACK);
    draw_circle(SIDEBAR_WIDTH * 0.3, screen_height() - SIDEBAR_WIDTH * 2.0 - 20.0, 8.0, obscuring_undo_circle);
    draw_circle(SIDEBAR_WIDTH * 0.4, screen_height() - SIDEBAR_WIDTH * 2.0 - 23.0, 4.0, BLACK);

    // Redo Button
    draw_circle_lines(SIDEBAR_WIDTH / 2.0, screen_height() - SIDEBAR_WIDTH * 2.0 + 25.0, 10.0, 2.0, BLACK);
    draw_circle(SIDEBAR_WIDTH * 0.7, screen_height() - SIDEBAR_WIDTH * 2.0 + 20.0, 8.0, obscuring_redo_circle);
    draw_circle(SIDEBAR_WIDTH * 0.6, screen_height() - SIDEBAR_WIDTH * 2.0 + 17.0, 4.0, BLACK);

    if is_mouse_button_down(MouseButton::Left) && mouse_x > 0.0 && mouse_x < SIDEBAR_WIDTH && mouse_y > SIDEBAR_WIDTH * 6.0 - 40.0 && mouse_y < SIDEBAR_WIDTH * 6.5 - 32.5 {
        draw_rectangle(10.0, SIDEBAR_WIDTH * 6.0 - 40.0, SIDEBAR_WIDTH - 20.0, SIDEBAR_WIDTH / 2.0 + 7.5, WHITE);
    } else if is_mouse_button_down(MouseButton::Left) && mouse_x > 0.0 && mouse_x < SIDEBAR_WIDTH && mouse_y > SIDEBAR_WIDTH * 6.5 - 30.0 && mouse_y < SIDEBAR_WIDTH * 7.0 - 22.5 {
        draw_rectangle(10.0, SIDEBAR_WIDTH * 6.5 - 30.0, SIDEBAR_WIDTH - 20.0, SIDEBAR_WIDTH / 2.0 + 7.5, WHITE);
    }

    // Plus Sign
    draw_line(SIDEBAR_WIDTH / 2.0, SIDEBAR_WIDTH * 6.0 - 32.5, SIDEBAR_WIDTH / 2.0, SIDEBAR_WIDTH * 6.0 - 7.5, 2.0, BLACK);
    draw_line(20.0, SIDEBAR_WIDTH * 6.0 - 20.0, SIDEBAR_WIDTH - 20.0, SIDEBAR_WIDTH * 6.0 - 20.0, 2.0, BLACK);

    // Minus Sign
    draw_line(20.0, SIDEBAR_WIDTH * 6.5 - 10.0, SIDEBAR_WIDTH - 20.0, SIDEBAR_WIDTH * 6.5 - 10.0, 2.0, BLACK);

    if is_mouse_button_down(MouseButton::Left) && mouse_x > 0.0 && mouse_x < SIDEBAR_WIDTH && mouse_y > screen_height() - SIDEBAR_WIDTH - 15.0 && mouse_y < screen_height() - 25.0 {
        draw_rectangle(5.0, screen_height() - SIDEBAR_WIDTH - 15.0, SIDEBAR_WIDTH - 10.0, SIDEBAR_WIDTH - 10.0, WHITE);
    }

    draw_line(20.0, screen_height() - SIDEBAR_WIDTH, 45.0, screen_height() - 40.0, 3.0, BLACK);
    draw_line(45.0, screen_height() - SIDEBAR_WIDTH, 20.0, screen_height() - 40.0, 3.0, BLACK);

    /*brush_settings.color = BLACK;
    brush_settings.size = 5.0;*/
}

pub fn ui_logic(brush_settings: &mut StrokeSettings) {
    let mut button_list: Vec<Button> = vec![];

    for (c, color) in COLOR_OPTIONS.iter().enumerate() {
        button_list.append(&mut vec![Button {
            x: 0.0,
            y: SIDEBAR_WIDTH * (c as f32 + 1.0) - SIDEBAR_WIDTH / 2.0 - 15.0 - COLOR_BUTTON_BORDER_WIDTH / 2.0,
            width: SIDEBAR_WIDTH,
            height: SIDEBAR_WIDTH,
            on_click: "color",
            new_color: *color,
            new_size: 0.0,
        }]);
    }

    button_list.append(&mut vec![
        Button {
            x: 10.0,
            y: SIDEBAR_WIDTH * 6.0 - 40.0,
            width: SIDEBAR_WIDTH - 20.0,
            height: SIDEBAR_WIDTH / 2.0 + 7.5,
            on_click: "size",
            new_color: BLACK,
            new_size: brush_settings.size + 2.0,
        },
        Button {
            x: 10.0,
            y: SIDEBAR_WIDTH * 6.5 - 30.0,
            width: SIDEBAR_WIDTH - 20.0,
            height: SIDEBAR_WIDTH / 2.0 + 7.5,
            on_click: "size",
            new_color: BLACK,
            new_size: brush_settings.size - 2.0,
        }
    ]);

    let (mouse_x, mouse_y) = mouse_position();
    for button in &button_list {
        if is_mouse_button_down(MouseButton::Left) && mouse_x > button.x && mouse_x < button.x + button.width && mouse_y > button.y && mouse_y < button.y + button.height {
            execute_button(brush_settings, button.on_click, button.new_color, button.new_size);
        }
    }
}

fn execute_button(brush_settings: &mut StrokeSettings, on_click: &str, new_color: Color, new_size: f32) {
    if on_click == "color" {
        brush_settings.color = new_color;
    } else if on_click == "size" {
        if brush_settings.size < 4.0 && new_size < 2.0 {
            return;
        }

        brush_settings.size = new_size;
    } else {
        println!("Error in execute_button! That's not a proper on_click!");
    }
}