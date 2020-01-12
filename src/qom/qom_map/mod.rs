use crate::qom::qom_map::qom_layer::QomLayer;
use crate::qom::qom_map::qom_object::{QomObject, QomUnknownObject};
use crate::qom::tiled::Map;
use crate::qom::utility::Bounds;
use quicksilver::geom::Vector;
use quicksilver::prelude::{Image, Window};
use std::collections::{HashMap, HashSet};

pub mod qom_layer;
pub mod qom_object;

const TILE_WIDTH: i32 = 44;
const TILE_HEIGHT: i32 = 44;
const LAYER_GROUND_1: &str = "Ground 1";
const LAYER_GROUND_2: &str = "Ground 2";
const LAYER_ABOVE_GROUND_1: &str = "Above Ground 1";
const LAYER_ABOVE_GROUND_2: &str = "Above Ground 2";
const LAYER_ABOVE_GROUND_3: &str = "Above Ground 3";
const LAYER_TILE_COLLISION_MAP: &str = "Tile Collision Map";
const OBJECT_LAYER_NPCS_AND_INTERACTIONS: &str = "NPCs and Interactions";

pub struct QomMap {
    bounds: Bounds<i32>,
    render_layers: Vec<QomLayer>,
    collision_tiles: HashSet<(i32, i32)>,
    // todo: split into object types, signs/houses/npcs/entrances
    interactive_objects: Vec<Box<dyn QomObject>>,
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
            interactive_objects: vec![],
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

        // Create interact-able objects
        let object_layer = tiled_map
            .get_object_group_by_name(&String::from(OBJECT_LAYER_NPCS_AND_INTERACTIONS))
            .unwrap();
        for object in &object_layer.objects {
            let interactable_object = QomUnknownObject {
                x: object.x as i32,
                y: object.y as i32 - TILE_HEIGHT, // Objects are rendered using the bottom left corner as 0,0
                name: object.name.clone(),
                object_type: object.obj_type.clone(),
                image_id: object.gid,
            };
            qom_map
                .interactive_objects
                .push(Box::new(interactable_object));
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
        // Render world
        for layer in &self.render_layers {
            layer.render(window, images, view_position);
        }
        // Render interact-able objects
        for object in &self.interactive_objects {
            object.render(window, images, view_position);
        }
    }
}