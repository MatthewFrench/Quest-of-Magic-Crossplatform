use crate::qom::screens::Screen;
use crate::qom::transitions::ScreenTransition;
use crate::qom::QuestOfMagicData;
use quicksilver::graphics::Background::Img;
use quicksilver::prelude::{Shape, Window};

pub struct WorldScreen {}

impl Screen for WorldScreen {
    fn new(data: &mut QuestOfMagicData) -> WorldScreen {
        portable_log!("World screen");
        portable_log!("Data: {:?}", data.overworld_map);

        WorldScreen {}
    }

    fn update(&mut self, _window: &mut Window, data: &mut QuestOfMagicData) -> ScreenTransition {
        ScreenTransition::None
    }

    fn draw(&mut self, window: &mut Window, data: &mut QuestOfMagicData) {
        // Todo
        let image = data
            .image_assets
            .get(data.image_assets.keys().last().unwrap())
            .unwrap();
        window.draw(&image.area().with_center((400, 300)), Img(&image));
    }
}
