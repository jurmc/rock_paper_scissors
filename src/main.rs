use ecs::Entity;
use ecs::Coordinator;
use ecs::ComponentManager;
use ecs::ComponentType;
use ecs::System;
use ecs::Globals;

use raylib::prelude::*;
use rand::Rng;

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

pub struct Reaper {
    entities: HashSet<Entity>,
    component_types: HashSet<TypeId>,
}

impl Reaper {
    pub fn new() -> Reaper {
        Reaper {
            entities: HashSet::new(),
            component_types: HashSet::from_iter(vec![
                ComponentType::of::<TTL>(),
            ]),
        }
    }
}

impl System for Reaper {
    fn add(&mut self, e: Entity) {
        self.entities.insert(e);
    }
    fn remove(&mut self, e: Entity) {
        // TODO: not implemented
    }

    fn get_component_types(&self) -> &HashSet<TypeId> {
        &self.component_types
    }

    fn apply(&mut self, cm: &mut ComponentManager) -> Box<dyn Fn(&mut Coordinator)> {
        for e in self.entities.iter() {
            let mut kill_flag = false;
            if let Some(ttl) = cm.get_mut::<TTL>(e) {
                println!("ttl: {}", ttl.ttl);
                ttl.ttl -= 1;

                if ttl.ttl <= 0 {
                    kill_flag = true;
                }

                if kill_flag {
                    cm.remove::<TTL>(e);
                    if let Some(coords) = cm.get::<Coords>(e) {
                        let (x, y) = (coords.x, coords.y);
                        return Box::new(move |c| {
                            for _ in 1..2 {
                                for _ in 1..10 {
                                    let v = 20f32;
                                    let angle_deg = rand::rng().random_range(1f32..=360f32);
                                    let angle_rad = angle_deg / std::f32::consts::PI;
                                    let vx = (v * angle_rad.cos()) as f64;
                                    let vy = (v * angle_rad.sin()) as f64;

                                    let new_e = c.get_entity();
                                    c.add_component(new_e, Coords {x, y});
                                    c.add_component(new_e, Velocity {vx, vy});
                                    c.add_component(new_e, Weight { w: 2 });
                                    c.add_component(new_e, MySize { s: 2f32 });
                                    c.add_component(new_e, MyColor { c: Color::BLUEVIOLET });
                                }
                            }
                        });
                    }
                }
            }
        }
        Box::new(| _coordinator | {})
    }
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

    fn apply(&mut self, cm: &mut ComponentManager) -> Box<dyn Fn(&mut Coordinator)> {
        for e in self.entities.iter() {
            if let Some(v) = cm.get::<Velocity>(e) {
                let v = Velocity {vx: v.vx, vy: v.vy};
                if let Some(c) = cm.get::<Coords>(e) {
                    let new_coords = Coords { x: c.x + v.vx.round() as i32, y: c.y + v.vy.round() as i32 };
                    cm.add(*e, new_coords);
                }
            }
        }

        Box::new(| _coordinator | {})
    }
}

pub struct Render {
    entities: HashSet<Entity>,
    component_types: HashSet<TypeId>,

    ray_lib_data: Rc<RefCell<RayLibData>>,
}

impl Render {
    pub fn new(ray_lib_data: Rc<RefCell<RayLibData>>) -> Render {
        ray_lib_data.borrow().rl.borrow_mut().set_target_fps(20);

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

    fn apply(&mut self, cm: &mut ComponentManager) -> Box<dyn Fn(&mut Coordinator)> {
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
                let size = match cm.get::<MySize>(&e) {
                    Some(size) => size.s,
                    None => 10f32,
                };
                let color = match cm.get::<MyColor>(&e) {
                    Some(color) => color,
                    None => &mut MyColor { c: Color::CYAN},
                };
                d.draw_circle(c.x, c.y, size, color.c);
            }
        }

        Box::new(| _coordinator | {})
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

    fn apply(&mut self, cm: &mut ComponentManager) -> Box<dyn Fn(&mut Coordinator)> {
        let mouse_pos = self.rl.borrow().get_mouse_position().clone();

        for e in self.entities.iter() {
            cm.add(*e, Coords {
                x: mouse_pos.x.round() as i32,
                y: mouse_pos.y.round() as i32 });
        }

        if self.rl.borrow().is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            println!("our closure defined");
            let (x, y) = (mouse_pos.x.round() as i32, mouse_pos.y.round() as i32 );
            return Box::new(move | c| {
                let e = c.get_entity();
                let coords = Coords { x, y };
                println!("our closure triggered");
                c.add_component(e, coords);
                c.add_component(e, MySize { s: 3f32 });
                //c.add_component(e, Weight { w: 1 });
                c.add_component(e, TTL { ttl: 40 });
                c.add_component(e, MyColor { c: Color::INDIANRED });
            })
        }

        Box::new(| _ | {})
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

    fn apply(&mut self, cm: &mut ComponentManager) -> Box<dyn Fn(&mut Coordinator)> {
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

        Box::new(| _  | {})
    }
}

struct TTL {
    ttl: i32,
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

    fn apply(&mut self, cm: &mut ComponentManager) -> Box<dyn Fn(&mut Coordinator)> {
        for e in self.entities.iter() {
            let w = cm.get::<Weight>(e).unwrap().w;
            let new_potential_v = &mut Velocity { vx: 0f64, vy: 0f64 };
            let v = cm.get::<Velocity>(e).or(Some(new_potential_v)).unwrap();
            let newv = Velocity { vx: v.vx, vy: v.vy + w as f64 };
            cm.add::<Velocity>(*e, newv)
        }

        Box::new(| _ | {})
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
    let e0 = c.get_entity();
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
