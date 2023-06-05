use harmony::iterate_entities;

use crate::common::Position;
use crate::movement::Moveable;
use crate::{Game, common::Cell};
use crate::player::Player;
use crate::clones::{Clone, switch_to_clone};

pub struct WinTile {
  pub color: u8,
  pub level: usize
}

pub fn update(game: &mut Game) {
  check_win(game);
}

fn check_win(game: &mut Game) {
  let mut win = true;

  let mut win_cells = vec![];
  let mut player_cell = (Cell::new(0, 0), 0);

  let mut clone_cells = vec![]; 

  iterate_entities!(game.world, [WinTile, Cell], 
  |win: &WinTile, cell: &Cell| {
    if win.level == game.current_room {
      win_cells.push((cell.clone(), win.color));
    }
  });

  iterate_entities!(game.world, [Player, Cell],
  |player: &Player, cell: &Cell| {
    player_cell = (cell.clone(), player.color);
  });

  iterate_entities!(game.world, [Clone, Cell],
    |clone: &Clone, cell: &Cell| {
      clone_cells.push((cell.clone(), clone.color));
  });

  'win_tile: for win_tile in win_cells {
    for clone_cell in clone_cells.iter() {
      if win_tile.1 == clone_cell.1 {
        if win_tile.0.value != clone_cell.0.value {
          win = false;
        } 
        continue 'win_tile;
      }
    }
    if player_cell.1 == win_tile.1 {
      if win_tile.0 != player_cell.0 {
        win = false;
      }
      continue 'win_tile;
    }
    win = false;
  }

  if win || game.input.skip_level {
    game.current_room += 1;
    if game.current_room < 6 {
      iterate_entities!(game.world, (Player, Position, Cell, Moveable), 
      |player: &mut Player, position: &mut Position, cell: &mut Cell, moveable: &mut Moveable| {
  
        player.playback_dir = vec![];
  
        moveable.moving = false;
  
        let new_cell = game.clone_spawns[game.current_room][player.color as usize];
  
        cell.value = new_cell.value;
        position.value = new_cell.to_position().value;
      });
  
      iterate_entities!(game.world, (Clone, Position, Cell, Moveable), 
      |clone: &mut Clone, position: &mut Position, cell: &mut Cell, moveable: &mut Moveable| {
  
        clone.paths = vec![];
        clone.playback_dir = vec![];
  
        moveable.moving = false;
  
        let new_cell = game.clone_spawns[game.current_room][clone.color as usize];
  
        cell.value = new_cell.value;
        position.value = new_cell.to_position().value;
      });
  
      switch_to_clone(game, 0);
      }
    }
}