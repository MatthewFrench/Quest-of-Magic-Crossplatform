use crate::qom::screens::Screen;
use crate::qom::tiled::Map;
use crate::qom::transitions::ScreenTransition;
use crate::qom::QuestOfMagicData;
use quicksilver::geom::Rectangle;
use quicksilver::graphics::Background::Img;
use quicksilver::graphics::View;
use quicksilver::input::Key;
use quicksilver::prelude::{Image, Shape, Window};
use std::collections::HashMap;

const TILE_WIDTH: i32 = 44;
const TILE_HEIGHT: i32 = 44;

const LAYER_GROUND_1: &str = "Ground 1";
const LAYER_GROUND_2: &str = "Ground 2";
const LAYER_ABOVE_GROUND_1: &str = "Above Ground 1";
const LAYER_ABOVE_GROUND_2: &str = "Above Ground 2";
const LAYER_ABOVE_GROUND_3: &str = "Above Ground 3";
const LAYER_TILE_COLLISION_MAP: &str = "Tile Collision Map";
const LAYER_NPCS_AND_INTERACTIONS: &str = "NPCs and Interactions";

pub struct WorldScreen {
    center_x: i32,
    center_y: i32,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    render_layers: Vec<String>,
}
impl Default for WorldScreen {
    fn default() -> Self {
        WorldScreen {
            center_x: 0,
            center_y: 0,
            min_x: 0,
            max_x: 0,
            max_y: 0,
            min_y: 0,
            render_layers: vec![
                String::from(LAYER_GROUND_1),
                String::from(LAYER_GROUND_2),
                String::from(LAYER_ABOVE_GROUND_1),
                String::from(LAYER_ABOVE_GROUND_2),
                String::from(LAYER_ABOVE_GROUND_3),
            ],
        }
    }
}

impl WorldScreen {
    fn render_layer(
        &self,
        window: &mut Window,
        layer_name: &str,
        map: &Map,
        images: &HashMap<u32, Image>,
    ) {
        let tile_width = map.tile_width as i32;
        let tile_height = map.tile_height as i32;
        let layer = map.get_layer_by_name(&String::from(layer_name));
        if let Some(layer) = layer {
            for (y, row) in &layer.tiles {
                for (x, tile) in row {
                    let image = images.get(&tile.gid);
                    if let Some(image) = image {
                        window.draw(
                            &image.area().with_center((
                                (x - self.center_x) * tile_width,
                                (y - self.center_y) * tile_height,
                            )),
                            Img(image),
                        );
                    }
                }
            }
        }
    }
}

impl Screen for WorldScreen {
    fn new(data: &mut QuestOfMagicData) -> WorldScreen {
        portable_log!("World screen");

        let mut world = WorldScreen {
            ..Default::default()
        };

        // Calculate center of tiles
        let mut set = false;
        let mut x1: i32 = 0;
        let mut x2: i32 = 0;
        let mut y1: i32 = 0;
        let mut y2: i32 = 0;

        if let Some(overworld_map) = &data.overworld_map {
            for layer_name in &world.render_layers {
                let layer = overworld_map.get_layer_by_name(layer_name);
                if let Some(layer) = layer {
                    for (y, row) in &layer.tiles {
                        for (x, tile) in row {
                            let image = data.image_assets.get(&tile.gid);
                            if let Some(image) = image {
                                if !set {
                                    set = true;
                                    x1 = *x;
                                    x2 = *x;
                                    y1 = *y;
                                    y2 = *y;
                                }
                                if *x < x1 {
                                    x1 = *x;
                                }
                                if *y < y1 {
                                    y1 = *y;
                                }
                                if *x > x2 {
                                    x2 = *x;
                                }
                                if *x > y2 {
                                    y2 = *y;
                                }
                            }
                        }
                    }
                }
            }
        }

        world.center_x = (x2 - x1) / 2 + x1;
        world.center_y = (y2 - y1) / 2 + y1;
        world.min_x = x1;
        world.max_x = x2;
        world.min_y = y1;
        world.max_y = y2;

        portable_log!("x: {} to {}", x1, x2);
        portable_log!("y: {} to {}", y1, y2);

        return world;
    }

    fn update(&mut self, window: &mut Window, _data: &mut QuestOfMagicData) -> ScreenTransition {
        if window.keyboard()[Key::Left].is_down() {
            self.center_x -= 1;
        }
        if window.keyboard()[Key::Right].is_down() {
            self.center_x += 1;
        }
        if window.keyboard()[Key::Down].is_down() {
            self.center_y += 1;
        }
        if window.keyboard()[Key::Up].is_down() {
            self.center_y -= 1;
        }
        let max =
            ((self.max_x - self.min_x) * TILE_WIDTH).max((self.max_y - self.min_y) * TILE_HEIGHT);
        let view = Rectangle::new_sized((max, max)).translate((self.center_x, self.center_y));
        window.set_view(View::new(view));
        ScreenTransition::None
    }

    fn draw(&mut self, window: &mut Window, data: &mut QuestOfMagicData) {
        if let Some(overworld_map) = &data.overworld_map {
            for layer in &self.render_layers {
                self.render_layer(window, layer, &overworld_map, &data.image_assets);
            }
        }
    }
}
