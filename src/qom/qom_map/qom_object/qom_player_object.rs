use crate::qom::qom_map::{TILE_HEIGHT, TILE_WIDTH};
use nalgebra::Point2;
use quicksilver::geom::Vector;
use quicksilver::graphics::Background::Img;
use quicksilver::prelude::Shape;
use quicksilver::prelude::{Image, Window};
use std::cmp::max;
use std::collections::HashMap;
use std::time::{Duration, Instant};

const PIXEL_MOVE_SPEED_PER_SECOND_MAX_SPEED: f32 = TILE_WIDTH as f32 * 4.0;
const PIXEL_MOVE_SPEED_PER_SECOND_MIN_SPEED: f32 = PIXEL_MOVE_SPEED_PER_SECOND_MAX_SPEED / 4.0;
const PLAYER_ACCELERATION_SPEED: f32 =
    (PIXEL_MOVE_SPEED_PER_SECOND_MAX_SPEED - PIXEL_MOVE_SPEED_PER_SECOND_MIN_SPEED) / 60.0;

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
        self.pixel_position = Point2::new(
            position.x * TILE_WIDTH as f32,
            position.y * TILE_HEIGHT as f32,
        );
    }
    pub fn move_direction(&mut self, direction: MoveDirection) {
        self.current_desired_direction = direction;
    }
    /**
    Move character if moving.
    */
    pub fn update(&mut self) {
        let mut move_to: Point2<f32> = match self.current_desired_direction {
            MoveDirection::Left => Point2::new(-1.0, 0.0),
            MoveDirection::Right => Point2::new(1.0, 0.0),
            MoveDirection::Up => Point2::new(0.0, -1.0),
            MoveDirection::Down => Point2::new(0.0, 1.0),
            MoveDirection::UpLeft => Point2::new(-1.0, -1.0),
            MoveDirection::DownLeft => Point2::new(-1.0, 1.0),
            MoveDirection::UpRight => Point2::new(1.0, -1.0),
            MoveDirection::DownRight => Point2::new(1.0, 1.0),
            MoveDirection::None => Point2::new(0.0, 0.0),
        };
        // Todo: Change if complete direction change, make 0, if half direction change, cut speed in half
        if self.previous_moved_direction != self.current_desired_direction {
            self.current_direction_speed = 0.0;
        }
        self.previous_moved_direction = self.current_desired_direction.clone();
        if move_to.x != 0.0 || move_to.y != 0.0 {
            self.current_direction_speed += PLAYER_ACCELERATION_SPEED;
            self.current_direction_speed = self
                .current_direction_speed
                .max(PIXEL_MOVE_SPEED_PER_SECOND_MIN_SPEED);
            self.current_direction_speed = self
                .current_direction_speed
                .min(PIXEL_MOVE_SPEED_PER_SECOND_MAX_SPEED);
            move_to = Point2::new(
                move_to.x * self.current_direction_speed / 60.0 + self.pixel_position.x,
                move_to.y * self.current_direction_speed / 60.0 + self.pixel_position.y,
            );
            self.pixel_position = move_to;
        } else {
            self.current_direction_speed = 0.0;
        }
    }
    pub fn get_pixel_x(&self) -> f32 {
        self.pixel_position.x
    }
    pub fn get_pixel_y(&self) -> f32 {
        self.pixel_position.y
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
