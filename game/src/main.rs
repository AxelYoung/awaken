use chroma::Chroma;
use winit::{event_loop::{EventLoop, ControlFlow}, 
            window::WindowBuilder, dpi::PhysicalSize, 
            event::{Event, WindowEvent, ElementState, VirtualKeyCode, KeyboardInput}};

#[derive(Default)]
struct EntityComponents {
    sprite_component: Option<SpriteComponent>,
    position_component: Option<PositionComponent>,
    velocity_component: Option<VelocityComponent>
}

struct SpriteComponent {
    name: String,
}

struct PositionComponent {
    x: f32,
    y: f32
}

struct VelocityComponent {
    x: f32,
    y: f32
}

struct World {
    chroma: Chroma,

    sprite_components: Vec<Option<SpriteComponent>>,
    position_components: Vec<Option<PositionComponent>>,
    velocity_components: Vec<Option<VelocityComponent>>
}

impl World {
    pub fn new(chroma: Chroma) -> Self {
        Self {
            chroma,
            sprite_components: Vec::new(),
            position_components: Vec::new(),
            velocity_components: Vec::new()
        }
    }

    pub fn add_entity(&mut self, components: EntityComponents) {
        self.sprite_components.push(components.sprite_component);
        self.position_components.push(components.position_component);
        self.velocity_components.push(components.velocity_component);
    }

    pub fn velocity(&mut self) {
        let zip = self.velocity_components.iter().zip(self.position_components.iter());

        let renderable = zip.filter_map(|(velocity, position): (&Option<VelocityComponent>, &Option<PositionComponent>)| { 
            Some((velocity.as_ref()?, position.as_ref()?))
        });

        for (velocity, position) in renderable {
            //position.x += velocity.x;
            //position.y += velocity.y;
        }
    }

    pub fn render(&mut self) {
        let zip = self.sprite_components.iter().zip(self.position_components.iter());

        let renderable = zip.filter_map(|(sprite, position): (&Option<SpriteComponent>, &Option<PositionComponent>)| { 
            Some((sprite.as_ref()?, position.as_ref()?))
        });

        for (sprite, position) in renderable {
            self.chroma.draw_sprite(
                asset_loader::get_sprite(&sprite.name), 
                position.x as u32, 
                position.y as u32
            );
        }
        
        self.chroma.render();
    }
}

fn main() {
    run();
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

    let mut world = World::new(
        Chroma::new(32, 32, &window)
    );

    world.add_entity(
        EntityComponents { 
            sprite_component: 
                Some(SpriteComponent { 
                    name: "stone".to_string() 
                }), 
            position_component: 
                Some(PositionComponent {
                     x: 13.0, 
                     y: 9.0 
                }) ,
            ..Default::default()
         }
    );

    world.add_entity(
        EntityComponents { 
            sprite_component: 
                Some(SpriteComponent { 
                    name: "grass".to_string() 
                }), 
            position_component: 
                Some(PositionComponent {
                     x: 4.0, 
                     y: 10.0 
                }),
            ..Default::default()
         }
    );

    world.add_entity(
        EntityComponents { 
            sprite_component: 
                Some(SpriteComponent { 
                    name: "sentinel".to_string() 
                }), 
            position_component: 
                Some(PositionComponent {
                     x: 1.0, 
                     y: 5.0 
                }),
            velocity_component:
                Some(VelocityComponent { 
                    x: 0.1, 
                    y: 0.1 
                })
         }
    );

    world.render();

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