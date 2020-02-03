use crate::qom::qom_data::QuestOfMagicData;
use crate::qom::qom_map::{TILE_HEIGHT, TILE_WIDTH};
use crate::qom::screens::Screen;
use crate::qom::transitions::ScreenTransition;
use quicksilver::geom::{Rectangle, Vector};
use quicksilver::graphics::View;
use quicksilver::input::Key;
use quicksilver::prelude::Window;

pub struct WorldScreen {
    // todo: camera object
}

impl Default for WorldScreen {
    fn default() -> Self {
        WorldScreen {}
    }
}

impl Screen for WorldScreen {
    fn new(data: &mut QuestOfMagicData) -> WorldScreen {
        portable_log!("World screen");

        let mut world = WorldScreen {
            ..Default::default()
        };

        // Temporary Initialize game data
        // Set the player start to start human-start
        let human_start = data.overworld_map.get_human_start();
        data.player_data.x = human_start.x as f32;
        data.player_data.y = human_start.y as f32;
        data.player_data.image_id = human_start.image_id;
        return world;
    }

    fn update(&mut self, window: &mut Window, data: &mut QuestOfMagicData) -> ScreenTransition {
        if window.keyboard()[Key::Left].is_down() {
            data.player_data.move_amount(-TILE_WIDTH as f32, 0.0);
        }
        if window.keyboard()[Key::Right].is_down() {
            data.player_data.move_amount(TILE_WIDTH as f32, 0.0);
        }
        if window.keyboard()[Key::Down].is_down() {
            data.player_data.move_amount(0.0, TILE_HEIGHT as f32);
        }
        if window.keyboard()[Key::Up].is_down() {
            data.player_data.move_amount(0.0, -TILE_HEIGHT as f32);
        }
        data.player_data.update();
        let view = Rectangle::new_sized((window.screen_size().x, window.screen_size().y));
        window.set_view(View::new(view));
        ScreenTransition::None
    }

    fn draw(&mut self, window: &mut Window, data: &mut QuestOfMagicData) {
        let view_position = Vector::new(
            data.player_data.x + TILE_WIDTH as f32 / 2.0 - window.screen_size().x / 2.0,
            data.player_data.y + TILE_HEIGHT as f32 / 2.0 - window.screen_size().y / 2.0,
        );
        data.overworld_map
            .render(window, &data.image_assets, view_position);
        // Draw player
        data.player_data
            .render(window, &data.image_assets, view_position);
    }
}
