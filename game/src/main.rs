#![allow(non_snake_case)]

use harmony::*;
use chroma::Chroma;
use rand::Rng;
use winit::{event_loop::{EventLoop, ControlFlow}, 
            window::WindowBuilder, dpi::PhysicalSize, 
            event::VirtualKeyCode};

use winit_input_helper::WinitInputHelper;
use itertools::multizip;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

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
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
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

#[derive(PartialEq, Clone, Copy, Debug)]
struct Vec2 {
    x: f32,
    y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    
    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

struct Sprite<'a> {
    name: &'a str,
    index: Option<u32>
}

#[derive(PartialEq, Debug)]
struct Position {
   value: Vec2
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self { Self { value: Vec2::new(x, y) } }
}

impl std::ops::Deref for Position {
    type Target = Vec2;
    fn deref(&self) -> &Vec2 { &self.value }
}

impl std::ops::DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Vec2 { &mut self.value }
}

struct Velocity {
    value: Vec2
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self { Self { value: Vec2::new(x, y) } }
}

impl std::ops::Deref for Velocity {
    type Target = Vec2;
    fn deref(&self) -> &Vec2 { &self.value }
}

impl std::ops::DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Vec2 { &mut self.value }
}

struct Collider{}
struct Moveable {
    speed: f32,
    active: bool,
    dir: Vec2
}

struct Bounds {
    right: f32,
    bottom: f32,
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
        window.set_inner_size(PhysicalSize::new(SCREEN_SIZE.x, SCREEN_SIZE.y));
        
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
    world.add_component_to_entity(e, Position::new(20.0, 50.0));
    world.add_component_to_entity(e, Velocity::new(4.5, 0.0));
    world.add_component_to_entity(e, Collider{});

    let e = world.new_entity();
    
    world.add_component_to_entity(e, Sprite{ name: "stone", index: None });
    world.add_component_to_entity(e, Position::new(200.0, 50.0));
    world.add_component_to_entity(e, Velocity::new(-4.5, 0.0));
    world.add_component_to_entity(e, Collider{});

    let e = world.new_entity();
    
    world.add_component_to_entity(e, Sprite{ name: "stone", index: None });
    world.add_component_to_entity(e, Position::new(100.0, 80.0));
    world.add_component_to_entity(e, Velocity::new(10.5, 2.0));
    world.add_component_to_entity(e, Collider{});

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
    set_moveable_dir(world, input);
    turn(world);
    //check_moveable(world);
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
    check_collisions(world);
    move_entity(world);
    velocity_drag(world);
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

            let position = Position::new(x as f32 * 16.0, y as f32 * 16.0);
            let e = world.new_entity();
            world.add_component_to_entity(e, sprite);
            world.add_component_to_entity(e, position);
            if MAP[y][x] == 0 {
                world.add_component_to_entity(e, Collider{});
            }
        }
    }
}

fn create_player_entity(world: &mut World) {
    let e = world.new_entity();

    world.add_component_to_entity(e, Sprite{name: "sentinel", index: Some(1)});
    world.add_component_to_entity(e, Position::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0));
    world.add_component_to_entity(e, Velocity::new(0.0, 0.0));
    world.add_component_to_entity(e, Moveable {speed: 1.0, active: true, dir: Vec2::new(0.0, 0.0)});
    world.add_component_to_entity(e, Collider{});
}

fn move_entity(world: &mut World) {
    iterate_entities!(world, [Velocity], (Position), 
        |velocity: &Velocity, position: &mut Position| {
            position.x += velocity.x;
            position.y += velocity.y;
        
            position.x = position.x.clamp(0.0, SCREEN_WIDTH as f32 - 16.0);
            position.y = position.y.clamp(0.0, SCREEN_HEIGHT as f32 - 16.0);
    });
}

fn turn(world: &mut World) {
    iterate_entities!(world, [Velocity, Moveable], (Sprite), 
        |velocity: &Velocity, _, sprite: &mut Sprite| {
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
        }
    );
}


fn velocity_drag(world: &mut World) {
    iterate_entities!(world, (Velocity), 
        |velocity: &mut Velocity| {
            velocity.x -= velocity.x * 0.05;
            velocity.y -= velocity.y * 0.05;
        }
    );
}

fn check_collisions(world: &mut World) {
    let mut collided : Vec<usize> = vec![];
    let mut collided_velocities : Vec<(f32, f32)> = vec![];

    iterate_entities_with_id!(world, [Position, Collider], (Velocity), 
        |id, position_a: &Position, _, velocity: &mut Velocity| {            
            iterate_entities!(world, [Position, Collider], 
                |position_b: &Position, _| {
                    if position_a != position_b {
                        if check_collision(position_a.value, Bounds{right: 16.0, bottom: 16.0}, position_b.value, Bounds{right: 16.0, bottom: 16.0}) {
                            collided_velocities.push((velocity.x / 2.0, velocity.y / 2.0));
                            velocity.x = 0.0;
                            velocity.y = 0.0;
                            collided.push(id);
                        }
                    }
                }
            );
        }
    );

    for (id, velocity) in collided.iter().zip(collided_velocities) {
        if let Some(Some(position)) = world.get_component_from_entity::<Position>(*id) {
            position.x -= velocity.0 * 2.0;
            position.y -= velocity.1 * 2.0;
        }
    }
}

fn check_collision(pos_a: Vec2, bounds_a: Bounds, pos_b: Vec2, bounds_b: Bounds) -> bool {
    let right_a = pos_a.x + bounds_a.right;
    let bot_a = pos_a.y + bounds_a.bottom;
    let right_b = pos_b.x + bounds_b.right;
    let bot_b = pos_b.y + bounds_b.bottom;

    if pos_a.x < right_b && right_a > pos_b.x && pos_a.y < bot_b && bot_a > pos_b.y {
        return true;
    }

    false
}

fn set_moveable_dir(world: &mut World, input: &Input) {
    iterate_entities!(world, (Moveable, Velocity),
        |moveable: &mut Moveable, velocity: &mut Velocity| {
            let dir_x : f32 = if input.right_pressed { 1.0 } else if input.left_pressed { -1.0 } else { 0.0 };
            let dir_y : f32 = if input.up_pressed { -1.0 } else if input.down_pressed { 1.0 } else { 0.0 };
        
            let magnitude = dir_x.abs() + dir_y.abs();
        
            let normalized_x = dir_x / magnitude;
            let normalized_y = dir_y / magnitude;
            
            moveable.dir.x = normalized_x;
            moveable.dir.y = normalized_y;

            if magnitude != 0.0 {
                velocity.x = normalized_x * moveable.speed;
                velocity.y = normalized_y * moveable.speed;
            }
        }
    );
}

fn check_moveable(world: &mut World) {
    iterate_entities_with_id!(world, [Position], (Moveable), 
        |moveable_id, moveable_pos: &Position, moveable: &mut Moveable| {
            iterate_entities_with_id!(world, [Collider, Position], 
                |id, _, pos: &Position| {
                    if moveable_id != id {
                        if check_collision((moveable_pos.value + Vec2::new(8.0, 8.0)) + (moveable.dir * 10.0), Bounds { right: 0.0, bottom: 0.0 }, pos.value, Bounds { right: 16.0, bottom: 16.0 }) {
                            dbg!(pos);
                        }
                    } 
                }
            );
        }
    );
}