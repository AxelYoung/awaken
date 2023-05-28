use winit::{event_loop::ControlFlow, event::VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

use super::math::Vec2i;

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

    pub fn dir (&self) -> Vec2i {
        if self.up_pressed { Vec2i::new(0, 1) } 
        else if self.down_pressed { Vec2i::new(0, -1) }
        else if self.right_pressed { Vec2i::new(1, 0) }
        else if self.left_pressed { Vec2i::new(-1, 0) }
        else { Vec2i::zero() }
    }

    pub fn none() -> Self {
        Self {
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
            loop_pressed: false
        }
    }
}
