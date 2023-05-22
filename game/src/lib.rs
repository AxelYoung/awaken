#![allow(non_snake_case)]

use std::vec;

use harmony::*;
use chroma::*;

use input::Input;
use winit::{event_loop::EventLoop, 
            window::{WindowBuilder, Window}, dpi::PhysicalSize};

use winit_input_helper::WinitInputHelper;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

mod animation;
mod buttons;
mod common;
mod input;
mod looping;
mod map_gen;
mod physics;
mod player;
mod math;
mod pushables;
mod render;
mod transitions;

const TICK_DURATION: u128 = 20;
const SCREEN_WIDTH: u32 = 128;
const SCREEN_HEIGHT: u32 = 112;

const SCREEN_SCALE: u32 = 8;

const WINDOW_WIDTH: u32 = SCREEN_WIDTH * SCREEN_SCALE;
const WINDOW_HEIGHT: u32 = SCREEN_HEIGHT * SCREEN_SCALE;

pub struct Game{ 
    player: usize,
    timer: usize,
    clones: [usize; 5],
    clone_commands: [Vec<(math::Vec2, u128)>; 5],
    current_clone: usize,
    time: u128,
    clone_count: usize,
    pub world: World,
    pub chroma: Chroma,
    pub input: Input,
    delta_time: u128
}

impl Game {
    pub fn new(window: &Window) -> Self {
        Self {
            player: 0, 
            timer: 0, 
            clones: [0;5], 
            clone_commands: [vec![], vec![], vec![], vec![], vec![]], 
            current_clone: 0, 
            time: 0, 
            clone_count: 0, 
            world: World::new(),
            chroma: pollster::block_on(Chroma::new(SCREEN_WIDTH, SCREEN_HEIGHT, &window)),
            input: Input::none(),
            delta_time: 0
        }
    }
}

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

    let mut game = Game::new(&window);

    map_gen::create(&mut game);

    looping::create_ui(&mut game);

    player::create(&mut game);

    let mut winit = WinitInputHelper::new();
    
    let mut last_tick = instant::Instant::now();
    let mut tick_accumultor: u128 = 0;

    event_loop.run(move |event, _, control_flow| {
        if winit.update(&event) {
            let current_time = instant::Instant::now();
            game.delta_time = current_time.duration_since(last_tick).as_millis();
            last_tick = current_time;

            game.time += game.delta_time;
            game.input = input::Input::new(&mut winit, control_flow);

            update(&mut game);

            fixed_tick_manager(&mut game, &mut tick_accumultor);
        
            render::draw(&mut game);
        }
    });
}

fn update(game: &mut Game) {
    animation::update(game);
    player::update(game);
    looping::update(game);
}

fn fixed_tick_manager(game: &mut Game, tick_accumulator: &mut u128) {
    *tick_accumulator += game.delta_time;

    while *tick_accumulator >= TICK_DURATION {
        fixed_update(game);
        *tick_accumulator -= TICK_DURATION;
    }
}

fn fixed_update(game: &mut Game) {
    pushables::fixed_update(game);
    buttons::fixed_update(game);
    transitions::fixed_update(game);
    physics::fixed_update(game);
}