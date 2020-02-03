use crate::qom::qom_map::qom_object::qom_player_object::QomPlayerObject;
use crate::qom::qom_map::QomMap;
use quicksilver::prelude::Image;
use std::collections::HashMap;

pub struct QuestOfMagicData {
    /// All game data goes here
    pub overworld_map: QomMap,
    // gid : image
    pub image_assets: HashMap<u32, Image>,
    pub player_data: QomPlayerObject,
}
