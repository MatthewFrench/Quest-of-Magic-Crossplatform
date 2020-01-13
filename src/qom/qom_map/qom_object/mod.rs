use quicksilver::geom::Vector;
use quicksilver::prelude::{Image, Window};
use std::collections::HashMap;

pub mod qom_entrance_object;
pub mod qom_house_object;
pub mod qom_npc_object;
pub mod qom_player_object;
pub mod qom_sign_object;
pub mod qom_unknown_object;

pub trait QomObject {
    fn render(
        &self,
        // todo: Window isn't required, drawables can be drawn directly to context
        window: &mut Window,
        images: &HashMap<u32, Image>,
        view_position: Vector,
    );
}
