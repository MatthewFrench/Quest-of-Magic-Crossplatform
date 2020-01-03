extern crate quicksilver;

use tiled::{parse_file, Map, Tileset};

use crate::qom::screens::loading_screen::LoadingScreen;
use crate::qom::screens::Screen;
use quicksilver::prelude::{Asset, Event, Image};
use quicksilver::{
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{run, Settings, State, Window},
    Result,
};
use std::collections::HashMap;
pub mod screens;
pub mod tiled;

pub const LAYER_GROUND_1: &str = "Ground 1";
pub const LAYER_GROUND_2: &str = "Ground 2";
pub const LAYER_ABOVE_GROUND_1: &str = "Above Ground 1";
pub const LAYER_ABOVE_GROUND_2: &str = "Above Ground 2";
pub const LAYER_ABOVE_GROUND_3: &str = "Above Ground 3";
pub const LAYER_TILE_COLLISION_MAP: &str = "Tile Collision Map";
pub const LAYER_NPCS_AND_INTERACTIONS: &str = "NPCs and Interactions";

pub struct QuestOfMagicData {
    /// All game data goes here
    pub overworld_map: Option<Map>,
    pub image_assets: HashMap<String, Image>,
}

pub struct QuestOfMagic {
    /// Game state goes here
    pub data: QuestOfMagicData,
    pub menu_stack: Vec<Box<dyn Screen>>,
}

impl State for QuestOfMagic {
    fn new() -> Result<QuestOfMagic> {
        let mut qom = QuestOfMagic {
            data: QuestOfMagicData {
                overworld_map: None,
                image_assets: HashMap::new(),
            },
            menu_stack: Vec::new(),
        };
        // Push loading screen to stack
        qom.menu_stack.push(Box::new(LoadingScreen::new()));
        Ok(qom)
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        /*
        Data gets passed in, menu stack transactions get returned
        */
        if let Some(menu) = self.menu_stack.last_mut() {
            return menu.update(window, &mut self.data);
        }
        Ok(())
    }
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        if let Some(menu) = self.menu_stack.last_mut() {
            return menu.event(event, window);
        }
        Ok(())
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        // draw the top of the stack
        if let Some(menu) = self.menu_stack.last_mut() {
            return menu.draw(window);
        }
        Ok(())
    }
}
