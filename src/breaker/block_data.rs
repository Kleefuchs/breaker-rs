use crate::{breaker::block, gameobject};
use raylib::prelude::*;
mod blocks;

pub struct BlockData {
    pub blocks: Vec<block::Block>,
    pub block_area: Rectangle,
}

impl BlockData {
    
    pub fn new(world_width: i32, world_height: i32, divide_height_area: i32, rows: i32, collumns: i32, gap: i32, colors: &mut [Color]) -> BlockData {
        let block_area: Rectangle = Rectangle {
            x: 0.0,
            y: 0.0,
            width: world_width as f32,
            height: (world_height / divide_height_area) as f32,
        };
        BlockData {
            block_area,
            blocks: blocks::generate_blocks(world_width, rows, collumns, block_area, gap, colors)
        }
    }

}

impl gameobject::Gameobject for BlockData {
    fn update(&mut self, _handle: &RaylibHandle) {
    }

    fn draw<T>(&self, texture_mode: &mut RaylibTextureMode<T>) {
        for block in &self.blocks {
            block.draw(texture_mode);
        }
    }
}
