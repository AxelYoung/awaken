use itertools::iproduct;

use mathrix::{lst, lst::Lst};
use mathrix::vec::Vec2i;

use crate::Game;
use crate::animation::{Animator, Animation, AnimationFrame};
use crate::boxes::PushBox;
use crate::buttons::SlaveButton;
use crate::collision::Collider;
use crate::movement::Moveable;
use crate::win::WinTile;

use super::buttons::{Button, ButtonType};
use super::common::Cell;
use super::render::{Sprite, SPRITE_SIZE, SPRITE_CENTER};

use super::boxes::BoxType;

pub const ROOM_TILE_WIDTH : u32 = 16;
pub const ROOM_TILE_HEIGHT : u32 = 14;

pub const ROOM_PIXEL_WIDTH : u32 = ROOM_TILE_WIDTH * SPRITE_SIZE;
pub const ROOM_PIXEL_HEIGHT : u32 = ROOM_TILE_HEIGHT * SPRITE_SIZE;

pub const MAP_ROOM_WIDTH: u8 = 3;
pub const MAP_ROOM_HEIGHT: u8 = 4;

pub const MAP_TILE_WIDTH: usize = 
  MAP_ROOM_WIDTH as usize * ROOM_TILE_WIDTH as usize;
pub const MAP_TILE_HEIGHT: usize = 
  MAP_ROOM_HEIGHT as usize * ROOM_TILE_HEIGHT as usize;

const MAP: &[&[&[&[Tile]]]] = &[
  &[EMPTY_ROOM, LVL_3_START, LVL_5],
  &[EMPTY_ROOM, LVL_3_ROOM_2, LVL_5_TOP],
  &[EMPTY_ROOM, LVL_3_ROOM_3, LVL_4],
  &[EMPTY_ROOM_WIN, LVL_1, LVL_2]
];

#[derive(PartialEq)]
enum Tile<'a> {
  SW,
  SF,
  PL(usize, usize),
  BT(ButtonType, &'a [Cell], &'a[Slave], &'a[&'a[WireTile]]),
  BX(BoxType),
  EM,
  WT(u8, usize),
  SPRITE(Sprite),
  SHEET(&'a[&'a[Tile<'a>]])
}

#[derive(PartialEq)]
struct Slave {
  pub button_type: ButtonType,
  pub cell: Cell
}

use Tile::*;

const B0: Tile = BX(BoxType::Color(0));
const B1: Tile = BX(BoxType::Color(1));
const B2: Tile = BX(BoxType::Color(2));
const B3: Tile = BX(BoxType::Color(3));
const B4: Tile = BX(BoxType::Color(4));
const BA: Tile = BX(BoxType::Any);

const BB : Tile = BT(ButtonType::Color(0), 
  &[
    Cell { 
      value: Vec2i {
        x: 7, 
        y: 0
      } 
    },
    Cell { 
      value: Vec2i {
        x: 8, 
        y: 0
      } 
    }
  ],
  &[
    Slave { 
      button_type: ButtonType::Color(1), 
      cell: Cell { 
        value: { 
          Vec2i { x: 4, y: 4}
        }
      }
    },
    Slave { 
      button_type: ButtonType::AnyColor, 
      cell: Cell { 
        value: { 
          Vec2i { x: 10, y: 4}
        }
      }
    },
    Slave { 
      button_type: ButtonType::Color(4), 
      cell: Cell { 
        value: { 
          Vec2i { x: 13, y: 4}
        }
      }
    }
  ],
  LVL_3_START_WIRE
);

const EMPTY_ROOM: &[&[Tile]] = &[
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM]
];

const PN2 : Tile = PL(0, 2);
const PG2 : Tile = PL(1, 2);
const PY2 : Tile = PL(2, 2);
const PR2 : Tile = PL(3, 2);
const PP2 : Tile = PL(4, 2);

const W02: Tile = WT(0, 2);
const W12: Tile = WT(1, 2);
const W22: Tile = WT(2, 2);
const W32: Tile = WT(3, 2);
const W42: Tile = WT(4, 2);

const LVL_3_START: &[&[Tile]] = &[
  &[SW,SW,SW,SW,SW,SW,SW,SF,SF,SW,SW,SW,SW,SW,SW,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,BB,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,PN2,SF,SF,PG2,SF,SF,PY2,SF,SF,PR2,SF,SF,PP2,SF,SW],
  &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW]
];

