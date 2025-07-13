use raylib::prelude::*;
use crate::breaker::block;

pub fn generate_blocks(world_width: i32, rows: i32, collumns: i32, height_area: Rectangle, gap: i32, colors: &mut [Color]) -> Vec<block::Block> {
    assert!(colors.len() > 0);

    let block_width: i32 = world_width / collumns;
    let block_height: i32 = height_area.height as i32 / rows;
    let mut blocks: Vec<block::Block> = Vec::new();
    let mut color_index: usize = 0;
    for y_level in 0..rows {
        for x_level in 0..collumns {

            let block: block::Block = block::Block {
                body: Rectangle {
                    x:      (x_level * block_width + gap * x_level) as f32,
                    y:      (y_level * block_height) as f32,
                    width:  (block_width - gap) as f32,
                    height: (block_height - gap) as f32
                },
                color: colors[color_index],
            };
            blocks.push(block);

        }
        color_index += 1;

        if color_index >= colors.len() {
            color_index = 0;
        }
    }
    blocks
}
