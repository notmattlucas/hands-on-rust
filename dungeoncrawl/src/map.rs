use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x:i32, y:i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

fn within_lim(i: i32, lim: i32) -> bool {
    i >= 0 && i < lim
}

impl Map {

    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        within_lim(point.x, SCREEN_WIDTH) && within_lim(point.y, SCREEN_HEIGHT)
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        // TODO - page 99
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

}