const LVL_3_START_WIRE: &[&[WireTile]] = &[
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,VR,ET,ET,VR,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,VR,ET,ET,VR,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,VR,ET,ET,VR,ET,ET,ET,ET,ET,ET],
  &[ET,SR,HR,HR,HR,HR,TU,HR,HR,TU,HR,HR,HR,SL,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET]
];

const LVL_3_ROOM_2: &[&[Tile]] = &[
  &[SW,SW,SW,SW,SW,SW,SW,SF,SF,SW,SW,SW,SW,SW,SW,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,BA,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SW,SW,SW,SW,SW,SW,SF,SF,SW,SW,SW,SW,SW,SW,SW]
];

const LVL_3_ROOM_3: &[&[Tile]] = &[
  &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,W22,SF,SF,W32,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SW,SW,SW,SW,SW,SW,SF,SF,SW,SW,SW,SW,SW,SW,SW]
];


const W00: Tile = WT(0, 0);
const W10: Tile = WT(1, 0);
const W20: Tile = WT(2, 0);
const W30: Tile = WT(3, 0);
const W40: Tile = WT(4, 0);

const PN0 : Tile = PL(0, 0);
const PG0 : Tile = PL(1, 0);
const PY0 : Tile = PL(2, 0);
const PR0 : Tile = PL(3, 0);
const PP0 : Tile = PL(4, 0);

const LVL_1: &[&[Tile]] = &[
  &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW],
  &[SW,W00,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,W20,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,W40,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,W10,SF,SF,SF,SW,SW,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SW,SW,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,W30,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,PN0,SF,SF,PG0,SF,SF,PY0,SF,SF,PR0,SF,SF,PP0,SF,SW],
  &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW]
];

const W01: Tile = WT(0, 1);
const W11: Tile = WT(1, 1);
const W21: Tile = WT(2, 1);
const W31: Tile = WT(3, 1);
const W41: Tile = WT(4, 1);

const PN1 : Tile = PL(0, 1);
const PG1 : Tile = PL(1, 1);
const PY1 : Tile = PL(2, 1);
const PR1 : Tile = PL(3, 1);
const PP1 : Tile = PL(4, 1);

const BT1 : Tile = BT(ButtonType::AnyColor, &[Cell {value: { Vec2i {x: 8, y: 7} }}], &[], LVL_2_WIRE);

const LVL_2: &[&[Tile]] = &[
  &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW],
  &[SW,W01,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,W21,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,W41,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SW,SW,SW,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,W11,SF,SF,SF,SW,W31,SW,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SW,SF,SW,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,BT1,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,PN1,SF,SF,PG1,SF,SF,PY1,SF,SF,PR1,SF,SF,PP1,SF,SW],
  &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW]
];

const LVL_2_WIRE: &[&[WireTile]] = &[
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,SR,L4,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET]
];

const W04: Tile = WT(0, 3);
const W14: Tile = WT(1, 3);
const W24: Tile = WT(2, 3);
const W34: Tile = WT(3, 3);
const W44: Tile = WT(4, 3);

const PN4 : Tile = PL(0, 3);
const PG4 : Tile = PL(1, 3);
const PY4 : Tile = PL(2, 3);
const PR4 : Tile = PL(3, 3);
const PP4 : Tile = PL(4, 3);

const BT4 : Tile = 
BT(ButtonType::AnyColor, 
  &[Cell 
    {value: 
      { Vec2i 
        {x: 3, y: 6} 
      }
    },
    Cell 
    {value: 
      { Vec2i 
        {x: 3, y: 7} 
      }
    },
  ], &[], LVL_4_WIRE);

const LVL_4: &[&[Tile]] = &[
  &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW],
  &[SW,W14,SF,SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,W04,SW],
  &[SW,SF,SF,SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,B1,SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,BT4,SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,PN4,SF,SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,PG4,SW],
  &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW]
];

const LVL_4_WIRE: &[&[WireTile]] = &[
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,L1,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,VR,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,SU,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET]
];

const W05: Tile = WT(0, 4);
const W15: Tile = WT(1, 4);
const W25: Tile = WT(2, 4);
const W35: Tile = WT(3, 4);
const W45: Tile = WT(4, 4);

const PN5 : Tile = PL(0, 4);
const PG5 : Tile = PL(1, 4);
const PY5 : Tile = PL(2, 4);
const PR5 : Tile = PL(3, 4);
const PP5 : Tile = PL(4, 4);

