#![allow(non_snake_case)]

use harmony::*;
use chroma::Chroma;
use rand::Rng;
use winit::{event_loop::{EventLoop, ControlFlow}, 
            window::WindowBuilder, dpi::PhysicalSize, 
            event::VirtualKeyCode};

use winit_input_helper::WinitInputHelper;
use itertools::multizip;

// In milliseconds
const TICK_DURATION: u16 = 15;

const MAP: [[u8;16];14] = [
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,0,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
];

fn main() {
    run();
}

#[derive(Debug)]
struct Sprite<'a> {
    name: &'a str,
    index: Option<u32>
}

#[derive(PartialEq, Debug)]
struct Position {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32
}

struct Collider();
struct Moveable {
    speed: f32
}

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 224;

const SCREEN_SCALE: u32 = 4;

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

    create_map_entities(&mut world);

    create_player_entity(&mut world);

    let e = world.new_entity();
    
    world.add_component_to_entity(e, Sprite{ name: "stone", index: None });
    world.add_component_to_entity(e, Position { x: 20.0, y: 50.0} );
    world.add_component_to_entity(e, Velocity { x: 4.5, y: 0.0} );
    world.add_component_to_entity(e, Collider());

    let e = world.new_entity();
    
    world.add_component_to_entity(e, Sprite{ name: "stone", index: None });
    world.add_component_to_entity(e, Position { x: 200.0, y: 50.0} );
    world.add_component_to_entity(e, Velocity { x: -4.5, y: 0.0} );
    world.add_component_to_entity(e, Collider());

    let e = world.new_entity();
    
    world.add_component_to_entity(e, Sprite{ name: "stone", index: None });
    world.add_component_to_entity(e, Position { x: 100.0, y: 80.0} );
    world.add_component_to_entity(e, Velocity { x: 10.5, y: 2.0} );
    world.add_component_to_entity(e, Collider());

    let mut input = WinitInputHelper::new();

    // EVENT LOOP
    
    let mut last_tick = instant::now();
    let mut tick_time = 0.0;

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            update(&mut world, &input_manager(&mut input, control_flow));

            tick_manager(&mut world, &mut tick_time, &mut last_tick);
        
            draw(&mut world, &mut chroma);
        }
    });
}

fn update(world: &mut World, input: &Input) {
    set_entity_velocity(world, input);
    turn(world);
}

fn tick_manager(world: &mut World, tick_time: &mut f64, last_tick: &mut f64) {
    *tick_time += instant::now() - *last_tick;
    *last_tick = instant::now();

    while *tick_time >= TICK_DURATION as f64 {
        fixed_update(world);
        *tick_time -= TICK_DURATION as f64;
    }
}

fn fixed_update(world: &mut World) {
    check_collision_iter(world);
    move_entity(world);
    velocity_drag_iterator(world);
}

fn draw(world: &mut World, chroma: &mut Chroma){
    chroma.clear();
        
    draw_entity(world, chroma);

    chroma.render();
}

