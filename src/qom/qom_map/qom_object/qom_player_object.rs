use crate::qom::qom_map::{TILE_HEIGHT, TILE_WIDTH};
use quicksilver::geom::Vector;
use quicksilver::graphics::Background::Img;
use quicksilver::prelude::Shape;
use quicksilver::prelude::{Image, Window};
use std::collections::HashMap;
use std::time::{Duration, Instant};

const MOVEMENT_TIME_IN_SECONDS: f64 = 0.1;

pub struct QomPlayerObject {
    pub is_visible: bool,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub image_id: u32,
    // Tracks if the character is moving
    pub is_moving: bool,
    pub moving_starting_x: f32,
    pub moving_starting_y: f32,
    pub move_to_x: f32,
    pub move_to_y: f32,
    pub end_movement_time_in_seconds: f64,
    pub start_move_instant: Instant,
}
impl QomPlayerObject {
    /**
    Track player movement. Move by a set amount with a set speed. Only move if not previously moving.
    In a future iteration, might make better movement chaining.
    */
    pub fn move_amount(&mut self, move_x_amount: f32, move_y_amount: f32) {
        let elapsed_duration = self.start_move_instant.elapsed();
        // Percentage can likely be ease-in ease-out using trigonometry of some sort
        let mut percentage = elapsed_duration.as_secs_f64() / self.end_movement_time_in_seconds;
        let x_direction = if move_x_amount > 0.0 {
            1
        } else if move_x_amount < 0.0 {
            -1
        } else {
            0
        };
        let y_direction = if move_y_amount > 0.0 {
            1
        } else if move_y_amount < 0.0 {
            -1
        } else {
            0
        };
        let old_x_direction = if self.move_to_x - self.moving_starting_x > 0.0 {
            1
        } else if self.move_to_x - self.moving_starting_x < 0.0 {
            -1
        } else {
            0
        };
        let old_y_direction = if self.move_to_y - self.moving_starting_y > 0.0 {
            1
        } else if self.move_to_y - self.moving_starting_y < 0.0 {
            -1
        } else {
            0
        };
        let same_direction = x_direction == old_x_direction && y_direction == old_y_direction;
        if !self.is_moving {
            self.is_moving = true;
            self.moving_starting_x = self.x;
            self.moving_starting_y = self.y;
            self.move_to_x = self.x + move_x_amount;
            self.move_to_y = self.y + move_y_amount;
            self.end_movement_time_in_seconds = MOVEMENT_TIME_IN_SECONDS;
            self.start_move_instant = Instant::now();
        } else if self.is_moving && same_direction && percentage > 0.95 {
            // Start on next movement command
            self.move_to_x += move_x_amount;
            self.move_to_y += move_y_amount;
            self.end_movement_time_in_seconds += MOVEMENT_TIME_IN_SECONDS;
        }
    }
    /**
    Move character if moving.
    */
    pub fn update(&mut self) {
        if self.is_moving {
            let elapsed_duration = self.start_move_instant.elapsed();
            // Percentage can likely be ease-in ease-out using trigonometry of some sort
            let mut percentage = elapsed_duration.as_secs_f64() / self.end_movement_time_in_seconds;
            if percentage >= 1.0 {
                percentage = 1.0;
                self.is_moving = false;
                self.x = self.move_to_x;
                self.y = self.move_to_y;
            } else {
                self.x = (self.move_to_x - self.moving_starting_x) * percentage as f32
                    + self.moving_starting_x;
                self.y = (self.move_to_y - self.moving_starting_y) * percentage as f32
                    + self.moving_starting_y;
            }
        }
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
                    &image
                        .area()
                        .translate((self.x - view_position.x, self.y - view_position.y)),
                    Img(image),
                );
            }
        } else {
            portable_log!("Unable to find image with image id {}", self.image_id);
        }
    }
}
