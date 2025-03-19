use crate::Weight;
use crate::Velocity;

use ecs::ComponentType;
use ecs::ComponentManager;
use ecs::Entity;
use ecs::System;
use ecs::Coordinator;

use std::collections::HashSet;

pub struct Gravity {
    entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,
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

    fn get_component_types(&self) -> &HashSet<ComponentType> {
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
