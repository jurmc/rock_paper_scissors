use ecs::Entity;
use ecs::coordinator::Coordinator;    // TODO: remove coordinatro from this path
use ecs::component::ComponentManager; // TODO: remove component from this path
use ecs::ComponentType;
use ecs::system::System;              // TODO: remove system from this path

use raylib::prelude::*;

use std::collections::HashSet;
use std::any::TypeId;
use std::sync::{Mutex, LazyLock};

pub struct RayLibData {
    rl: RaylibHandle,
    raylib_thread: RaylibThread,
    screen: Screen,
}

impl RayLibData {
    pub fn new(width: i32, height: i32) -> RayLibData {
        let (mut rl, raylib_thread) = raylib::init()
            .size(width, height)
            .title("RenderSystem1")
            .build();

        let screen = Screen {
            width,
            height,
        };

        RayLibData {
            rl,
            raylib_thread,
            screen,
        }
    }
}

#[derive(Debug)]
pub struct Coords {
    x: i32,
    y: i32,
}

pub struct MyColor {
    c: Color,
}

struct Velocity {
    vx: f64,
    vy: f64,
}

pub struct Screen {
    width: i32,
    height: i32
}

struct TempContainer {
    ball_pos: (i32, i32),
}

pub struct IntegrateVelocity {
    entities: HashSet<Entity>,
    component_types: HashSet<TypeId>,
}

impl IntegrateVelocity {
    pub fn new() -> IntegrateVelocity {
        IntegrateVelocity {
            entities: HashSet::new(),
            component_types: HashSet::new(),
        }
    }
}

impl System for IntegrateVelocity {
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
        for e in self.entities.iter() {
            if let Some(v) = cm.get::<Velocity>(e) {
                let v = Velocity {vx: v.vx, vy: v.vy};
                if let Some(c) = cm.get::<Coords>(e) {
                    let new_coords = Coords { x: c.x + v.vx.round() as i32, y: c.y + v.vy.round() as i32 };
                    cm.add(*e, new_coords);
                }
            }
        }
    }
}

pub struct Render {
    entities: HashSet<Entity>,
    component_types: HashSet<TypeId>,

    //rl_data: RayLibData,

    temp_c: TempContainer,
}

impl Render {
    pub fn new() -> Render {
        let (width, height) = (640, 480);

        //let mut rl_data = RayLibData::new(width, height);
        //rl_data.rl.set_target_fps(5);

        let screen = Screen {
            width,
            height,
        };

        let ball_pos = (screen.width / 2, screen.height / 2);

        let temp_c = TempContainer {
            ball_pos,
        };


        let render = Render {
            entities: HashSet::new(),
            component_types: HashSet::new(),

            //rl_data,

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
        ////
        ////let mut rl_data = cm.get::<RayLibData>();
        //if rl_data = None {
        //    println!("RayLibData not initialized. Cannot render...");
        //    return;
        //}
        /////

        let mouse_pos;
//{
        let rl_data = cm.get_global::<RayLibData>(2).unwrap();


        if rl_data.rl.window_should_close() {
            panic!("Exitted..."); // TODO: this condition should rather be somehow signalled to the
                                  // outside world...
        }

        // TODO: point of focus: extract below code into separate system and components
        // ONGOING

        let wheel_move_v = rl_data.rl.get_mouse_wheel_move_v();

        if rl_data.rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            if !(self.temp_c.ball_pos.0 >= (rl_data.screen.width)) {
                self.temp_c.ball_pos.0 += 2;
            }
        }
        if rl_data.rl.is_key_down(KeyboardKey::KEY_LEFT) {
            if !(self.temp_c.ball_pos.0 <= 0) {
                self.temp_c.ball_pos.0 -= 2;
            }
        }
        if rl_data.rl.is_key_down(KeyboardKey::KEY_UP) {
            if !(self.temp_c.ball_pos.1 <= 0) {
                self.temp_c.ball_pos.1 -= 2;
            }
        }
        if rl_data.rl.is_key_down(KeyboardKey::KEY_DOWN) {
            if !(self.temp_c.ball_pos.1 >= rl_data.screen.height) {
                self.temp_c.ball_pos.1 += 2;
            }
        }

        mouse_pos = rl_data.rl.get_mouse_position();
        let mut d = rl_data.rl.begin_drawing(&rl_data.raylib_thread);

        d.draw_circle(self.temp_c.ball_pos.0, self.temp_c.ball_pos.1, 10f32, Color::MAROON);
        d.clear_background(Color::WHITE);

        d.draw_circle_v(mouse_pos, 20f32, Color::BLUE);
//}

        for e in self.entities.iter() {
            let c = cm.get::<Coords>(&e);
            if let Some(c) = c {
                let c = Coords {x: c.x, y: c.y};
                let color = match cm.get::<MyColor>(&e) {
                    Some(color) => color,
                    None => &mut MyColor { c: Color::CYAN},
                };
                d.draw_circle(c.x, c.y, 5f32, color.c);
            }
        }

    }
}

pub struct MouseInput {
    entities: HashSet<Entity>,
    component_types: HashSet<TypeId>,
}

impl MouseInput {
    pub fn new() -> MouseInput {
        MouseInput {
            entities: HashSet::new(),
            component_types: HashSet::from_iter(vec![ComponentType::of::<Coords>()]),
        }
    }
}

impl System for MouseInput {
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
        let mut mouse_coords = cm.get_global::<Coords>(1).unwrap();

        println!("mouse input, coords {:?}", mouse_coords);

        mouse_coords.x += 1;
        mouse_coords.y += 1;
//        let mouse_pos = self.rl.get_mouse_position();
        //d.draw_circle_v(mouse_pos, 20f32, Color::BLUE);
//        let coords = Coords {mouse_pos.};
//
//        for e in self.entities.iter() {
//            let coords = cm.get::<Coords>(e);
//            cm.add
//        }

    }
}
fn main() {
    let mut c = Coordinator::new();

    let (width, height) = (640, 480);
    let mut rl_data = RayLibData::new(width, height);
    rl_data.rl.set_target_fps(5);
    c.add_global(2, rl_data);

    let mouse_coords = Coords { x: 0, y: 0 }; // TODO: remove this
    c.add_global(1, mouse_coords);           // TODO: remove this

    let e1 = c.get_entity();
    let e2 = c.get_entity();

    c.register_system(Render::new()); // TODO: this block of registered systems should
                                      // also work if move after block of registered component
                                      // types, and adding components to coordinato
    c.register_system(IntegrateVelocity::new());
    c.register_system(MouseInput::new());

    c.register_component::<Coords>();
    c.register_component::<MyColor>();
    c.register_component::<Velocity>();

    c.add_component(e1, Coords{x: 20, y: 30});
    c.add_component(e1, MyColor {c: Color::RED});

    c.add_component(e2, Coords{x: 20, y: 60});
    c.add_component(e2, Velocity{vx: 1f64, vy: 2f64});

    loop {
        c.apply_all();
    }
}
