#![allow(non_snake_case)]

use harmony::*;
use chroma::*;

use winit::{event_loop::{EventLoop, ControlFlow}, 
            window::WindowBuilder, dpi::PhysicalSize, 
            event::VirtualKeyCode};

use winit_input_helper::WinitInputHelper;
use itertools::multizip;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

const SPRITE_SIZE: u16 = 8;

// In milliseconds
const TICK_DURATION: u128 = 20;

const ROOM_WIDTH : u8 = 16;
const ROOM_HEIGHT : u8 = 14;

const MAP_WIDTH: u8 = 1;
const MAP_HEIGHT: u8 = 2;

const MAP: [[[[u8;ROOM_WIDTH as usize];ROOM_HEIGHT as usize]; MAP_WIDTH as usize]; MAP_HEIGHT as usize] = [
    [START_HALL],
    [START_ROOM]
];

const START_ROOM: [[u8;ROOM_WIDTH as usize];ROOM_HEIGHT as usize] = [
    [0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0],
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

const START_HALL: [[u8;ROOM_WIDTH as usize];ROOM_HEIGHT as usize] = [
    [0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0]
];

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

struct Sprite {
    index: u32,
    flip_x: bool
}

impl Sprite {
    pub fn new(index: u32) -> Self {
        Self {
            index,
            flip_x: false
        }
    }
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

struct Transition {
    dir: Vec2,
    collided: bool
}

struct Animator {
    animation: Animation,
    frame_index: usize,
    time: u128,
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
    length: u128
}

struct Light {
    strength: f32,
    color: Color
}

struct Pushable {

}

struct Button {
    gate_ids: Vec<usize>,
    collided: Option<usize>
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
    pub fn new (sprite: u32, length: u128) -> Self {
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

    let mut chroma =  pollster::block_on(Chroma::new(SCREEN_WIDTH, SCREEN_HEIGHT, &window));

    chroma.update_camera(0.0, 4.0);

    // ECS
    
    let mut world = World::new();

    create_map_entities(&mut world);

    let e = world.new_entity();

    world.add_component_to_entity(e, Position::new(8.0 * SPRITE_SIZE as f32, 0.0));
    world.add_component_to_entity(e, Transition {dir: Vec2::new(0.0, 1.0), collided: false});

    let gate_1 = world.new_entity();

    world.add_component_to_entity(gate_1, Position::new(8.0 * SPRITE_SIZE as f32, -1.0 * SPRITE_SIZE as f32));
    world.add_component_to_entity(gate_1, Sprite::new(6));
    //world.add_component_to_entity(gate_1, Collider {});

    let gate_2 = world.new_entity();

    world.add_component_to_entity(gate_2, Position::new(7.0 * SPRITE_SIZE as f32, -1.0 * SPRITE_SIZE as f32));
    world.add_component_to_entity(gate_2, Sprite::new(6));
    //world.add_component_to_entity(gate_2, Collider {});

    let button = world.new_entity();

    world.add_component_to_entity(button, Position::new(4.0 * SPRITE_SIZE as f32, -5.0 * SPRITE_SIZE as f32));
    world.add_component_to_entity(button, Sprite::new(7));
    world.add_component_to_entity(button, Button { gate_ids: vec![gate_1, gate_2], collided: None});

    let push_box = world.new_entity();

    world.add_component_to_entity(push_box, Position::new(12.0 * SPRITE_SIZE as f32, -9.0 * SPRITE_SIZE as f32));
    world.add_component_to_entity(push_box, Sprite::new(9));
    world.add_component_to_entity(push_box, Velocity::new(0.0, 0.0));
    world.add_component_to_entity(push_box, Collider{});
    world.add_component_to_entity(push_box, Pushable{});

    create_player_entity(&mut world);

    let mut input = WinitInputHelper::new();

    // EVENT LOOP
    
    let mut last_tick = instant::Instant::now();
    let mut tick_accumultor: u128 = 0;

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            let current_time = instant::Instant::now();
            let delta_time = current_time.duration_since(last_tick);
            last_tick = current_time;

            update(&mut world, &input_manager(&mut input, control_flow), &delta_time.as_millis());

            fixed_tick_manager(&mut world, &mut chroma, &delta_time.as_millis(), &mut tick_accumultor);
        
            draw(&mut world, &mut chroma);
        }
    });
}

