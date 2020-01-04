use crate::qom::transitions::ScreenTransition;
use crate::qom::QuestOfMagicData;
use quicksilver::prelude::{Event, Window};

pub mod loading_screen;
pub mod world_screen;

//: 'static
pub trait Screen {
    fn new(_data: &mut QuestOfMagicData) -> Self
    where
        Self: Sized;
    fn update(&mut self, _window: &mut Window, _data: &mut QuestOfMagicData) -> ScreenTransition {
        ScreenTransition::None
    }
    fn event(&mut self, _event: &Event, _window: &mut Window) -> ScreenTransition {
        ScreenTransition::None
    }
    fn draw(&mut self, window: &mut Window) {}
}
