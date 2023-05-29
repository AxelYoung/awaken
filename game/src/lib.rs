#![allow(non_snake_case)]

use chroma::*;
use common::Cell;
use harmony::*;

use input::Input;
use map_gen::{MAP_TILE_WIDTH, MAP_TILE_HEIGHT};
use winit::{
   dpi::PhysicalSize,
   event_loop::EventLoop,
   window::{Window, WindowBuilder},
};

use winit_input_helper::WinitInputHelper;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod animation;
mod camera;
mod clones;
mod common;
mod input;
mod map_gen;
mod math;
mod render;
mod player;
mod collision;
mod movement;
mod trails;

const TICK_DURATION: u128 = 20;

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 224;
const SCREEN_SCALE: u32 = 4;

const WINDOW_WIDTH: u32 = SCREEN_WIDTH * SCREEN_SCALE;
const WINDOW_HEIGHT: u32 = SCREEN_HEIGHT * SCREEN_SCALE;

pub struct Game {
   pub world: World,
   pub chroma: Chroma,
   pub input: Input,

   delta_time: u128,
   time: u128,

   player: usize,
   pub clone_spawns: [Cell; 5],

   colliders: [[bool; MAP_TILE_HEIGHT]; MAP_TILE_WIDTH]
}

impl Game {
   pub fn new(window: &Window) -> Self {
      Self {
         player: 0,
         clone_spawns: [Cell::new(0,0); 5],
         time: 0,
         world: World::new(),
         chroma: pollster::block_on(
            Chroma::new(SCREEN_WIDTH, SCREEN_HEIGHT, &window)
         ),
         input: Input::none(),
         delta_time: 0,
         colliders: [[false; MAP_TILE_HEIGHT]; MAP_TILE_WIDTH]
      }
   }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
   cfg_if::cfg_if! {
      if #[cfg(target_arch = "wasm32")] {
         std::panic::set_hook(Box::new(console_error_panic_hook::hook));
         console_log::init_with_level(log::Level::Warn)
            .expect("Couldn't initialize logger");
      } else {
         env_logger::init();
      }
   }

   let event_loop = EventLoop::new();

   let window = WindowBuilder::new()
      .with_title("Awaken")
      .with_inner_size(PhysicalSize {
         width: WINDOW_WIDTH,
         height: WINDOW_HEIGHT,
      })
      .with_resizable(false)
      .build(&event_loop)
      .unwrap();

   #[cfg(target_arch = "wasm32")]
   {
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
   camera::update(game);
   player::update(game);
   movement::update(game);
   clones::update(game);
   trails::update(game);
}

fn fixed_tick_manager(game: &mut Game, tick_accumulator: &mut u128) {
   *tick_accumulator += game.delta_time;

   while *tick_accumulator >= TICK_DURATION {
      fixed_update(game);
      *tick_accumulator -= TICK_DURATION;
   }
}

fn fixed_update(game: &mut Game) {
}
