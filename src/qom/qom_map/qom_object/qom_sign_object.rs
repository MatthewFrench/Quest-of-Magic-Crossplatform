use quicksilver::geom::Vector;
use quicksilver::graphics::Background::Img;
use quicksilver::prelude::Shape;
use quicksilver::prelude::{Image, Window};
use std::collections::HashMap;

pub struct QomSignObject {
    pub is_visible: bool,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub image_id: u32,
}
impl QomSignObject {
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
                    &image
                        .area()
                        .translate((self.x - view_position.x, self.y - view_position.y)),
                    Img(image),
                );
            }
        } else {
            portable_log!("Unable to find image with image id {}", self.image_id);
        }
    }
}
