use harmony::*;
use crate::Game;
use super::render::Sprite;

pub struct Animator {
    pub animation: Animation,
    pub frame_index: usize,
    pub time: u128,
    pub playing: bool,
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

pub struct Animation {
    pub frames: Vec<AnimationFrame>,
    pub r#loop: bool
}

pub struct AnimationFrame {
    sprite: u32,
    length: u128
}

impl AnimationFrame {
    pub fn new (sprite: u32, length: u128) -> Self {
        Self {
            sprite,
            length
        }
    }
}

pub fn update(game: &mut Game) {
    animate_entities(game);
}

fn animate_entities(game: &mut Game) {
    iterate_entities!(game.world, (Animator, Sprite), 
    |animator: &mut Animator, sprite: &mut Sprite| {
        if animator.playing {
            animator.time += game.delta_time;
            if animator.time > animator.current_frame().length {
                animator.time = 0;
                animator.step();
                sprite.index = animator.current_frame().sprite;
            }
        }
    });
}
