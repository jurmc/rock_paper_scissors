use crate::Velocity;
use crate::Coords;

use ecs::ComponentType;
use ecs::ComponentManager;
use ecs::Coordinator;
use ecs::Entity;
use ecs::System;

use std::collections::HashSet;

pub struct IntegrateVelocity {
    entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,
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

    fn get_component_types(&self) -> &HashSet<ComponentType> {
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
