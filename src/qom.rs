extern crate quicksilver;

use crate::qom::qom_data::QuestOfMagicData;
use crate::qom::qom_map::qom_object::qom_player_object::QomPlayerObject;
use crate::qom::qom_map::QomMap;
use crate::qom::screens::loading_screen::LoadingScreen;
use crate::qom::screens::Screen;
use crate::qom::transitions::ScreenTransition;
use nalgebra::Point2;
use quicksilver::prelude::{Event, Image};
use quicksilver::{
    graphics::Color,
    lifecycle::{State, Window},
    Result,
};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub mod qom_data;
pub mod qom_map;
pub mod screens;
pub mod tiled;
pub mod transitions;
pub mod utility;

pub struct QuestOfMagic {
    /// Game state goes here
    pub data: QuestOfMagicData,
    pub screen_stack: Vec<Box<dyn Screen>>,
}

impl QuestOfMagic {
    fn handle_transition(&mut self, transition: ScreenTransition) {
        match transition {
            ScreenTransition::None => {}
            ScreenTransition::Pop => {
                self.screen_stack.pop();
            }
            ScreenTransition::Push(screen, transition_effect) => {
                self.screen_stack.push(screen);
                // Todo transitions
            }
            ScreenTransition::Replace(screen, transition_effect) => {
                self.screen_stack.pop();
                self.screen_stack.push(screen);
                // Todo transitions
            }
        }
    }
}

impl State for QuestOfMagic {
    fn new() -> Result<QuestOfMagic> {
        let mut qom = QuestOfMagic {
            data: QuestOfMagicData {
                overworld_map: QomMap::empty(),
                image_assets: HashMap::new(),
                player_data: QomPlayerObject::new(),
            },
            screen_stack: Vec::new(),
        };
        // Push loading screen to stack
        qom.screen_stack
            .push(Box::new(LoadingScreen::new(&mut qom.data)));
        Ok(qom)
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        /*
        Data gets passed in, screen stack transactions get returned
        */
        if let Some(screen) = self.screen_stack.last_mut() {
            let transition = screen.update(window, &mut self.data);
            self.handle_transition(transition);
        }
        Ok(())
    }
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        if let Some(screen) = self.screen_stack.last_mut() {
            let transition = screen.event(event, window, &mut self.data);
        }
        Ok(())
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        // draw the top of the stack
        if let Some(screen) = self.screen_stack.last_mut() {
            screen.draw(window, &mut self.data);
        }
        Ok(())
    }
}
