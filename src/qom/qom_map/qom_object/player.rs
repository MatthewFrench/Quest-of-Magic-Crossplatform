use quicksilver::geom::Vector;
use quicksilver::graphics::Background::Img;
use quicksilver::prelude::Shape;
use quicksilver::prelude::{Image, Window};
use std::collections::HashMap;

struct Player {
    is_visible: bool,
    x: f32,
    y: f32,
    image_id: u32,
}
impl Player {
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
        }
    }
}
