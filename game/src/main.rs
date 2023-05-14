use std::{dbg, println, cell::{RefCell, RefMut}};

use chroma::Chroma;
use winit::{event_loop::{EventLoop, ControlFlow}, 
            window::WindowBuilder, dpi::PhysicalSize, 
            event::VirtualKeyCode};

use winit_input_helper::WinitInputHelper;

// In milliseconds
const TICK_DURATION: u16 = 15;

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

struct Sprite<'a>(&'a str);

struct Position {
    x: f32,
    y: f32
}

struct Velocity {
    x: f32,
    y: f32
}

struct Moveable(f32);

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 224;

const SCREEN_SCALE: u32 = 3;

const WINDOW_WIDTH: u32 = SCREEN_WIDTH * SCREEN_SCALE;
const WINDOW_HEIGHT: u32 = SCREEN_HEIGHT * SCREEN_SCALE;

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
        .with_inner_size(PhysicalSize { width: WINDOW_WIDTH, height: WINDOW_HEIGHT})
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")] {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));

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

    // Renderer

    let mut chroma = Chroma::new(SCREEN_WIDTH as u16, SCREEN_HEIGHT as u16, &window);

    // ECS

    let mut world = World::new();

    let e = world.new_entity();

    world.add_component_to_entity(e, Sprite("stone"));
    world.add_component_to_entity(e, Position {x: 30.0, y: 15.0});
    world.add_component_to_entity(e, Velocity {x: 3.0, y: 0.0});

    let e = world.new_entity();

    world.add_component_to_entity(e, Sprite("grass"));
    world.add_component_to_entity(e, Position {x: 70.0, y: 80.0});
    world.add_component_to_entity(e, Velocity {x: 7.0, y: 0.1});

    let e = world.new_entity();

    world.add_component_to_entity(e, Sprite("sentinel"));
    world.add_component_to_entity(e, Position {x: 3.0, y: 5.0});
    world.add_component_to_entity(e, Velocity {x: 0.0, y: 0.0});
    world.add_component_to_entity(e, Moveable(2.0));

    let mut input_grabber = WinitInputHelper::new();

    // EVENT LOOP

    let mut last_tick = instant::now();
    let mut tick_time = 0.0;

    event_loop.run(move |event, _, control_flow| {

        if input_grabber.update(&event) {
            tick_time += instant::now() - last_tick;
            last_tick = instant::now();

            let input = input_manager(&mut input_grabber, control_flow);
        
            let mut sprite_components = world.borrow_component_vec::<Sprite>().unwrap();
            let mut position_components = world.borrow_component_vec::<Position>().unwrap();
    
            let mut velocity_components = world.borrow_component_vec::<Velocity>().unwrap();

            let mut moveable_components = world.borrow_component_vec::<Moveable>().unwrap();

            let zip = velocity_components.iter_mut().zip(moveable_components.iter_mut());

            for (velocity, moveable) in zip.filter_map(|(velocity, moveable)| Some((velocity.as_mut()?, moveable.as_mut()?))) {
                
                let dir_x : f32 = if input.right_pressed { 1.0 } else if input.left_pressed { -1.0 } else { 0.0 };
                let dir_y : f32 = if input.up_pressed { -1.0 } else if input.down_pressed { 1.0 } else { 0.0 };

                let magnitude = dir_x.abs() + dir_y.abs();

                let normalized_x = dir_x / magnitude;
                let normalized_y = dir_y / magnitude;

                if magnitude != 0.0 {
                    velocity.x = normalized_x * moveable.0;
                    velocity.y = normalized_y * moveable.0;
                }
            }

            while tick_time >= TICK_DURATION as f64 {
                // FIXED UPDATE LOGIC
                
                let zip = velocity_components.iter_mut().zip(position_components.iter_mut());

                for (velocity, position) in zip.filter_map(|(velocity, position)| Some((velocity.as_mut()?, position.as_mut()?))) {
                    position.x += velocity.x;
                    position.y += velocity.y;

                    velocity.x -= velocity.x * 0.1;
                    velocity.y -= velocity.y * 0.1;
                }
                // FIXED UPDATE LOGIC END

                tick_time -= TICK_DURATION as f64;
            }

            let zip = sprite_components.iter_mut().zip(position_components.iter_mut());
        
            chroma.clear();

            for (sprite, position) in zip.filter_map(|(sprite, position)| Some((sprite.as_mut()?, position.as_mut()?))) {
                chroma.draw_sprite(asset_loader::get_sprite(&sprite.0), position.x as u32, position.y as u32);
            }
        
            chroma.render();
        }
    });
}

fn input_manager(input: &mut WinitInputHelper, control_flow: &mut ControlFlow) -> Input {

    let up_pressed = input.key_held(VirtualKeyCode::Up);
    let down_pressed = input.key_held(VirtualKeyCode::Down);
    let left_pressed = input.key_held(VirtualKeyCode::Left);
    let right_pressed = input.key_held(VirtualKeyCode::Right);

    if input.key_released(VirtualKeyCode::Escape) || input.close_requested() || input.destroyed() {
        *control_flow = ControlFlow::Exit;
    }

    Input {
        up_pressed,
        down_pressed,
        left_pressed,
        right_pressed
    }
}

struct Input {
    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool
}