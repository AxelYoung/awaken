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

const SPRITE_SIZE: u16 = 8;

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
    pub fn dist(&self, comp: Vec2) -> f32 {
        let dx = comp.x - self.x;
        let dy = comp.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
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
    index: Option<u32>,
    flip_x: bool
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

struct Animator {
    animation: Animation,
    frame_index: usize,
    time: f64,
    playing: bool,
}

impl Animator {
    pub fn current_frame(&self) -> &AnimationFrame {
        &self.animation.frames[self.frame_index]
    }

    pub fn step(&mut self) {
        self.frame_index += 1;
        if self.frame_index == self.animation.frames.len() {
            self.frame_index = 0;
        }
    }
}

struct Animation {
    frames: Vec<AnimationFrame>,
    r#loop: bool
}

struct AnimationFrame {
    sprite: u32,
    length: f64
}

struct Light {
    strength: f32,
    color: Color
}

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
}

impl AnimationFrame {
    pub fn new (sprite: u32, length: f64) -> Self {
        Self {
            sprite,
            length
        }
    }
}

const SCREEN_WIDTH: u32 = 128;
const SCREEN_HEIGHT: u32 = 112;

const SCREEN_SCALE: u32 = 8;

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

    let e = world.new_entity();

    world.add_component_to_entity(e, Position::new(80.0, 80.0));
    world.add_component_to_entity(e, Sprite{name: "torch", index: None, flip_x: false});
    world.add_component_to_entity(e, Light{strength: 30.0, color: Color::new(181, 98, 34)});

    let e = world.new_entity();

    world.add_component_to_entity(e, Position::new(40.0, 40.0));
    world.add_component_to_entity(e, Sprite{name: "torch", index: None, flip_x: false});
    world.add_component_to_entity(e, Light{strength: 12.0, color: Color::new(100, 200, 100)});

    create_player_entity(&mut world);

    let mut input = WinitInputHelper::new();

    // EVENT LOOP
    
    let mut last_tick = instant::now();
    let mut tick_accumultor = 0.0;

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            let delta_time = instant::now() - last_tick;
            last_tick = instant::now();

            update(&mut world, &input_manager(&mut input, control_flow), &delta_time);

            fixed_tick_manager(&mut world, &delta_time, &mut tick_accumultor);
        
            draw(&mut world, &mut chroma);
        }
    });
}

fn update(world: &mut World, input: &Input, delta_time: &f64) {
    animate(world, delta_time);
    set_moveable_dir(world, input);
}

fn fixed_tick_manager(world: &mut World, delta_time: &f64, tick_accumulator: &mut f64) {
    *tick_accumulator += delta_time;

    while *tick_accumulator >= TICK_DURATION as f64 {
        fixed_update(world);
        *tick_accumulator -= TICK_DURATION as f64;
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
    draw_lightmap(world, chroma);

    chroma.render();
}

fn draw_entity(world: &mut World, chroma: &mut Chroma) {
    iterate_entities!(world, [Sprite, Position], 
        |sprite : &Sprite, position: &Position| {
            let sprite_data = asset_loader::get_sprite(sprite.name);

            let x = position.x as u32;
            let y = position.y as u32;
        
            if let Some(index) = sprite.index {
                chroma.draw_sprite_from_sheet(sprite_data, index, x, y, sprite.flip_x);
            } else {
                chroma.draw_sprite(sprite_data, x, y, sprite.flip_x);
            }
        }
    );
}

const DARKNESS: i16 = 100;

fn draw_lightmap(world: &mut World, chroma: &mut Chroma) {
    
    let mut lights : Vec<(f32, Color, Vec2)> = Vec::new();
    
    
    iterate_entities!(world, [Light, Position], 
        |light: &Light, position: &Position| {
            lights.push((light.strength, light.color, position.value));
        }
    );
    
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let mut r = -DARKNESS as f32;
            let mut g = -DARKNESS as f32;
            let mut b = -DARKNESS as f32;

            for (light, color, position) in lights.iter() {
                let dist = position.dist(Vec2::new(x as f32 - 3.0, y as f32 - 3.0));
                let min_dist = *light + rand::thread_rng().gen_range(-1.0..=2.0);
                if dist <= min_dist { 
                    let falloff = 1.0 - (dist / min_dist);
                    r = r + (color.r as f32 * falloff);
                    g = g + (color.g as f32 * falloff);
                    b = b + (color.b as f32 * falloff);
                }
            }

            let mut pixel = chroma.get_pixel(x, y);
            pixel[0] = (pixel[0] as f32 + r) as u8;
            pixel[1] = (pixel[1] as f32 + g) as u8;
            pixel[2] = (pixel[2] as f32 + b) as u8;
            chroma.draw_pixel(&pixel, x, y);
        }
    }
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

