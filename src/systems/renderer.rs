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
use std::boxed::Box;

pub struct Renderer {
    entities: HashSet<Entity>,
    component_types: HashSet<ComponentType>,

    ray_lib_data: Rc<RefCell<RayLibData>>,
    draw_cmds: Box<dyn Fn(&mut RaylibDrawHandle)>,
}


impl Renderer {
    pub fn new(ray_lib_data: Rc<RefCell<RayLibData>>) -> Renderer {
        ray_lib_data.borrow().rl.borrow_mut().set_target_fps(20);

        let renderer = Renderer {
            entities: HashSet::new(),
            component_types: HashSet::new(),

            ray_lib_data,
            draw_cmds: Renderer::empty_cmds(),
        };

        renderer
    }

    pub fn draw_buffered_cmds(&mut self, d: &mut RaylibDrawHandle) {
        let cmds = &mut self.draw_cmds;
        cmds(d);
        self.draw_cmds = Renderer::empty_cmds();
    }

    fn empty_cmds() -> Box<dyn Fn(&mut RaylibDrawHandle)> {
        Box::new(|_| {})
    }

    // Extract this to main game loop
    fn draw_gui(&self, d: &mut RaylibDrawHandle, gui_x: i32, gui_y: i32) {
        d.draw_line(30, 30, 130, 130, Color::DEEPSKYBLUE);

        d.gui_label(
            rrect(gui_x + 5, gui_y + 5, 100, 30),
            "Entities - label"
            );

        if d.gui_button( rrect(gui_x + 5, gui_y + 35, 100, 30), "Add") {
            println!("entities.push_str(\"\nentityX\");");
        }

        d.gui_list_view(
            rrect(gui_x +5, gui_y + 70, 100, 200),
            "abc\ndef\nghjikl",
            &mut 1,
            &mut 1);
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
        let screen = ray_lib_data.screen.borrow();

        d.clear_background(Color::DARKGRAY);
        //d.draw_rectangle(2, 2, screen.width - 4, screen.height - 4, Color::GRAY);
        //d.draw_rectangle(screen.width , 2, 240-2, screen.height - 4, Color::GRAY);

        let (gui_x, gui_y) = (screen.width, 0);
        self.draw_gui(&mut d, gui_x, gui_y);

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

        self.draw_cmds = Box::new(|h| {
            h.draw_circle(50, 150, 30f32, Color::DEEPPINK);
        });

        Box::new(| _coordinator | {})
    }
}
