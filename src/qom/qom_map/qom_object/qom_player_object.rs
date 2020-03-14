use crate::qom::qom_map::{TILE_HEIGHT, TILE_WIDTH};
use nalgebra::Point2;
use quicksilver::geom::Vector;
use quicksilver::graphics::Background::Img;
use quicksilver::prelude::Shape;
use quicksilver::prelude::{Image, Window};
use std::collections::HashMap;
use std::time::{Duration, Instant};

const TILE_MOVE_TIME_IN_SECONDS: f64 = 0.2;

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

// Todo: Queue a movement direction instead of queueing a player move to tile

pub struct MovementCommand {
    moving_start_tile: Point2<f32>,
    move_to_tile: Point2<f32>,
    transition_total_time: f64,
    transition_start: Instant,
    ease_out: bool,
    ease_in: bool,
}

pub struct QomPlayerObject {
    is_visible: bool,
    name: String,
    tile_position: Point2<f32>,
    image_id: u32,
    // Tracks if the character is moving
    movement_commands: Vec<MovementCommand>,
}
impl Default for QomPlayerObject {
    fn default() -> QomPlayerObject {
        QomPlayerObject {
            is_visible: true,
            name: "".parse().unwrap(),
            tile_position: Point2::new(0.0, 0.0),
            image_id: 0,
            movement_commands: vec![],
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
        self.movement_commands.clear();
        self.tile_position = position.clone();
    }
    fn is_moving(&self) -> bool {
        self.movement_commands.len() > 0
    }
    fn move_to(&mut self, to_tile: Point2<f32>) {
        if self.movement_commands.len() == 0 {
            self.movement_commands.push(MovementCommand {
                moving_start_tile: self.tile_position.clone(),
                move_to_tile: to_tile,
                transition_total_time: TILE_MOVE_TIME_IN_SECONDS,
                transition_start: Instant::now(),
                ease_out: false,
                ease_in: false,
            });
        }
    }
    fn cancel_next_movement_command(&mut self) {
        // Todo, cancel next movement command assuming it is from the player
    }
    pub fn move_direction(&mut self, direction: MoveDirection) {
        let mut move_to: Point2<f32> = match direction {
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
        if move_to.x == 0.0 && move_to.y == 0.0 {
            self.cancel_next_movement_command();
        } else {
            move_to = Point2::new(
                (move_to.x + self.tile_position.x).round(),
                (move_to.y + self.tile_position.y).round(),
            );
            self.move_to(move_to);
        }

        /*
        if self.is_moving() {

        } else {

        }

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

        let elapsed_duration = self.transition_start.elapsed();
        // Percentage can likely be ease-in ease-out using trigonometry of some sort
        let mut percentage = elapsed_duration.as_secs_f64() / self.transition_total_time;
        if !self.is_moving {
            // Todo Determine if next area is blocked by collisions

            self.is_moving = true;
            self.moving_starting_x = self.x;
            self.moving_starting_y = self.y;
            self.move_to_x = self.x + move_x_amount;
            self.move_to_y = self.y + move_y_amount;
            self.transition_total_time = TILE_MOVE_TIME_IN_SECONDS;
            self.transition_start = Instant::now();
        } else if self.is_moving && same_direction && percentage > 0.995 {
            // Todo Determine if next area is blocked by collisions

            // Start on next movement command
            self.move_to_x += move_x_amount;
            self.move_to_y += move_y_amount;
            self.transition_total_time += TILE_MOVE_TIME_IN_SECONDS;
        }
        */
    }
    /**
    Move character if moving.
    */
    pub fn update(&mut self) {
        if self.is_moving() {
            let command = self.movement_commands.first_mut().unwrap();
            let elapsed_duration = command.transition_start.elapsed();
            // Percentage can likely be ease-in ease-out using trigonometry of some sort
            let mut percentage = elapsed_duration.as_secs_f64() / command.transition_total_time;
            if percentage >= 1.0 {
                percentage = 1.0;
                self.tile_position = command.move_to_tile.clone();
                self.movement_commands.remove(0);
            } else {
                self.tile_position = Point2::new(
                    (command.move_to_tile.x - command.moving_start_tile.x) * percentage as f32
                        + command.moving_start_tile.x,
                    (command.move_to_tile.y - command.moving_start_tile.y) * percentage as f32
                        + command.moving_start_tile.y,
                );
            }
        }
    }
    pub fn get_pixel_x(&self) -> f32 {
        self.tile_position.x * TILE_WIDTH as f32
    }
    pub fn get_pixel_y(&self) -> f32 {
        self.tile_position.y * TILE_HEIGHT as f32
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
