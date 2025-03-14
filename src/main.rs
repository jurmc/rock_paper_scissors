use ecs::Entity;
use ecs::Coordinator;
use ecs::ComponentManager;
use ecs::ComponentType;
use ecs::System;
use ecs::Globals;

use raylib::prelude::*;

use std::collections::HashSet;
use std::any::TypeId;
use std::rc::Rc;
use std::cell::RefCell;

// TODO: all geters in ECS needs to have mutable and imutable versions

pub struct RayLibData {
    rl: Rc<RefCell<RaylibHandle>>,
    raylib_thread: Rc<RefCell<RaylibThread>>,
    screen: Rc<RefCell<Screen>>,
}

impl RayLibData {
    pub fn new(width: i32, height: i32) -> RayLibData {
        let (mut rl, raylib_thread) = raylib::init()
            .size(width, height)
            .title("Updated RenderSystem1")
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

    ray_lib_data: Rc<RefCell<RayLibData>>,
}

impl Render {
    pub fn new(ray_lib_data: Rc<RefCell<RayLibData>>) -> Render {
        let (width, height) = (640, 480);
        ray_lib_data.borrow().rl.borrow_mut().set_target_fps(30);

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

            ray_lib_data,
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

        let ray_lib_data = self.ray_lib_data.borrow_mut();

        let mut rl= ray_lib_data.rl.borrow_mut();
        let raylib_thread = ray_lib_data.raylib_thread.borrow();

        if rl.window_should_close() {
            panic!("Exitted..."); // TODO: this condition should rather be somehow signalled to the
                                  // outside world...
        }


        let mut d = rl.begin_drawing(&raylib_thread);
        d.clear_background(Color::WHITE);

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

    rl: Rc<RefCell<RaylibHandle>>,
}

impl MouseInput {
    pub fn new(ray_lib_data: Rc<RefCell<RayLibData>>) -> MouseInput {
        MouseInput {
            entities: HashSet::new(),
            component_types: HashSet::from_iter(vec![
                ComponentType::of::<Coords>(),
                ComponentType::of::<MouseControlled>(),
            ]),

            rl: ray_lib_data.borrow().rl.clone(),
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
        let mouse_pos = self.rl.borrow().get_mouse_position();

        for e in self.entities.iter() {
            cm.add(*e, Coords {
                x: mouse_pos.x.round() as i32,
                y: mouse_pos.y.round() as i32 });
        }
    }
}

pub struct CursorInput {
    entities: HashSet<Entity>,
    component_types: HashSet<TypeId>,

    rl: Rc<RefCell<RaylibHandle>>,
}

impl CursorInput {
    pub fn new(ray_lib_data: Rc<RefCell<RayLibData>>) -> CursorInput {
        CursorInput {
            entities: HashSet::new(),
            component_types: HashSet::from_iter(vec![
                ComponentType::of::<Coords>(),
                ComponentType::of::<CursorControlled>(),
            ]),

            rl: ray_lib_data.borrow().rl.clone(),
        }
    }
}

impl System for CursorInput {
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
        let (mut inc_x, mut inc_y) = (0, 0);
        let rl = self.rl.borrow();

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            inc_x += 1;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            inc_x -= 1;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            inc_y -= 1;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            inc_y +=1;
        }

        for e in self.entities.iter() {
            let old = cm.get::<Coords>(e).unwrap();
            let new = Coords { x: old.x + inc_x, y: old.y + inc_y };
            cm.add(*e, new);
        }
    }
}

struct Weight {
    w: i32,
}

pub struct Gravity {
    entities: HashSet<Entity>,
    component_types: HashSet<TypeId>,
}

impl Gravity {
    pub fn new() -> Gravity {
        Gravity {
            entities: HashSet::new(),
            component_types: HashSet::from_iter(vec![
                ComponentType::of::<Weight>(),
            ]),
        }
    }
}

impl System for Gravity {
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
            let w = cm.get::<Weight>(e).unwrap().w;
            let new_potential_v = &mut Velocity { vx: 0f64, vy: 0f64 };
            let v = cm.get::<Velocity>(e).or(Some(new_potential_v)).unwrap();
            let newv = Velocity { vx: v.vx, vy: v.vy + w as f64 };
            cm.add::<Velocity>(*e, newv)
        }
    }
}

struct MouseControlled {}
struct CursorControlled {}

fn main() {

    let mut globals = Globals::new();

    let (width, height) = (640, 480);
    let rl_data = RayLibData::new(width, height);
    let rl_data = Rc::new(RefCell::new(rl_data));

    let render_sys = Render::new(rl_data.clone());
    let mouse_input_sys = MouseInput::new(rl_data.clone());
    let cursor_input_sys = CursorInput::new(rl_data.clone());

    let mut c = Coordinator::new();

    let mouse = c.get_entity();
    let cursor = c.get_entity();
    let e1 = c.get_entity();
    let e2 = c.get_entity();
    let e3 = c.get_entity();

    c.register_system(render_sys); // TODO: this block of registered systems should
                                      // also work if move after block of registered component
                                      // types, and adding components to coordinato
    c.register_system(mouse_input_sys);
    c.register_system(cursor_input_sys);
    c.register_system(IntegrateVelocity::new());
    c.register_system(Gravity::new());

    c.register_component::<Coords>();
    c.register_component::<MyColor>();
    c.register_component::<Velocity>();
    c.register_component::<MouseControlled>();
    c.register_component::<CursorControlled>();
    c.register_component::<Weight>();

    c.add_component(mouse, Coords {x: 70, y: 90});
    c.add_component(mouse, MyColor {c: Color::ORANGE});
    c.add_component(mouse, MouseControlled{});

    c.add_component(cursor, Coords { x: 30, y: 130 });
    c.add_component(cursor, MyColor {c: Color::INDIGO});
    c.add_component(cursor, CursorControlled{});

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
        c.apply_all();
    }
}