const BT5 : Tile = 
BT(ButtonType::AnyColor, 
  &[Cell 
    {value: 
      { Vec2i 
        {x: 7, y: 0} 
      }
    },
    Cell 
    {value: 
      { Vec2i 
        {x: 8, y:0} 
      }
    },
  ], &[], LVL_5_WIRE);

const BT5U : Tile = 
BT(ButtonType::AnyColor, 
  &[Cell 
    {value: 
      { Vec2i 
        {x: 7, y: 13} 
      }
    },
    Cell 
    {value: 
      { Vec2i 
        {x: 8, y:13} 
      }
    },
  ], &[], LVL_5_WIRE_U);

const LVL_5: &[&[Tile]] = &[
  &[SW,SW,SW,SW,SW,SW,SW,SF,SF,SW,SW,SW,SW,SW,SW,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,BT5,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,W15,PN5,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,PY5,W35,SW],
  &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW]
];


const LVL_5_TOP: &[&[Tile]] = &[
  &[SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW,SW],
  &[SW,W05,PG5,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,PR5,W25,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,BT5U,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SF,SW],
  &[SW,SW,SW,SW,SW,SW,SW,SF,SF,SW,SW,SW,SW,SW,SW,SW]
];

const LVL_5_WIRE: &[&[WireTile]] = &[
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,VR,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,SU,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET]
];

const LVL_5_WIRE_U: &[&[WireTile]] = &[
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,SD,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,VR,ET,ET,ET,ET,ET,ET,ET,ET,ET],
  &[ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET,ET]
];

const Y : Tile = SPRITE(Sprite { index_x: 0, index_y: 23, flip_x: false, layer: 127, render: true});
const O : Tile = SPRITE(Sprite { index_x: 1, index_y: 23, flip_x: false, layer: 127, render: true});
const U : Tile = SPRITE(Sprite { index_x: 2, index_y: 23, flip_x: false, layer: 127, render: true});
const B : Tile = SPRITE(Sprite { index_x: 3, index_y: 23, flip_x: false, layer: 127, render: true});
const W : Tile = SPRITE(Sprite { index_x: 4, index_y: 23, flip_x: false, layer: 127, render: true});
const I : Tile = SPRITE(Sprite { index_x: 5, index_y: 23, flip_x: false, layer: 127, render: true});
const N : Tile = SPRITE(Sprite { index_x: 6, index_y: 23, flip_x: false, layer: 127, render: true});
const X : Tile = SPRITE(Sprite { index_x: 7, index_y: 23, flip_x: false, layer: 127, render: true});

const EMPTY_ROOM_WIN: &[&[Tile]] = &[
  &[SHEET(WIN_SCREEN),EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,WT(0, 5)],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,PL(0, 5),EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM],
  &[EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM,EM]
];

const WIN_SCREEN: &[&[Tile]] = &[
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B],
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B],
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B],
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B],
  &[B,B,B,B,Y,O,U,B,W,I,N,X,B,B,B,B],
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B],
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B],
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B],
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B],
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B],
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B],
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B],
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B],
  &[B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B]
];

#[derive(PartialEq)]
enum WireTile {
  ET,
  VR,
  HR,
  SU,
  SD,
  SL,
  SR,
  XC,
  TU,
  TR,
  TL,
  TD,
  L1,
  L2,
  L3,
  L4
}

use WireTile::*;


