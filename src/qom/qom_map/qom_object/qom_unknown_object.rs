use crate::qom::qom_map::qom_object::QomObject;
use quicksilver::geom::Vector;
use quicksilver::graphics::Background::Img;
use quicksilver::prelude::Shape;
use quicksilver::prelude::{Image, Window};
use std::collections::HashMap;

pub struct QomUnknownObject {
    pub x: i32,
    pub y: i32,
    pub name: String,
    pub object_type: String,
    pub image_id: u32,
}

impl QomObject for QomUnknownObject {
    fn render(
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
                        self.x as f32 - view_position.x,
                        self.y as f32 - view_position.y,
                    )),
                    Img(image),
                );
            }
        }
    }
}