fn animate(world: &mut World, delta_time: &f64) {
    iterate_entities!(world, (Animator, Sprite), 
        |animator: &mut Animator, sprite: &mut Sprite| {
            if animator.playing {
                animator.time += delta_time;
                if animator.time > animator.current_frame().length {
                    animator.time = 0.0;
                    animator.step();
                    sprite.index = Some(animator.current_frame().sprite);
                }
            }
        }
    );
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
                    0 => "stone",
                    _ => "cobble"
                },
                index: None,
                flip_x: false
            };

            let position = Position::new(x as f32 * SPRITE_SIZE as f32, y as f32 * SPRITE_SIZE as f32);
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

    world.add_component_to_entity(e, Sprite{name: "sentinel", index: Some(1), flip_x: false});
    world.add_component_to_entity(e, Position::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0));
    world.add_component_to_entity(e, Velocity::new(0.0, 0.0));
    world.add_component_to_entity(e, Moveable {speed: 0.6, active: true, dir: Vec2::new(0.0, 0.0)});
    world.add_component_to_entity(e, Collider{});
    world.add_component_to_entity(e, Animator{
        animation: Animation {
            frames: vec![AnimationFrame::new(1, 100.0), AnimationFrame::new(0, 100.0)],
            r#loop: true
        },
        frame_index: 0,
        time: 0.0,
        playing: false
    });
    world.add_component_to_entity(e, Light{strength: 20.0, color: Color::new(60, 60, 100)});
}

fn move_entity(world: &mut World) {
    iterate_entities!(world, [Velocity], (Position), 
        |velocity: &Velocity, position: &mut Position| {
            position.x += velocity.x;
            position.y += velocity.y;
    });
}

fn velocity_drag(world: &mut World) {
    iterate_entities!(world, (Velocity), 
        |velocity: &mut Velocity| {
            velocity.x -= velocity.x * 0.2;
            velocity.y -= velocity.y * 0.2;
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
                        if check_collision(position_a.value, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}, position_b.value, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}) {
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
    iterate_entities!(world, (Moveable, Velocity, Animator, Sprite),
        |moveable: &mut Moveable, velocity: &mut Velocity, animator: &mut Animator, sprite: &mut Sprite| {
            let dir_x : f32 = 
            if input.right_pressed {
                sprite.flip_x = false;
                 1.0 
            } else if input.left_pressed {
                sprite.flip_x = true;
                 -1.0 
            } else { 0.0 };

            let dir_y : f32 = if input.up_pressed { -1.0 } else if input.down_pressed { 1.0 } else { 0.0 };
        
            let magnitude = dir_x.abs() + dir_y.abs();
        
            let normalized_x = dir_x / magnitude;
            let normalized_y = dir_y / magnitude;
            
            moveable.dir.x = normalized_x;
            moveable.dir.y = normalized_y;

            if magnitude != 0.0 {
                velocity.x = normalized_x * moveable.speed;
                velocity.y = normalized_y * moveable.speed;
                animator.playing = true;
            } else {
                animator.playing = false;
                animator.time = 0.0;
            }
        }
    );
}