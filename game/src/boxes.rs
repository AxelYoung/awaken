use crate::common::Cell;

#[derive(PartialEq, Clone, Copy)]
pub enum BoxType {
   Color(u8),
   Any
}

pub struct PushBox {
   pub start_cell: Cell
}