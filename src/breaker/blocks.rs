use raylib::prelude::*;
use crate::breaker::block;

pub fn generate_blocks(world_width: i32, world_height: i32, rows: i32, collumns: i32, divide_height_area: i32, gap: i32) -> Vec<block::Block> {
    let block_width: i32 = world_width / collumns;
    let block_height: i32 = world_height / divide_height_area / rows;
    let mut blocks: Vec<block::Block> = Vec::new();
    for y_level in 0..rows {
        for x_level in 0..collumns {
            let block: block::Block = block::Block {
                body: Rectangle {
                    x:      (x_level * block_width + gap * x_level) as f32,
                    y:      (y_level * block_height) as f32,
                    width:  (block_width - gap) as f32,
                    height: (block_height - gap) as f32
                },
                color: Color::WHITE,
            };
            blocks.push(block);
        }
    }
    blocks
}
