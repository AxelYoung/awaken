use winit::{event_loop::ControlFlow, event::VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

pub struct Input {
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub loop_pressed: bool
}

impl Input {
    pub fn new (input: &mut WinitInputHelper, control_flow: &mut ControlFlow) -> Self {
        let up_pressed = input.key_held(VirtualKeyCode::Up);
        let down_pressed = input.key_held(VirtualKeyCode::Down);
        let left_pressed = input.key_held(VirtualKeyCode::Left);
        let right_pressed = input.key_held(VirtualKeyCode::Right);
        let loop_pressed = input.key_pressed(VirtualKeyCode::Space);
    
        if input.key_released(VirtualKeyCode::Escape) || input.close_requested() || input.destroyed() {
            *control_flow = ControlFlow::Exit;
        }
    
        Self {
            up_pressed,
            down_pressed,
            left_pressed,
            right_pressed,
            loop_pressed
        }
    }
}
