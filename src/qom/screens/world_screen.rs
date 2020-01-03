use crate::qom::screens::Screen;
use crate::qom::transitions::ScreenTransition;
use crate::qom::QuestOfMagicData;
use quicksilver::prelude::Window;

pub struct WorldScreen {}

impl Screen for WorldScreen {
    fn new() -> WorldScreen {
        portable_log!("World screen");

        WorldScreen {}
    }

    fn update(&mut self, _window: &mut Window, data: &mut QuestOfMagicData) -> ScreenTransition {
        ScreenTransition::None
    }

    fn draw(&mut self, window: &mut Window) {
        // Todo
    }
}
