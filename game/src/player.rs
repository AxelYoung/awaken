use harmony::*;

use super::common::Position;

use super::movement::Moveable;

use super::render::Sprite;

use super::Game;

use super::common::Cell;
use super::math::Vec2i;
use super::collision::check_collision;

pub struct Player {
    pub moved: bool
}

pub fn create(game: &mut Game) {
    let player = game.world.new_entity();
    let cell = Cell::new(24, 6);
    game.world.add_component_to_entity(player, cell);
    game.world.add_component_to_entity(player, cell.to_position());
    game.world.add_component_to_entity(player, Sprite::new(0, 0, 100));
    game.world.add_component_to_entity(player, Player {moved: false});
    game.world.add_component_to_entity(player, Moveable {
        start_cell: Cell::new(0, 0),
        end_cell: Cell::new(0, 0),
        duration: 150,
        accumulator: 0,
        moving: false
    });

    game.player = player;
}

pub fn update(game: &mut Game) {
    set_movement_dir(game);
}

fn set_movement_dir(game: &mut Game) {
    let dir = game.input.dir();

    let mut set_moveable = false;

    iterate_entities_with_id!(game.world, [Moveable], (Player, Cell, Position, Sprite),
    |id, moveable: &Moveable, player: &mut Player, cell: &mut Cell, position: &mut Position, sprite: &mut Sprite| {
        if !moveable.moving {
            if dir != Vec2i::zero() {
                let goal_cell = Cell::new(cell.x + dir.x, cell.y + dir.y);
                if !check_collision(game, goal_cell) {
                    set_moveable = true;

                    player.moved = true;

                    update_sprite_dir(dir, sprite);
                }
            }
        }
    });

    if set_moveable {

        let mut cell = Cell::new(0, 0);
        let mut goal_cell = Cell::new(0, 0);

        {
            cell = *game.world.get_component_from_entity_mut::<Cell>(game.player).unwrap().as_mut().unwrap();

            goal_cell = Cell::new(cell.x + dir.x, cell.y + dir.y);
        }

        let moveable = game.world.get_component_from_entity_mut::<Moveable>(game.player).unwrap().as_mut().unwrap();

        moveable.start_cell = cell;
        moveable.end_cell = goal_cell;
        moveable.moving = true;
    }
}

fn update_sprite_dir(dir: Vec2i, sprite: &mut Sprite) {
    sprite.index_x = match dir {
        Vec2i {x: 0, y: -1 } => { 0 },
        Vec2i {x: 0, y: 1 } => { 1 },
        Vec2i {x: 1, y: 0 } => { 2 },
        Vec2i {x: -1, y: 0 } => { 3 },
        _ =>  {0}
    }
}