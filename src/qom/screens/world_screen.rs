use crate::qom::qom_data::QuestOfMagicData;
use crate::qom::qom_map::qom_object::qom_player_object::MoveDirection;
use crate::qom::qom_map::{TILE_HEIGHT, TILE_WIDTH};
use crate::qom::screens::Screen;
use crate::qom::transitions::ScreenTransition;
use nalgebra::Point2;
use quicksilver::geom::{Rectangle, Vector};
use quicksilver::graphics::View;
use quicksilver::input::Key;
use quicksilver::prelude::Window;

// Keyboard arrow key combination constants
#[derive(PartialEq, Eq)]
struct UpDownLeftRight {
    up: bool,
    left: bool,
    down: bool,
    right: bool,
}
const KEYBOARD_UP_LEFT: UpDownLeftRight = UpDownLeftRight {
    up: true,
    left: true,
    down: false,
    right: false,
};
const KEYBOARD_UP_RIGHT: UpDownLeftRight = UpDownLeftRight {
    up: true,
    left: false,
    down: false,
    right: true,
};
const KEYBOARD_UP: UpDownLeftRight = UpDownLeftRight {
    up: true,
    left: false,
    down: false,
    right: false,
};
const KEYBOARD_DOWN_LEFT: UpDownLeftRight = UpDownLeftRight {
    up: false,
    left: true,
    down: true,
    right: false,
};
const KEYBOARD_DOWN_RIGHT: UpDownLeftRight = UpDownLeftRight {
    up: false,
    left: false,
    down: true,
    right: true,
};
const KEYBOARD_DOWN: UpDownLeftRight = UpDownLeftRight {
    up: false,
    left: false,
    down: true,
    right: false,
};
const KEYBOARD_RIGHT: UpDownLeftRight = UpDownLeftRight {
    up: false,
    left: false,
    down: false,
    right: true,
};
const KEYBOARD_LEFT: UpDownLeftRight = UpDownLeftRight {
    up: false,
    left: true,
    down: false,
    right: false,
};

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
        data.player_data.set_tile_position(Point2::new(
            (human_start.x as f32 / TILE_WIDTH as f32).round(),
            (human_start.y as f32 / TILE_HEIGHT as f32).round(),
        ));
        data.player_data.set_image_id(human_start.image_id);
        return world;
    }

    fn update(&mut self, window: &mut Window, data: &mut QuestOfMagicData) -> ScreenTransition {
        match (UpDownLeftRight {
            up: window.keyboard()[Key::Up].is_down(),
            left: window.keyboard()[Key::Left].is_down(),
            down: window.keyboard()[Key::Down].is_down(),
            right: window.keyboard()[Key::Right].is_down(),
        }) {
            KEYBOARD_UP_LEFT => data.player_data.move_direction(MoveDirection::UpLeft),
            KEYBOARD_UP_RIGHT => data.player_data.move_direction(MoveDirection::UpRight),
            KEYBOARD_UP => data.player_data.move_direction(MoveDirection::Up),
            KEYBOARD_DOWN_LEFT => data.player_data.move_direction(MoveDirection::DownLeft),
            KEYBOARD_DOWN_RIGHT => data.player_data.move_direction(MoveDirection::DownRight),
            KEYBOARD_DOWN => data.player_data.move_direction(MoveDirection::Down),
            KEYBOARD_LEFT => data.player_data.move_direction(MoveDirection::Left),
            KEYBOARD_RIGHT => data.player_data.move_direction(MoveDirection::Right),
            _ => data.player_data.move_direction(MoveDirection::None),
        }
        data.player_data.update();

        let view = Rectangle::new_sized((window.screen_size().x, window.screen_size().y));
        window.set_view(View::new(view));
        ScreenTransition::None
    }

    fn draw(&mut self, window: &mut Window, data: &mut QuestOfMagicData) {
        let view_position = Vector::new(
            data.player_data.get_pixel_x() + TILE_WIDTH as f32 / 2.0 - window.screen_size().x / 2.0,
            data.player_data.get_pixel_y() + TILE_HEIGHT as f32 / 2.0
                - window.screen_size().y / 2.0,
        );
        data.overworld_map
            .render(window, &data.image_assets, view_position);
        // Draw player
        data.player_data
            .render(window, &data.image_assets, view_position);
    }
}
