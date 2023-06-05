use winit::{event_loop::ControlFlow, event::VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

use mathrix::vec::Vec2i;

pub struct Input {
  pub up_pressed: bool,
  pub down_pressed: bool,
  pub left_pressed: bool,
  pub right_pressed: bool,
  pub one_pressed: bool,
  pub two_pressed: bool,
  pub three_pressed: bool,
  pub four_pressed: bool,
  pub five_pressed: bool,
  pub skip_pressed: bool,
  pub skip_level: bool
}

impl Input {
  pub fn new (input: &mut WinitInputHelper, control_flow: &mut ControlFlow) -> Self {
    let up_pressed = input.key_held(VirtualKeyCode::W);
    let down_pressed = input.key_held(VirtualKeyCode::S);
    let left_pressed = input.key_held(VirtualKeyCode::A);
    let right_pressed = input.key_held(VirtualKeyCode::D);

    let one_pressed = input.key_pressed(VirtualKeyCode::Key1);
    let two_pressed = input.key_pressed(VirtualKeyCode::Key2);
    let three_pressed = input.key_pressed(VirtualKeyCode::Key3);
    let four_pressed = input.key_pressed(VirtualKeyCode::Key4);
    let five_pressed = input.key_pressed(VirtualKeyCode::Key5);

    let skip_pressed = input.key_pressed(VirtualKeyCode::Space);

    let skip_level = input.key_pressed(VirtualKeyCode::Back);
  
    if input.key_released(VirtualKeyCode::Escape) 
    || input.close_requested()
    || input.destroyed() {
      *control_flow = ControlFlow::Exit;
    }
  
    Self {
      up_pressed,
      down_pressed,
      left_pressed,
      right_pressed,
      one_pressed,
      two_pressed,
      three_pressed,
      four_pressed,
      five_pressed,
      skip_pressed,
      skip_level
    }
  }

  pub fn direction (&self) -> Vec2i {
    if self.up_pressed { Vec2i::new(0, 1) } 
    else if self.down_pressed { Vec2i::new(0, -1) }
    else if self.right_pressed { Vec2i::new(1, 0) }
    else if self.left_pressed { Vec2i::new(-1, 0) }
    else { Vec2i::zero() }
  }

  pub fn loop_num(&self) -> Option<usize> {
    if self.one_pressed { Some(0) } 
    else if self.two_pressed { Some(1) }
    else if self.three_pressed { Some(2) }
    else if self.four_pressed { Some(3) }
    else if self.five_pressed { Some(4) }
    else { None }
  }

  pub fn none() -> Self {
    Self {
      up_pressed: false,
      down_pressed: false,
      left_pressed: false,
      right_pressed: false,
      one_pressed: false,
      two_pressed: false,
      three_pressed: false,
      four_pressed: false,
      five_pressed: false,
      skip_pressed: false,
      skip_level: false
    }
  }
}
