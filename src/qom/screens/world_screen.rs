use crate::qom::qom_data::QuestOfMagicData;
use crate::qom::screens::Screen;
use crate::qom::transitions::ScreenTransition;
use quicksilver::geom::{Rectangle, Vector};
use quicksilver::graphics::View;
use quicksilver::input::Key;
use quicksilver::prelude::Window;

pub struct WorldScreen {
    // todo: replace with a camera object
    view_position: Vector,
    world_id: u32,
}

impl Default for WorldScreen {
    fn default() -> Self {
        WorldScreen {
            view_position: Default::default(),
            world_id: 0,
        }
    }
}

impl Screen for WorldScreen {
    fn new(data: &mut QuestOfMagicData) -> WorldScreen {
        portable_log!("World screen");

        let mut world = WorldScreen {
            ..Default::default()
        };

        return world;
    }

    fn update(&mut self, window: &mut Window, _data: &mut QuestOfMagicData) -> ScreenTransition {
        if window.keyboard()[Key::Left].is_down() {
            self.view_position.x -= 10.0;
        }
        if window.keyboard()[Key::Right].is_down() {
            self.view_position.x += 10.0;
        }
        if window.keyboard()[Key::Down].is_down() {
            self.view_position.y += 10.0;
        }
        if window.keyboard()[Key::Up].is_down() {
            self.view_position.y -= 10.0;
        }
        let view = Rectangle::new_sized((window.screen_size().x, window.screen_size().y));
        window.set_view(View::new(view));
        ScreenTransition::None
    }

    fn draw(&mut self, window: &mut Window, data: &mut QuestOfMagicData) {
        data.overworld_map
            .render(window, &data.image_assets, self.view_position);
    }
}
