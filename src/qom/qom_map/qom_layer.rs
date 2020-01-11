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
            let image = images.get(&tile);
            if let Some(image) = image {
                window.draw(
                    &image.area().with_center((
                        *x as f32 * TILE_WIDTH as f32 - view_position.x,
                        *y as f32 * TILE_HEIGHT as f32 - view_position.y,
                    )),
                    Img(image),
                );
            }
        }
    }
}
