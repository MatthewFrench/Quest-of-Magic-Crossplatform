use crate::qom::QuestOfMagicData;
use quicksilver::prelude::{Event, Image, Window};
use quicksilver::Result;
use std::collections::HashMap;

pub mod loading_screen;

//: 'static
pub trait Screen {
    fn new() -> Self
    where
        Self: Sized;
    fn update(&mut self, _window: &mut Window, _data: &mut QuestOfMagicData) -> Result<()> {
        Ok(())
    }
    fn event(&mut self, _event: &Event, _window: &mut Window) -> Result<()> {
        Ok(())
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }
}
