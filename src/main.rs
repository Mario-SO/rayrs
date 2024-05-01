extern crate raylib;
use rand::Rng;
use raylib::prelude::*; // To generate random numbers

const WINDOW_WIDTH: i32 = 400;
const WINDOW_HEIGHT: i32 = 400;
const BOX_WIDTH: i32 = WINDOW_WIDTH - 20;
const BOX_HEIGHT: i32 = WINDOW_HEIGHT - 20;
const TEXT: &str = "@mariodev__";
const TEXT_SIZE: i32 = 20;

fn main() {
    let mut color: Color = Color::LIGHTGRAY;
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("mariodev__")
        .build();

    let mut text_x = BOX_WIDTH as f32 / 2.0;
    let mut text_y = BOX_HEIGHT as f32 / 2.0;
    let mut text_vel_x = 1.0; // Horizontal velocity
    let mut text_vel_y = 1.0; // Vertical velocity

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        // Update text position
        text_x += text_vel_x * 0.05;
        text_y += text_vel_y * 0.05;

        // Check boundaries for text bounce
        let text_width = measure_text(TEXT, TEXT_SIZE);

        if text_x < 10.0 || text_x + text_width as f32 > BOX_WIDTH as f32 {
            text_vel_x *= -1.0;
            change_color_randomly(&mut color);
        }
        if text_y < 10.0 || text_y + TEXT_SIZE as f32 > BOX_HEIGHT as f32 {
            text_vel_y *= -1.0;
            change_color_randomly(&mut color);
        }

        // Draw text and box
        d.draw_text(TEXT, text_x as i32, text_y as i32, TEXT_SIZE, color);

        d.draw_rectangle_lines_ex(
            Rectangle::new(
                5.0,
                5.0,
                (WINDOW_WIDTH - 10) as f32,
                (WINDOW_HEIGHT - 10) as f32,
            ),
            10,
            Color::BLACK,
        );
    }
}

fn change_color_randomly(color: &mut Color) {
    let mut rng = rand::thread_rng();
    *color = Color::new(
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        255,
    );
}
