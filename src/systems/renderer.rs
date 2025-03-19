use crate::RayLibData;
use crate::Coords;
use crate::MyColor;
use crate::MySize;

use ecs::ComponentType;
use ecs::ComponentManager;
use ecs::Entity;
use ecs::System;
use ecs::Coordinator;
use raylib::prelude::*;

use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Renderer {
    entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,

    ray_lib_data: Rc<RefCell<RayLibData>>,
}

impl Renderer {
    pub fn new(ray_lib_data: Rc<RefCell<RayLibData>>) -> Renderer {
        ray_lib_data.borrow().rl.borrow_mut().set_target_fps(20);

        let renderer = Renderer {
            entities: HashSet::new(),
            component_types: HashSet::new(),

            ray_lib_data,
        };

        renderer
    }
}

impl System for Renderer {
    fn add(&mut self, e: Entity) {
        self.entities.insert(e);
    }
    fn remove(&mut self, e: Entity) {
        // TODO: not implemented
    }

    fn get_component_types(&self) -> &HashSet<ComponentType> {
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
        d.clear_background(Color::DARKGRAY);
        let screen = ray_lib_data.screen.borrow();
        d.draw_rectangle(2, 2, screen.width - 4, screen.height - 4, Color::GRAY);
        d.draw_rectangle(screen.width , 2, 240-2, screen.height - 4, Color::GRAY);

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