pub fn create(game: &mut Game) {
  let map_loop = iproduct!(
    0..MAP_ROOM_WIDTH as usize, 
    0..MAP_ROOM_HEIGHT as usize, 
    0..ROOM_TILE_WIDTH as usize, 
    0..ROOM_TILE_HEIGHT as usize);

  for (room_x, room_y, x, y) in map_loop {
    let cell = get_tile_cell(room_x, room_y, x, y);

    match &MAP[room_y][room_x][y][x] {
      SW => { create_stone_wall(game, cell) },
      SF => { create_stone_floor(game, cell) },
      BT(button_type, cells, slave_buttons, wires) => {
        let mut relative_cells = vec![]; 
        for cell in *cells {
          relative_cells.push(
            get_tile_cell(
              room_x, room_y, 
              cell.x as usize, cell.y as usize
            )
          );
        }
        let mut slaves = vec![];
        for slave in slave_buttons.clone() {
          slaves.push(
            create_slave_button(
              game, 
              get_tile_cell(
                room_x, room_y, 
                slave.cell.x as usize, slave.cell.y as usize
              ), 
              slave.button_type
            )
          );
        }
        let wires = draw_wires(game, wires, room_x, room_y);
        create_button(
          game, 
          cell, 
          button_type.clone(), 
          relative_cells,
          slaves,
          wires
        ) 
      },
      PL(clone, level) => { create_player_spawn(game, cell, *clone, *level) },
      BX(box_type) => { create_box(game, cell, box_type) }
      WT(color, level) => { create_win_tile(game, cell, *color, *level) }
      SPRITE(sprite) => { draw_sprite(game, cell, *sprite) },
      SHEET(sheet) => {
        for inner_x in 0..ROOM_TILE_WIDTH as usize {
          for inner_y in 0..ROOM_TILE_HEIGHT as usize {
            let cell = get_tile_cell(room_x, room_y, inner_x, inner_y);

            match &sheet[inner_y][inner_x] {
              SW => { create_stone_wall(game, cell) },
              SF => { create_stone_floor(game, cell) },
              BT(button_type, cells, slave_buttons, wires) => {
                let mut relative_cells = vec![]; 
                for cell in *cells {
                  relative_cells.push(
                    get_tile_cell(
                      room_x, room_y, 
                      cell.x as usize, cell.y as usize
                    )
                  );
                }
                let mut slaves = vec![];
                for slave in slave_buttons.clone() {
                  slaves.push(
                    create_slave_button(
                      game, 
                      get_tile_cell(
                        room_x, room_y, 
                        slave.cell.x as usize, slave.cell.y as usize
                      ), 
                      slave.button_type
                    )
                  );
                }
                let wires = draw_wires(game, wires, room_x, room_y);
                create_button(
                  game, 
                  cell, 
                  button_type.clone(), 
                  relative_cells,
                  slaves,
                  wires
                ) 
              },
              PL(clone, level) => { create_player_spawn(game, cell, *clone, *level) },
              BX(box_type) => { create_box(game, cell, box_type) }
              WT(color, level) => { create_win_tile(game, cell, *color, *level) }
              SPRITE(sprite) => { draw_sprite(game, cell, *sprite) },
              _ => { continue; }
            }
          }
        }
      }
      _ => { continue; }
    }
  }
}

fn draw_sprite(game: &mut Game, cell: Cell, sprite: Sprite) {
  let sprite_entity = game.world.new_entity();

  game.world.add_component_to_entity(sprite_entity, cell);
  game.world.add_component_to_entity(sprite_entity, cell.to_position());
  game.world.add_component_to_entity(sprite_entity, sprite);
}

fn create_win_tile(game: &mut Game, cell: Cell, color: u8, level: usize) {
  create_stone_floor(game, cell);

  let win_tile = game.world.new_entity();

  game.world.add_component_to_entity(win_tile, cell);
  game.world.add_component_to_entity(win_tile, cell.to_position());
  game.world.add_component_to_entity(win_tile, Sprite::new(0, 18 + color as u32, 1));
  game.world.add_component_to_entity(win_tile, Animator {
    animation: Animation {
      frames: vec![
        AnimationFrame::new(0, 18 + color as u32, 100),
        AnimationFrame::new(1, 18 + color as u32, 100),
        AnimationFrame::new(2, 18 + color as u32, 100)
      ],
      r#loop: true
    },
    frame_index: 0,
    time: 0,
    playing: true
  });

  game.world.add_component_to_entity(win_tile, WinTile { color, level });
}

fn draw_wires(
  game: &mut Game, wires: &[&[WireTile]], room_x: usize, room_y: usize
) -> Lst<usize> {
  let mut wire_entities : Lst<usize> = lst![];

  for x in 0..ROOM_TILE_WIDTH as usize {
    for y in 0..ROOM_TILE_HEIGHT as usize {
      let sprite_x = match &wires[y][x] {
        VR => {1}
        HR => {0}
        SU => {4}
        SD => {5}
        SL => {2}
        SR => {3}
        XC => {6}
        TU => {7}
        TR => {10}
        TL => {9}
        TD => {8}
        L1 => {14}
        L2 => {12}
        L3 => {13}
        L4 => {11}
        _ => { continue; }
      };
      let wire = game.world.new_entity();
      game.world.add_component_to_entity(wire, 
        Sprite::new(sprite_x as u32, 12, 1)
      );

      let cell = get_tile_cell(room_x, room_y, x, y);

      game.world.add_component_to_entity(wire, cell);
      game.world.add_component_to_entity(wire, cell.to_position());

      wire_entities.push(wire);
    }
  }

  wire_entities
}

