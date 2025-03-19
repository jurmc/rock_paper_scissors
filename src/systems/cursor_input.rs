use crate::RayLibData;
use crate::Entity;
use crate::ComponentType;
use crate::Coords;
use crate::CursorControlled;

use raylib::prelude::*;
use ecs::ComponentManager;
use ecs::Coordinator;
use ecs::System;

use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

pub struct CursorInput {
    entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,

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

    fn get_component_types(&self) -> &HashSet<ComponentType> {
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

