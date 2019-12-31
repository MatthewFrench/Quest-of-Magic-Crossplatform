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
use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings},
};

fn main() {
    run::<QuestOfMagic>("Quest of Magic", Vector::new(800, 600), Settings::default());
}
