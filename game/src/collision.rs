use super::Game;
use super::common::Cell;

pub fn check_collision(game: &Game, cell: Cell) -> bool {
    game.colliders[cell.x as usize][cell.y as usize]
}