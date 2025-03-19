pub mod systems;

use systems::CursorInput;
use systems::Reaper;
use systems::IntegrateVelocity;
use systems::Gravity;
use systems::Renderer;
use systems::MouseInput;

use ecs::Entity;
use ecs::Coordinator;
use ecs::ComponentType;
use ecs::Globals;
use raylib::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;


pub struct RayLibData {
    rl: Rc<RefCell<RaylibHandle>>,
    raylib_thread: Rc<RefCell<RaylibThread>>,
    screen: Rc<RefCell<Screen>>,
}

impl RayLibData {
    pub fn new(width: i32, height: i32) -> RayLibData {
        let gui_width = 240;

        let (mut rl, raylib_thread) = raylib::init()
            .size(width + gui_width, height)
            .title("ECS demo")
            .build();

        let screen = Screen {
            width,
            height,
        };

        RayLibData {
            rl: Rc::new(RefCell::new(rl)).clone(),
            raylib_thread: Rc::new(RefCell::new(raylib_thread)).clone(),
            screen: Rc::new(RefCell::new(screen)).clone(),
        }
    }
}

#[derive(Debug)]
pub struct Coords {
    x: i32,
    y: i32,
}

pub struct MySize {
    s: f32,
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


struct TTL {
    ttl: i32,
}

struct Weight {
    w: i32,
}

struct MouseControlled {}
struct CursorControlled {}

fn main() {

    let mut globals = Globals::new();

    let (width, height) = (640, 480);
    let rl_data = RayLibData::new(width, height);
    let rl_data = Rc::new(RefCell::new(rl_data));

    let renderer_sys = Renderer::new(rl_data.clone());
    let mouse_input_sys = MouseInput::new(rl_data.clone());
    let cursor_input_sys = CursorInput::new(rl_data.clone());

    let mut c = Coordinator::new();

    let mouse = c.get_entity();
    let cursor = c.get_entity();
    let e0 = c.get_entity();
    let e1 = c.get_entity();
    let e2 = c.get_entity();
    let e3 = c.get_entity();

    c.register_system(renderer_sys); // TODO: this block of registered systems should
                                      // also work if move after block of registered component
                                      // types, and adding components to coordinato
    c.register_system(mouse_input_sys);
    c.register_system(cursor_input_sys);
    c.register_system(IntegrateVelocity::new());
    c.register_system(Gravity::new());
    c.register_system(Reaper::new());

    c.register_component::<Coords>();
    c.register_component::<MyColor>();
    c.register_component::<Velocity>();
    c.register_component::<MouseControlled>();
    c.register_component::<CursorControlled>();
    c.register_component::<Weight>();
    c.register_component::<MySize>();
    c.register_component::<TTL>();

    c.add_component(mouse, Coords {x: 70, y: 90});
    c.add_component(mouse, MyColor {c: Color::ORANGE});
    c.add_component(mouse, MouseControlled{});
    c.add_component(mouse, MySize { s: 0f32 });

    c.add_component(cursor, Coords { x: 30, y: 130 });
    c.add_component(cursor, MyColor {c: Color::INDIGO});
    c.add_component(cursor, CursorControlled{});

    c.add_component(e0, Coords { x: 230, y: 130 });
    c.add_component(e0, MyColor {c: Color::INDIGO});
    c.add_component(e0, TTL { ttl: 40 });

    c.add_component(e1, Coords{x: 20, y: 30});
    c.add_component(e1, MyColor {c: Color::GRAY});
    c.add_component(e1, Velocity{vx: 5f64, vy: 0f64});
    c.add_component(e1, Weight { w: 1 });

    c.add_component(e2, Coords{x: 20, y: 60});
    c.add_component(e2, Velocity{vx: 1f64, vy: 2f64});
    c.add_component(e2, Weight { w: 1 });

    c.add_component(e3, Coords{x: 500, y: 400});
    c.add_component(e3, Velocity{vx: -2f64, vy: 0f64});
    c.add_component(e3, Weight { w: -1 });

    loop {
        let updates = c.apply_all();
        for update in updates.iter() {
            update(&mut c);
        }
    }
}
