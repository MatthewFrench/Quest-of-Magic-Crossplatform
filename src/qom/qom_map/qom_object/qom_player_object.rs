use crate::qom::qom_data::QuestOfMagicData;
use crate::qom::qom_map::{QomMap, TILE_HEIGHT, TILE_WIDTH};
use nalgebra::Point2;
use quicksilver::geom::Vector;
use quicksilver::graphics::Background::Img;
use quicksilver::prelude::Shape;
use quicksilver::prelude::{Image, Window};
use std::collections::HashMap;
use std::f32::consts::FRAC_1_SQRT_2;
use std::time::{Duration, Instant};

const PIXEL_MOVE_SPEED_PER_SECOND_MAX_SPEED: f32 = TILE_WIDTH as f32 * 10.0;
const PIXEL_MOVE_SPEED_PER_SECOND_MIN_SPEED: f32 = TILE_WIDTH as f32 * 2.0;
const PIXEL_REALIGNMENT_SPEED_PER_SECOND: f32 = TILE_WIDTH as f32;
const PLAYER_ACCELERATION_SPEED: f32 = (TILE_WIDTH as f32) / 45.0;

#[derive(PartialEq, Eq, Clone)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    DownLeft,
    UpRight,
    DownRight,
    None,
}

// Todo: Switch everything to f64