fn create_box (game: &mut Game, cell: Cell, box_type: &BoxType) {
  create_stone_floor(game, cell);

  let box_entity = game.world.new_entity();

  game.world.add_component_to_entity(box_entity, cell);
  game.world.add_component_to_entity(box_entity, cell.to_position());
  game.world.add_component_to_entity(box_entity, box_sprite(box_type));
  game.world.add_component_to_entity(box_entity, Moveable {
    start_cell: cell,
    end_cell: cell,
    duration: 150,
    accumulator: 0,
    moving: false,
    box_moveable: true
  });
  game.world.add_component_to_entity(box_entity, PushBox { start_cell: cell} );

  game.colliders[cell.x as usize][cell.y as usize] = 
    Collider::Box(box_type.clone(), box_entity);
}

fn box_sprite(box_type: &BoxType) -> Sprite {
  match box_type {
    BoxType::Any => { Sprite::new(5, 11, 50) },
    BoxType::Color(col) => { Sprite::new(*col as u32, 11, 50) }
  }
}

fn create_button(
  game: &mut Game, cell: Cell, button_type: ButtonType, cells: Vec<Cell>,
  slaves: Vec<usize>, wires: Vec<usize>
) {
  create_stone_floor(game, cell);

  let button = game.world.new_entity();

  game.world.add_component_to_entity(button, cell);
  game.world.add_component_to_entity(button, cell.to_position());
  game.world.add_component_to_entity(button, button_sprite(button_type));

  let mut cells_entities = vec![];

  for cell in cells {
    let cell_entity = game.world.new_entity();

    game.world.add_component_to_entity(cell_entity, Sprite::new(2,5,2));
    game.world.add_component_to_entity(cell_entity, cell);
    game.world.add_component_to_entity(cell_entity, cell.to_position());
    game.colliders[cell.x as usize][cell.y as usize] = Collider::Solid;

    cells_entities.push(cell_entity);
  }

  game.world.add_component_to_entity(button, 
    Button::new(button_type, cells_entities, slaves, wires)
  );
}

fn create_slave_button(
  game: &mut Game, cell: Cell, button_type: ButtonType
) -> usize {
  create_stone_floor(game, cell);

  let button = game.world.new_entity();

  game.world.add_component_to_entity(button, cell);
  game.world.add_component_to_entity(button, cell.to_position());
  game.world.add_component_to_entity(button, button_sprite(button_type));

  game.world.add_component_to_entity(button, 
    SlaveButton { button_type, pressed: false }
  );

  button
}

fn button_sprite(button_type: ButtonType) -> Sprite {
  match button_type {
    ButtonType::AnyColor => { Sprite::new(5, 7, 2) },
    ButtonType::Color(col) => { Sprite::new(col as u32, 7, 2) }
  }
}

fn create_player_spawn(game: &mut Game, cell: Cell, clone: usize, level: usize) {
  create_stone_floor(game, cell);

  let player_spawn = game.world.new_entity();

  game.clone_spawns[level][clone] = cell;

  game.world.add_component_to_entity(player_spawn, cell.to_position());

  game.world.add_component_to_entity(
    player_spawn, Sprite::new(clone as u32, 6, 1)
  );
}

fn create_stone_floor(game: &mut Game, cell: Cell) {
  let stone_floor = game.world.new_entity();

  game.world.add_component_to_entity(stone_floor, cell.to_position());
  game.world.add_component_to_entity(stone_floor, Sprite::new(1, 5, 0));
}

fn create_stone_wall(game: &mut Game, cell: Cell) {
  let stone_wall = game.world.new_entity();

  game.world.add_component_to_entity(stone_wall, cell.to_position());
  game.world.add_component_to_entity(stone_wall, Sprite::new(0, 5, 0));
  game.colliders[cell.x as usize][cell.y as usize] = Collider::Solid;
}


fn get_tile_cell(room_x: usize, room_y: usize, x: usize, y: usize) -> Cell {
  let rev_y = (ROOM_TILE_HEIGHT - 1) - y as u32;

  let cell = Cell::new(        
    (room_x as i32 * ROOM_TILE_WIDTH as i32) + x as i32,
    (room_y as i32 * ROOM_TILE_HEIGHT as i32) + rev_y as i32
  );

  cell
}