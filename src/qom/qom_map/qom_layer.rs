use crate::qom::qom_map::{TILE_HEIGHT, TILE_WIDTH};
use quicksilver::geom::Vector;
use quicksilver::graphics::Background::Img;
use quicksilver::prelude::Shape;
use quicksilver::prelude::{Image, Window};
use std::collections::HashMap;

pub struct QomLayer {
    pub name: String,
    // (x, y), image_id
    pub tiles: HashMap<(i32, i32), u32>,
    pub layer_index: u32,
}
impl QomLayer {
    pub fn render(
        &self,
        // todo: Window isn't required, drawables can be drawn directly to context
        window: &mut Window,
        images: &HashMap<u32, Image>,
        view_position: Vector,
    ) {
        for ((x, y), tile) in &self.tiles {
            let render_x = *x as f32 * TILE_WIDTH as f32 - view_position.x;
            let render_y = *y as f32 * TILE_HEIGHT as f32 - view_position.y;
            if render_x > (0 - TILE_WIDTH) as f32
                && render_y > (0 - TILE_HEIGHT) as f32
                && render_x < window.screen_size().x
                && render_y < window.screen_size().y
            {
                let image = images.get(&tile);
                if let Some(image) = image {
                    window.draw(&image.area().translate((render_x, render_y)), Img(image));
                }
            }
        }
    }
}