fn draw_entity(world: &mut World, chroma: &mut Chroma) {
    iterate_entities!(world, [Sprite, Position], 
        |sprite : &Sprite, position: &Position| {
            let sprite_data = asset_loader::get_sprite(sprite.name);

            let x = position.x as u32;
            let y = position.y as u32;
        
            if let Some(index) = sprite.index {
                chroma.draw_sprite_from_sheet(sprite_data, index, x, y);
            } else {
                chroma.draw_sprite(sprite_data, x, y);
            }
        }
    );
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

fn create_map_entities(world: &mut World) {
    for x in 0..16 {
        for y in 0..14 {
            let sprite = Sprite {
                name: match MAP[y][x] {
                    0 => {"stone"},
                    _ => match rand::thread_rng().gen_bool(0.5) {
                        true => {"dirt"},
                        false => {"grass"}
                    }
                },
                index: None
            };

            let position = Position { 
                x: x as f32 * 16.0,
                y: y as f32 * 16.0
            };
            let e = world.new_entity();
            world.add_component_to_entity(e, sprite);
            world.add_component_to_entity(e, position);
            if MAP[y][x] == 0 {
                world.add_component_to_entity(e, Collider());
            }
        }
    }
}

fn create_player_entity(world: &mut World) {
    let e = world.new_entity();

    world.add_component_to_entity(e, Sprite{name: "sentinel", index: Some(1)});
    world.add_component_to_entity(e, Position {x: SCREEN_WIDTH as f32 / 2.0, y: SCREEN_HEIGHT as f32 / 2.0} );
    world.add_component_to_entity(e, Velocity {x: 0.0, y: 0.0} );
    world.add_component_to_entity(e, Moveable { speed: 1.0 });
    world.add_component_to_entity(e, Collider());
}

fn move_entity(world: &mut World) {
    let expr = |velocity: &Velocity, position: &mut Position| {
        position.x += velocity.x;
        position.y += velocity.y;
    
        position.x = position.x.clamp(0.0, SCREEN_WIDTH as f32 - 16.0);
        position.y = position.y.clamp(0.0, SCREEN_HEIGHT as f32 - 16.0);
    };

    iterate_entities!(world, [Velocity], (Position), expr);
}

fn turn(world: &mut World) {
    let expr = |velocity: &Velocity, _, sprite: &mut Sprite| {
        if velocity.y.abs() > 0.1 {
            if velocity.y > 0.0 {
                sprite.index = Some(0);
            } else if velocity.y < 0.0 {
                sprite.index = Some(1);
            }
        } else if velocity.x.abs() > 0.1 {
            if velocity.x > 0.0 {
                sprite.index = Some(2);
            } else if velocity.x < 0.0 {
                sprite.index = Some(3);
            }
        }
    };

    iterate_entities!(world, [Velocity, Moveable], (Sprite), expr);
}


fn velocity_drag_iterator(world: &mut World) {
    let expr = |velocity: &mut Velocity| {
        velocity.x -= velocity.x * 0.05;
        velocity.y -= velocity.y * 0.05;
    };

    iterate_entities!(world, (Velocity,), expr);
}


fn check_collision_iter(world: &mut World) {

    let mut collided : Vec<usize> = vec![];
    let mut collided_velocities : Vec<(f32, f32)> = vec![];
{
    let positions = world.borrow_components::<Position>().unwrap();
    let colliders = world.borrow_components::<Collider>().unwrap();
    let mut velocities = world.borrow_components_mut::<Velocity>().unwrap();

    let filter = positions.iter().zip(colliders.iter()).zip(velocities.iter_mut()).enumerate();

    for (e, (position_a, _), velocity) 
        in filter.filter_map(|(e, ((position, collider), velocity))| Some((e, (position.as_ref()?, collider.as_ref()?), velocity.as_mut()?))) {
            let filter_b = positions.iter().zip(colliders.iter());
            
            for (position_b, _) in filter_b.filter_map(|(position, collider)| Some((position.as_ref()?, collider.as_ref()?))) {
                if position_a == position_b { continue; }
                if check_collision(position_a, position_b) {
                    collided_velocities.push((velocity.x / 2.0, velocity.y / 2.0));
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                    collided.push(e);
                }
            }
    }
}
    for (e, velocity) in collided.iter().zip(collided_velocities) {
        if let Some(Some(position)) = world.get_component_from_entity::<Position>(*e) {
            position.x -= velocity.0;
            position.y -= velocity.1;
        }
    }
}

fn check_collision(pos_a: &Position, pos_b: &Position) -> bool {
    let right_a = pos_a.x + 16.0;
    let bot_a = pos_a.y + 16.0;
    let right_b = pos_b.x + 16.0;
    let bot_b = pos_b.y + 16.0;

    if pos_a.x < right_b && right_a > pos_b.x && pos_a.y < bot_b && bot_a > pos_b.y {

        let center_a = (pos_a.x + 8.0, pos_a.y + 8.0);
        let center_b = (pos_b.x + 8.0, pos_b.y + 8.0);

        let dir = (pos_b.x - pos_a.x, pos_b.y - pos_a.y);
        let normal = (-dir.1, dir.0);

        let normal_length = (normal.0 * normal.0 + normal.1 * normal.1).sqrt();
        let normalized_normal = (normal.0 / normal_length, normal.1 / normal_length);

        return true;
    }

    false
}

fn set_entity_velocity(world: &mut World, input: &Input) {
    let expr = |moveable: &Moveable, velocity: &mut Velocity| {
        let dir_x : f32 = if input.right_pressed { 1.0 } else if input.left_pressed { -1.0 } else { 0.0 };
        let dir_y : f32 = if input.up_pressed { -1.0 } else if input.down_pressed { 1.0 } else { 0.0 };
    
        let magnitude = dir_x.abs() + dir_y.abs();
    
        let normalized_x = dir_x / magnitude;
        let normalized_y = dir_y / magnitude;
    
        if magnitude != 0.0 {
            velocity.x = normalized_x * moveable.speed;
            velocity.y = normalized_y * moveable.speed;
        }
    };

    iterate_entities!(world, [Moveable], (Velocity), expr);
}
