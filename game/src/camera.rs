use crate::map_gen::{ROOM_PIXEL_WIDTH, ROOM_PIXEL_HEIGHT};

use super::Game;
use super::common::Position;

pub fn update(game: &mut Game) {
    let player_postion = game.world.get_component_from_entity_mut::<Position>(game.player).unwrap().as_mut().unwrap().value;    
    let camera_x = (player_postion.x / ROOM_PIXEL_WIDTH as f32).floor() * -4.0;
    let camera_y = (player_postion.y / ROOM_PIXEL_HEIGHT as f32).floor() * -4.0;
    game.chroma.update_camera(camera_x, camera_y);
}