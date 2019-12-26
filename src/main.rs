// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run},
};

struct DrawGeometry {
    pub x: f64
}

impl State for DrawGeometry {
    fn new() -> Result<DrawGeometry> {
        Ok(DrawGeometry{x: 0.0})
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.x += 0.01;
        window.clear(Color::WHITE)?;
        window.draw(&Rectangle::new((100 + self.x as i32, 100), (32, 32)), Col(Color::BLUE));
        window.draw_ex(&Rectangle::new((400 + self.x as i32, 300), (32, 32)), Col(Color::BLUE), Transform::rotate(45), 10);
        window.draw(&Circle::new((400 + self.x as i32, 300), 100), Col(Color::GREEN));
        window.draw_ex(
            &Line::new((50 + self.x as i32, 80),(600 + self.x as i32, 450)).with_thickness(2.0),
            Col(Color::RED),
            Transform::IDENTITY,
            5
        );
        window.draw_ex(
            &Triangle::new((500 + self.x as i32, 50), (450 + self.x as i32, 100), (650 + self.x as i32, 150)),
            Col(Color::RED),
            Transform::rotate(45) * Transform::scale((0.5, 0.5)),
            0
        );
        Ok(())
    }
}

fn main() {
    run::<DrawGeometry>("Draw Geometry", Vector::new(800, 600), Settings::default());
}