use include_dir::{include_dir, Dir};

pub const TILE_SIZE: u32 = 16;
pub const TILE_DATA_SIZE: usize = (TILE_SIZE * TILE_SIZE * 4) as usize;

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/");

pub fn get_sprite(name: &str) -> &[u8] {
    ASSETS.get_file("img/".to_owned() + name + ".pix").unwrap().contents()
}