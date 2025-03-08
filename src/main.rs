use ecs::Entity;
use ecs::coordinator::Coordinator;    // TODO: remove coordinatro from this path
use ecs::component::ComponentManager; // TODO: remove component from this path
use ecs::system::System;              // TODO: remove system from this path

use raylib::prelude::*;

use std::collections::HashSet;
use std::any::TypeId;

pub struct Screen {
    width: i32,
    height: i32
}

struct TempContainer {
    font_size: i32,
    y_min: f64,
    y_max: f64,
    wheel_speed: f32,

    ball_pos: (i32, i32),
    cnt_x: f64,
    cnt_y: f64,
    wheel_v: Vector2,
}

pub struct Render {
    entities: HashSet<Entity>,
    component_types: HashSet<TypeId>,

    pub rl: RaylibHandle,
    pub raylib_thread: RaylibThread,

    screen: Screen,
    temp_c: TempContainer,
}

impl Render {
    pub fn new() -> Render {
        let (width, height) = (640, 480);
        let (mut rl, raylib_thread) = raylib::init()
            .size(width, height)
            .title("RenderSystem1")
            .build();

        rl.set_target_fps(5);

        let screen = Screen {
            width,
            height,
        };

        let font_size = 20;
        let y_min = 12.0;
        let y_max = screen.height as f64 - 12.0 - font_size as f64;
        let wheel_speed = 4f32;

        let ball_pos = (screen.width / 2, screen.height / 2);
        let wheel_v = Vector2 { x: (screen.width / 4) as f32, y: (screen.height / 4) as f32 };

        let temp_c = TempContainer {
            font_size,
            y_min,
            y_max,
            wheel_speed,

            ball_pos,
            cnt_x: 0f64,
            cnt_y: 0f64,
            wheel_v,
        };


        let render = Render {
            entities: HashSet::new(),
            component_types: HashSet::new(),

            rl,
            raylib_thread,
            screen,

            temp_c,
        };

        render
    }
}

impl System for Render {
    fn add(&mut self, e: Entity) {
        self.entities.insert(e);
    }
    fn remove(&mut self, e: Entity) {
        // TODO: not implemented
    }

    fn get_component_types(&self) -> &HashSet<TypeId> {
        &self.component_types
    }

    fn apply(&mut self, cm: &mut ComponentManager) {
        println!("Apply for Render");
        for e in self.entities.iter() {
            println!(" e: {}", e);
        }

        {

            let wheel_move_v = self.rl.get_mouse_wheel_move_v();
            (self.temp_c.wheel_v.x, self.temp_c.wheel_v.y) = (self.temp_c.wheel_v.x + wheel_move_v.x, self.temp_c.wheel_v.y + wheel_move_v.y);

            if self.rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                if !(self.temp_c.ball_pos.0 >= (self.screen.width)) {
                    self.temp_c.ball_pos.0 += 2;
                }
            }
            if self.rl.is_key_down(KeyboardKey::KEY_LEFT) {
                if !(self.temp_c.ball_pos.0 <= 0) {
                    self.temp_c.ball_pos.0 -= 2;
                }
            }
            if self.rl.is_key_down(KeyboardKey::KEY_UP) {
                if !(self.temp_c.ball_pos.1 <= 0) {
                    self.temp_c.ball_pos.1 -= 2;
                }
            }
            if self.rl.is_key_down(KeyboardKey::KEY_DOWN) {
                if !(self.temp_c.ball_pos.1 >= self.screen.height) {
                    self.temp_c.ball_pos.1 += 2;
                }
            }

            let x = (self.screen.width - 50) as f64 / 2f64 + ((self.screen.width as f64 / 2f64)-30f64)*(((3.14*self.temp_c.cnt_x/360f64) as f64).sin());
            let y = (self.temp_c.y_min + (self.temp_c.y_max - self.temp_c.y_min)*((2f64*3.14f64 * self.temp_c.cnt_y / 360.0).cos())).abs();
            let mouse_pos = self.rl.get_mouse_position();
            let mut d = self.rl.begin_drawing(&self.raylib_thread);

            d.draw_rectangle_v(self.temp_c.wheel_v, Vector2 { x: self.temp_c.wheel_v.x + self.temp_c.wheel_speed * 10f32, y: self.temp_c.wheel_v.y + self.temp_c.wheel_speed * 10f32}, Color::YELLOWGREEN);
            d.draw_circle(self.temp_c.ball_pos.0, self.temp_c.ball_pos.1, 10f32, Color::MAROON);
            d.clear_background(Color::WHITE);
            d.draw_text("Hello", x.round() as i32, y.round() as i32, self.temp_c.font_size, Color::BLACK);

            d.draw_circle_v(mouse_pos, 20f32, Color::BLUE);

            self.temp_c.cnt_x += 2.0 * 1.7;
            self.temp_c.cnt_y += 4.0 * 1.0;
        }
    }
}

fn main() {
    let mut c = Coordinator::new();
    let e1 = c.get_entity();

    c.register_component::<i32>();
    c.register_component::<f32>();
    c.add_component(e1, 1i32);
    c.add_component(e1, 2f32);

    let render_sys = Render::new();
    c.register_system(render_sys);

    // TODO: point of focus
    //       we have to check "windows_should_close()" and have possibility to
    //       exit loop below
    loop {
        c.kick_all_systems();
    }
}
