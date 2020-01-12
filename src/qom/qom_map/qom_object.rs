use crate::qom::qom_map::{TILE_HEIGHT, TILE_WIDTH};
use quicksilver::geom::Vector;
use quicksilver::graphics::Background::Img;
use quicksilver::prelude::Shape;
use quicksilver::prelude::{Image, Window};
use std::collections::HashMap;

// Object coordinates have the bottom-left of the image as 0,0. So subtract height.
const OBJECT_WIDTH_OFFSET: f32 = 0.0;
const OBJECT_HEIGHT_OFFSET: f32 = -TILE_HEIGHT as f32 * 1.0;

pub struct QomObject {
    pub x: i32,
    pub y: i32,
    pub name: String,
    pub object_type: String,
    pub image_id: u32,
}

impl QomObject {
    pub fn render(
        &self,
        // todo: Window isn't required, drawables can be drawn directly to context
        window: &mut Window,
        images: &HashMap<u32, Image>,
        view_position: Vector,
    ) {
        if self.image_id > 0 {
            let image = images.get(&self.image_id);
            if let Some(image) = image {
                window.draw(
                    &image.area().translate((
                        self.x as f32 - view_position.x + OBJECT_WIDTH_OFFSET,
                        self.y as f32 - view_position.y + OBJECT_HEIGHT_OFFSET,
                    )),
                    Img(image),
                );
            }
        }
    }
}