fn update(world: &mut World, input: &Input, delta_time: &u128) {
    animate(world, delta_time);
    set_moveable_dir(world, input);
}

fn fixed_tick_manager(world: &mut World, chroma: &mut Chroma, delta_time: &u128, tick_accumulator: &mut u128) {
    *tick_accumulator += delta_time;

    while *tick_accumulator >= TICK_DURATION {
        fixed_update(world, chroma);
        *tick_accumulator -= TICK_DURATION;
    }
}

fn fixed_update(world: &mut World, chroma: &mut Chroma) {
    pushables(world);
    check_collisions(world);
    buttons(world);
    transitions(world, chroma);
    move_entity(world);
    velocity_drag(world);
}

fn draw(world: &mut World, chroma: &mut Chroma){
    chroma.clear();

    draw_entity(world, chroma);
    //draw_lightmap(world, chroma);

    chroma.render();
}

fn draw_lightmap(world: &mut World, chroma: &mut Chroma) {
    iterate_entities!(world, [Position, Light],
        |position: &Position, sprite : &Light| {
        // Create the quad with circles with raycasts and such
    }
);
}

fn draw_entity(world: &mut World, chroma: &mut Chroma) {
    iterate_entities!(world, [Position, Sprite], 
        |position: &Position, sprite : &Sprite| {
            chroma.add_tile(position.x, position.y, sprite.index, sprite.flip_x);
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

fn animate(world: &mut World, delta_time: &u128) {
    iterate_entities!(world, (Animator, Sprite), 
        |animator: &mut Animator, sprite: &mut Sprite| {
            if animator.playing {
                animator.time += delta_time;
                if animator.time > animator.current_frame().length {
                    animator.time = 0;
                    animator.step();
                    sprite.index = animator.current_frame().sprite + if sprite.flip_x {2} else {0};
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
    for room_x in 0..MAP_WIDTH as usize {
        for room_y in 0..MAP_HEIGHT as usize {
            for x in 0..ROOM_WIDTH as usize {
                for y in 0..ROOM_HEIGHT as usize{
                    let sprite = Sprite::new(match MAP[room_y][room_x][y][x] {
                        0 => 5,
                        _ => 4
                    });
        
                    let position = Position::new(
                        (x as f32 * SPRITE_SIZE as f32) + (room_x as u8 * ROOM_WIDTH * SPRITE_SIZE as u8) as f32, 
                        (((ROOM_HEIGHT - 1) as f32 - y as f32) * SPRITE_SIZE as f32) - (room_y as u8 * ROOM_HEIGHT * SPRITE_SIZE as u8) as f32);
                    let e = world.new_entity();
                    world.add_component_to_entity(e, sprite);
                    world.add_component_to_entity(e, position);
                    if MAP[room_y][room_x][y][x] == 0 {
                        world.add_component_to_entity(e, Collider{});
                    }
                }
            }
        } 
    } 
}

fn create_player_entity(world: &mut World) {
    let e = world.new_entity();

    world.add_component_to_entity(e, Sprite::new(0));
    world.add_component_to_entity(e, Position::new(SCREEN_WIDTH as f32 / 2.0, (SCREEN_HEIGHT as f32 / 2.0) - (ROOM_HEIGHT as u16 * SPRITE_SIZE) as f32));
    world.add_component_to_entity(e, Velocity::new(0.0, 0.0));
    world.add_component_to_entity(e, Moveable {speed: 0.8, active: true, dir: Vec2::new(0.0, 0.0)});
    world.add_component_to_entity(e, Collider{});
    world.add_component_to_entity(e, Animator{
        animation: Animation {
            frames: vec![AnimationFrame::new(1, 100), AnimationFrame::new(0, 100)],
            r#loop: true
        },
        frame_index: 0,
        time: 0,
        playing: false
    });
    world.add_component_to_entity(e, Light{strength: 20.0, color: Color::new(60, 60, 100)});

    world.player = Some(e);
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

fn pushables(world: &mut World) {

    let mut vel : Option<(f32, f32)> = None;
    let mut pushable : Option<usize> = None;

    iterate_entities!(world, [Position, Collider], (Velocity), 
        |position_a: &Position, _, velocity: &mut Velocity| {            
            iterate_entities_with_id!(world, [Collider, Pushable, Position], 
                |id, _, _, position_b: &Position| {
                    let next_pos = Vec2::new(position_a.x + velocity.x, position_a.y+ velocity.y);
                    if check_collision(next_pos, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}, position_b.value, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}) {
                        pushable = Some(id);
                        velocity.x *= 0.5;
                        velocity.y *= 0.5;
                        vel = Some((velocity.x, velocity.y));
                    }
                }
            );
        }
    );

    if let Some(id) = pushable {
        let pos = world.get_component_from_entity_mut::<Velocity>(id).unwrap().as_mut().unwrap();
        pos.x = vel.unwrap().0;
        pos.y = vel.unwrap().1;
    }
}

fn check_collisions(world: &mut World) {
    iterate_entities!(world, [Position, Collider], (Velocity), 
        |position_a: &Position, _, velocity: &mut Velocity| {            
            iterate_entities!(world, [Position, Collider], 
                |position_b: &Position, _| {
                    if position_a != position_b {
                        let next_pos = Vec2::new(position_a.x + velocity.x, position_a.y+ velocity.y);
                        if check_collision(next_pos, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}, position_b.value, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}) {
                            velocity.x = 0.0;
                            velocity.y = 0.0;
                        }
                    }
                }
            );
        }
    );
}

fn transitions(world: &mut World, chroma: &mut Chroma) {
    let player_position = world.borrow_components::<Position>().unwrap();
    let player_position = player_position[world.player.unwrap()].as_ref();
    iterate_entities!(world, [Position], (Transition), 
        |position_b: &Position, transition: &mut Transition| {
            let player_position = player_position.unwrap().value;
            if check_collision(player_position, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}, position_b.value, Bounds{right: SPRITE_SIZE as f32, bottom: 0.1 as f32}) {
                if !transition.collided {
                    transition.collided = true;
                    if transition.dir.x == 0.0 {
                        let dir = if player_position.y + 4.0 > position_b.y { -1.0 } else { 1.0 };
                        chroma.update_camera(chroma.camera.x, chroma.camera.y - (dir * 4.0));
                        println!("{} vs {}", player_position.y, position_b.y);
                    } else {
                        let dir = if player_position.x + 4.0 > position_b.x { -1.0 } else { 1.0 };
                        chroma.update_camera(chroma.camera.x + (dir * 4.0), chroma.camera.y);
                    }
                }
            } else {
                transition.collided = false;
            }
        }
    );
}

fn buttons(world: &mut World) {

    let mut gates_to_remove: Vec<usize> = vec![];
    let mut gates_to_add: Vec<usize> = vec![];

    iterate_entities_with_id!(world, [Position], (Collider), |id, position_a: &Position, _| {
        iterate_entities!(world, [Position], (Button, Sprite), 
        |position_b: &Position, button: &mut Button, sprite: &mut Sprite| {
            if check_collision(position_a.value, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}, position_b.value, Bounds{right: SPRITE_SIZE as f32, bottom: SPRITE_SIZE as f32}) {
                if button.collided == None {
                    for gate in &button.gate_ids {
                        gates_to_remove.push(*gate);
                    }
                    sprite.index = 8;
                    println!("Button down");
                    button.collided = Some(id);
                }
            } else if button.collided == Some(id) {
                for gate in &button.gate_ids {
                    gates_to_add.push(*gate);
                }
                sprite.index = 7;
                println!("Button up");
                button.collided = None;
            }
        }
    );
    });

    for gate in gates_to_add {
        world.add_component_to_entity(gate, Sprite::new(6));
        world.add_component_to_entity(gate, Collider{});
    }

    for gate in gates_to_remove {
        world.remove_component_from_entity::<Sprite>(gate);
        world.remove_component_from_entity::<Collider>(gate);
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

            let dir_y : f32 = if input.up_pressed { 1.0 } else if input.down_pressed { -1.0 } else { 0.0 };
        
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
                animator.time = 0;
            }
        }
    );
}