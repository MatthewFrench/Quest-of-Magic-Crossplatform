#[macro_use]
extern crate stdweb;
#[macro_use]
macro_rules! portable_log {
    ($($arg:tt)*) => ({
        #[cfg(target_arch = "wasm32")]
        console!(log, "{}", format!($($arg)*));
        #[cfg(not(target_arch = "wasm32"))]
        println!($($arg)*);
    })
}
extern crate quicksilver;
mod qom;

use crate::qom::QuestOfMagic;
use quicksilver::graphics::ResizeStrategy;
use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings},
};

fn main() {
    run::<QuestOfMagic>(
        "Quest of Magic",
        Vector::new(800, 600),
        Settings {
            resize: ResizeStrategy::Stretch,
            vsync: false,
            draw_rate: 1000.0 / 120.0,
            update_rate: 1000.0 / 240.0,
            multisampling: Some(2),
            max_updates: 10,
            ..Settings::default()
        },
    );
}
