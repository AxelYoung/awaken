use std::{dbg, println, cell::{RefCell, RefMut}};

use chroma::Chroma;
use winit::{event_loop::{EventLoop, ControlFlow}, 
            window::WindowBuilder, dpi::PhysicalSize, 
            event::{Event, WindowEvent, ElementState, VirtualKeyCode, KeyboardInput}};

struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>
}

impl World {
    fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new()
        }
    }

    fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    fn add_component_to_entity <ComponentType: 'static> (
        &mut self, 
        entity: usize, 
        component: ComponentType) {
            for component_vec in self.component_vecs.iter_mut() {
                if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<RefCell<Vec<Option<ComponentType>>>>() {
                    component_vec.get_mut()[entity] = Some(component);
                    return;
                }
            }

            let mut new_component_vec : Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);
        
            for _ in 0..self.entities_count {
                new_component_vec.push(None);
            }

            new_component_vec[entity] = Some(component);
            self.component_vecs.push(Box::new(RefCell::new(new_component_vec)));
    }

    fn borrow_component_vec <ComponentType: 'static> (&self) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec.as_any().downcast_ref::<RefCell<Vec<Option<ComponentType>>>>() {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }
}

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn push_none(&mut self) {
        self.get_mut().push(None);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

fn main() {
    run();
}

struct Name(String);

struct Position {
    x: f32,
    y: f32
}

struct Health(i32);

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn run() {

    // CREATE EVENT LOOP

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }    

    let event_loop = EventLoop::new();

    // CREATE WINDOW

    let window = WindowBuilder::new()
        .with_title("Awaken")
        .with_inner_size(PhysicalSize { width: 640, height: 640})
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")] {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(640, 640));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    // ECS

    let mut world = World::new();

    let e = world.new_entity();

    world.add_component_to_entity(e, Name("First Entity".to_owned()));

    let e = world.new_entity();

    world.add_component_to_entity(e, Name("Second Entity".to_owned()));

    world.add_component_to_entity(e, Health(10));
    world.add_component_to_entity(e, Position {x: 0.5, y: 1.4});

    let mut name_components = world.borrow_component_vec::<Name>().unwrap();

    for name_component in name_components.iter_mut().filter_map(|f| f.as_mut()) {
        println!("{}", name_component.0);
    }

    let mut health_components = world.borrow_component_vec::<Health>().unwrap();

    for health_component in health_components.iter_mut().filter_map(|f| f.as_mut()) {
        dbg!(health_component.0);
        health_component.0 = -10;
        dbg!(health_component.0);
    }

    // EVENT LOOP

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) => {

        },
        Event::MainEventsCleared => {

        },
        Event::WindowEvent {
            window_id,
            ref event,
        } => {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {

                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {

                },
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Space),
                            ..
                        },
                    ..
                } => {

                }
                _ => {}
            }
        },
        _ => {}
    });
}