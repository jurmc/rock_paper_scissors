use raylib::prelude::*;
use raylib::consts;
use raylib::core::color;

fn main() {
    let (w, h) = (640, 480);
    let mut ball_pos = (w / 2, h / 2);
    let font_size = 20;

    let (mut rl, thread) = raylib::init()
        .size(w, h)
        .title("Windows: Hello, World")
        .build();

    rl.set_target_fps(60);

    let (mut cnt_x, mut cnt_y) = (0.0, 0.0);

    let x_min = 12.0;
    let x_max = w as f64 - 12.0 - (12 * font_size) as f64;

    println!("x_min: {}, x_max: {}", x_min, x_max);

    let y_min = 12.0;
    let y_max = h as f64 - 12.0 - font_size as f64;

    let mut wheel_v = Vector2 { x: (w / 4) as f32, y: (h / 4) as f32 };
    let wheel_speed = 4f32;

    while !rl.window_should_close() {

        let wheel_move_v = rl.get_mouse_wheel_move_v();
        println!("wheel_move_v: {:?}", wheel_move_v);
        (wheel_v.x, wheel_v.y) = (wheel_v.x + wheel_move_v.x, wheel_v.y + wheel_move_v.y);

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            if !(ball_pos.0 >= (w)) {
                ball_pos.0 += 2;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            if !(ball_pos.0 <= 0) {
                ball_pos.0 -= 2;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            if !(ball_pos.1 <= 0) {
                ball_pos.1 -= 2;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            if !(ball_pos.1 >= h) {
                ball_pos.1 += 2;
            }
        }

        let x = (w - 50) as f64 / 2f64 + ((w as f64 / 2f64)-30f64)*(((3.14*cnt_x/360f64) as f64).sin());
        let y = (y_min + (y_max - y_min)*((2f64*3.14f64 * cnt_y / 360.0).cos())).abs();
        let mouse_pos = rl.get_mouse_position();
        let mut d = rl.begin_drawing(&thread);

        d.draw_rectangle_v(wheel_v, Vector2 { x: wheel_v.x + wheel_speed * 10f32, y: wheel_v.y + wheel_speed * 10f32}, Color::YELLOWGREEN);
        d.draw_circle(ball_pos.0, ball_pos.1, 10f32, Color::MAROON);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello", x.round() as i32, y.round() as i32, font_size, Color::BLACK);

        d.draw_circle_v(mouse_pos, 20f32, Color::BLUE);

        cnt_x += 2.0 * 1.7;
        cnt_y += 4.0 * 1.0;
    }
}
