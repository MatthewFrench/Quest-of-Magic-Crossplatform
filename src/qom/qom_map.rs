use crate::qom::tiled::Map;
use quicksilver::geom::Vector;
use quicksilver::graphics::Background::Img;
use quicksilver::prelude::{Image, Shape, Window};
use std::collections::{HashMap, HashSet};

const TILE_WIDTH: i32 = 44;
const TILE_HEIGHT: i32 = 44;
const LAYER_GROUND_1: &str = "Ground 1";
const LAYER_GROUND_2: &str = "Ground 2";
const LAYER_ABOVE_GROUND_1: &str = "Above Ground 1";
const LAYER_ABOVE_GROUND_2: &str = "Above Ground 2";
const LAYER_ABOVE_GROUND_3: &str = "Above Ground 3";
const LAYER_TILE_COLLISION_MAP: &str = "Tile Collision Map";
const LAYER_NPCS_AND_INTERACTIONS: &str = "NPCs and Interactions";

pub struct QomLayer {
    pub name: String,
    // (x, y), image_id
    pub tiles: HashMap<(i32, i32), u32>,
    pub layer_index: u32,
}
impl QomLayer {
    fn render(
        &self,
        // todo: Window isn't required, drawables can be drawn directly to context
        window: &mut Window,
        images: &HashMap<u32, Image>,
        view_position: Vector,
    ) {
        for ((x, y), tile) in &self.tiles {
            let image = images.get(&tile);
            if let Some(image) = image {
                window.draw(
                    &image.area().with_center((
                        *x as f32 * TILE_WIDTH as f32 - view_position.x,
                        *y as f32 * TILE_HEIGHT as f32 - view_position.y,
                    )),
                    Img(image),
                );
            }
        }
    }
}
pub struct Bounds<T> {
    x1: T,
    y1: T,
    x2: T,
    y2: T,
}
pub struct QomMap {
    bounds: Bounds<i32>,
    render_layers: Vec<QomLayer>,
    collision_tiles: HashSet<(i32, i32)>,
    // todo: objects
}
impl QomMap {
    pub fn empty() -> QomMap {
        QomMap {
            bounds: Bounds {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
            },
            render_layers: vec![],
            collision_tiles: HashSet::new(),
        }
    }
    pub fn new(tiled_map: &Map) -> QomMap {
        let mut qom_map = QomMap::empty();

        // Calculate bounds
        let mut bounds_set = false;
        for layer in &tiled_map.layers {
            for (y, row) in &layer.tiles {
                for (x, tile) in row {
                    if tile.gid != 0 {
                        if !bounds_set {
                            bounds_set = true;
                            qom_map.bounds.x1 = *x;
                            qom_map.bounds.x2 = *x;
                            qom_map.bounds.y1 = *y;
                            qom_map.bounds.y2 = *y;
                        }
                        if *x < qom_map.bounds.x1 {
                            qom_map.bounds.x1 = *x;
                        }
                        if *y < qom_map.bounds.y1 {
                            qom_map.bounds.y1 = *y;
                        }
                        if *x > qom_map.bounds.x2 {
                            qom_map.bounds.x2 = *x;
                        }
                        if *x > qom_map.bounds.y2 {
                            qom_map.bounds.y2 = *y;
                        }
                    }
                }
            }
        }

        // Calculate collision tiles
        let layer = tiled_map
            .get_layer_by_name(&String::from(LAYER_TILE_COLLISION_MAP))
            .unwrap();
        for (y, row) in &layer.tiles {
            for (x, tile) in row {
                if tile.gid != 0 {
                    qom_map.collision_tiles.insert((*x, *y));
                }
            }
        }

        // Create rendered layers
        let render_layer_names = vec![
            LAYER_GROUND_1,
            LAYER_GROUND_2,
            LAYER_ABOVE_GROUND_1,
            LAYER_ABOVE_GROUND_2,
            LAYER_ABOVE_GROUND_3,
        ];
        let mut index = 0;
        for layer_name in render_layer_names {
            let layer = tiled_map
                .get_layer_by_name(&String::from(layer_name))
                .unwrap();
            let mut tiles: HashMap<(i32, i32), u32> = HashMap::new();
            for (y, row) in &layer.tiles {
                for (x, tile) in row {
                    if tile.gid != 0 {
                        tiles.insert((*x, *y), tile.gid);
                    }
                }
            }
            qom_map.render_layers.push(QomLayer {
                name: String::from(layer_name),
                tiles,
                layer_index: index,
            });
            index += 1;
        }

        portable_log!("x: {} to {}", qom_map.bounds.x1, qom_map.bounds.x2);
        portable_log!("y: {} to {}", qom_map.bounds.y1, qom_map.bounds.y2);

        return qom_map;
    }
    pub fn render(
        &self,
        // todo: Window isn't required, drawables can be drawn directly to context
        window: &mut Window,
        images: &HashMap<u32, Image>,
        view_position: Vector,
    ) {
        for layer in &self.render_layers {
            layer.render(window, images, view_position);
        }
    }
}
