use crate::Coords;
use crate::Velocity;
use crate::Weight;
use crate::TTL;
use crate::MySize;
use crate::MyColor;

use ecs::ComponentType;
use ecs::ComponentManager;
use ecs::Entity;
use ecs::System;
use ecs::Coordinator;
use raylib::prelude::*;
use rand::Rng;

use std::collections::HashSet;

pub struct Reaper {
    entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,
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

    fn get_component_types(&self) -> &HashSet<ComponentType> {
        &self.component_types
    }

    fn apply(&mut self, cm: &mut ComponentManager) -> Box<dyn Fn(&mut Coordinator)> {
        for e in self.entities.iter() {
            let mut kill_flag = false;
            if let Some(ttl) = cm.get_mut::<TTL>(e) {
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