pub struct QomPlayerObject {
    is_visible: bool,
    name: String,
    pixel_position: Point2<f32>,
    image_id: u32,
    previous_moved_direction: MoveDirection,
    current_desired_direction: MoveDirection,
    current_direction_speed: f32,
}
impl Default for QomPlayerObject {
    fn default() -> QomPlayerObject {
        QomPlayerObject {
            is_visible: true,
            name: "".parse().unwrap(),
            pixel_position: Point2::new(0.0, 0.0),
            image_id: 0,
            previous_moved_direction: MoveDirection::None,
            current_desired_direction: MoveDirection::None,
            current_direction_speed: 0.0,
        }
    }
}
impl QomPlayerObject {
    pub fn new() -> QomPlayerObject {
        QomPlayerObject {
            ..Default::default()
        }
    }
    pub fn set_image_id(&mut self, image_id: u32) {
        self.image_id = image_id;
    }
    pub fn set_tile_position(&mut self, position: Point2<f32>) {
        self.pixel_position.x = position.x * TILE_WIDTH as f32;
        self.pixel_position.y = position.y * TILE_HEIGHT as f32;
    }
    pub fn move_direction(&mut self, direction: MoveDirection) {
        self.current_desired_direction = direction;
    }
    fn can_move_to_x_position(&mut self, map: &QomMap, x: f32) -> bool {
        let y1_tile = self.get_tile_y_fraction().floor() as i32;
        let y2_tile = self.get_tile_y_fraction().ceil() as i32;
        // Collision is a rectangle so we have to check both sides
        // In the future this could contain movement direction
        let x1_tile = (x / TILE_WIDTH as f32).floor() as i32;
        let x2_tile = (x / TILE_WIDTH as f32).ceil() as i32;
        return !map.collision_tiles.contains(&(x1_tile, y1_tile))
            && !map.collision_tiles.contains(&(x2_tile, y2_tile));
    }
    fn can_move_to_y_position(&mut self, map: &QomMap, y: f32) -> bool {
        let x1_tile = self.get_tile_x_fraction().floor() as i32;
        let x2_tile = self.get_tile_x_fraction().ceil() as i32;
        let y1_tile = (y / TILE_HEIGHT as f32).floor() as i32;
        let y2_tile = (y / TILE_HEIGHT as f32).ceil() as i32;
        return !map.collision_tiles.contains(&(x1_tile, y1_tile))
            && !map.collision_tiles.contains(&(x2_tile, y2_tile));
    }
    fn can_move_to_x_y_position(&mut self, map: &QomMap, x: f32, y: f32) -> bool {
        let x1_tile = (x / TILE_WIDTH as f32).floor() as i32;
        let x2_tile = (x / TILE_WIDTH as f32).ceil() as i32;
        let y1_tile = (y / TILE_HEIGHT as f32).floor() as i32;
        let y2_tile = (y / TILE_HEIGHT as f32).ceil() as i32;
        return !map.collision_tiles.contains(&(x1_tile, y1_tile))
            && !map.collision_tiles.contains(&(x2_tile, y2_tile));
    }
    /**
    Move character if moving.
    */
    pub fn update(&mut self, map: &QomMap) {
        // Todo: Track time since last update to move the player a delta amount

        // Todo: If a player taps a movement key, move to the next tile.

        // Convert direction to vector
        let move_to: Point2<f32> = match self.current_desired_direction {
            MoveDirection::Left => Point2::new(-1.0, 0.0),
            MoveDirection::Right => Point2::new(1.0, 0.0),
            MoveDirection::Up => Point2::new(0.0, -1.0),
            MoveDirection::Down => Point2::new(0.0, 1.0),
            MoveDirection::UpLeft => Point2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            MoveDirection::DownLeft => Point2::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            MoveDirection::UpRight => Point2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            MoveDirection::DownRight => Point2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            MoveDirection::None => Point2::new(0.0, 0.0),
        };
        // Reduce speed on turns
        if self.previous_moved_direction != self.current_desired_direction
            && !(move_to.x == 0.0 && move_to.y == 0.0)
        {
            let reduction_factor = match (
                self.previous_moved_direction.clone(),
                self.current_desired_direction.clone(),
            ) {
                (MoveDirection::Left, MoveDirection::DownLeft)
                | (MoveDirection::DownLeft, MoveDirection::Down)
                | (MoveDirection::Down, MoveDirection::DownRight)
                | (MoveDirection::DownRight, MoveDirection::Right)
                | (MoveDirection::Right, MoveDirection::UpRight)
                | (MoveDirection::UpRight, MoveDirection::Up)
                | (MoveDirection::Up, MoveDirection::UpLeft)
                | (MoveDirection::UpLeft, MoveDirection::Left)
                | (MoveDirection::Left, MoveDirection::UpLeft)
                | (MoveDirection::UpLeft, MoveDirection::Up)
                | (MoveDirection::Up, MoveDirection::UpRight)
                | (MoveDirection::UpRight, MoveDirection::Right)
                | (MoveDirection::Right, MoveDirection::DownRight)
                | (MoveDirection::DownRight, MoveDirection::Down)
                | (MoveDirection::Down, MoveDirection::DownLeft)
                | (MoveDirection::DownLeft, MoveDirection::Left) => FRAC_1_SQRT_2,
                (MoveDirection::Left, MoveDirection::Down)
                | (MoveDirection::Down, MoveDirection::Right)
                | (MoveDirection::Right, MoveDirection::Up)
                | (MoveDirection::Up, MoveDirection::Left)
                | (MoveDirection::Left, MoveDirection::Up)
                | (MoveDirection::Up, MoveDirection::Right)
                | (MoveDirection::Right, MoveDirection::Down)
                | (MoveDirection::Down, MoveDirection::Left) => 0.5,
                _ => 0.0,
            };
            self.current_direction_speed *= reduction_factor;
        }
        self.previous_moved_direction = self.current_desired_direction.clone();
        // Increase speed and move player
        if move_to.x != 0.0 || move_to.y != 0.0 {
            self.current_direction_speed += PLAYER_ACCELERATION_SPEED;
            self.current_direction_speed = self
                .current_direction_speed
                .max(PIXEL_MOVE_SPEED_PER_SECOND_MIN_SPEED);
            self.current_direction_speed = self
                .current_direction_speed
                .min(PIXEL_MOVE_SPEED_PER_SECOND_MAX_SPEED);
            // Check collisions
            let new_x_position =
                self.pixel_position.x + (move_to.x * self.current_direction_speed / 60.0);
            let new_y_position =
                self.pixel_position.y + (move_to.y * self.current_direction_speed / 60.0);
            let can_move_x = self.can_move_to_x_position(map, new_x_position);
            let can_move_y = self.can_move_to_y_position(map, new_y_position);
            let can_move_both = self.can_move_to_x_y_position(map, new_x_position, new_y_position);
            if can_move_x {
                self.pixel_position.x = new_x_position;
            }
            if can_move_y && (!can_move_x || can_move_both) {
                self.pixel_position.y = new_y_position;
            }
        } else {
            self.current_direction_speed = 0.0;
        }
        /*
        // Center the player on the tile if not moving
        if move_to.x == 0.0 && move_to.y != 0.0 {
            // Todo split tile -> pixel and pixel -> tile conversion into utility functions
            let current_tile_x_pixel_position = (self.get_tile_x() * TILE_WIDTH) as f32;
            if (self.pixel_position.x - current_tile_x_pixel_position as f32).abs()
                < PIXEL_REALIGNMENT_SPEED_PER_SECOND / 60.0
            {
                self.pixel_position.x = current_tile_x_pixel_position;
            } else if self.pixel_position.x > current_tile_x_pixel_position {
                self.pixel_position.x -= PIXEL_REALIGNMENT_SPEED_PER_SECOND / 60.0;
            } else if self.pixel_position.x < current_tile_x_pixel_position {
                self.pixel_position.x += PIXEL_REALIGNMENT_SPEED_PER_SECOND / 60.0;
            }
        }
        if move_to.y == 0.0 && move_to.x != 0.0 {
            // Todo split tile -> pixel and pixel -> tile conversion into utility functions
            let current_tile_y_pixel_position = (self.get_tile_y() * TILE_HEIGHT) as f32;
            if (self.pixel_position.y - current_tile_y_pixel_position as f32).abs()
                < PIXEL_REALIGNMENT_SPEED_PER_SECOND / 60.0
            {
                self.pixel_position.y = current_tile_y_pixel_position;
            } else if self.pixel_position.y > current_tile_y_pixel_position {
                self.pixel_position.y -= PIXEL_REALIGNMENT_SPEED_PER_SECOND / 60.0;
            } else if self.pixel_position.y < current_tile_y_pixel_position {
                self.pixel_position.y += PIXEL_REALIGNMENT_SPEED_PER_SECOND / 60.0;
            }
        }
        */
    }
    pub fn get_pixel_x(&self) -> f32 {
        self.pixel_position.x
    }
    pub fn get_pixel_y(&self) -> f32 {
        self.pixel_position.y
    }
    fn get_tile_x_fraction(&self) -> f32 {
        self.get_pixel_x() / TILE_WIDTH as f32
    }
    pub fn get_tile_x(&self) -> i32 {
        (self.get_tile_x_fraction()).round() as i32
    }
    fn get_tile_y_fraction(&self) -> f32 {
        self.get_pixel_y() / TILE_HEIGHT as f32
    }
    pub fn get_tile_y(&self) -> i32 {
        (self.get_tile_y_fraction()).round() as i32
    }
    pub fn render(
        &self,
        // todo: Window isn't required, drawables can be drawn directly to context
        window: &mut Window,
        images: &HashMap<u32, Image>,
        view_position: Vector,
    ) {
        if self.image_id > 0 {
            let image = images.get(&self.image_id);
            if let Some(image) = image {
                window.draw(
                    &image.area().translate((
                        self.get_pixel_x() - view_position.x,
                        self.get_pixel_y() - view_position.y,
                    )),
                    Img(image),
                );
            }
        } else {
            portable_log!("Unable to find image with image id {}", self.image_id);
        }
    }
}
