use crate::qom::screens::Screen;

pub enum ScreenTransition {
    // Does no screen transition
    None,
    // Pops the current screen
    Pop,
    // Push a new screen onto the stack
    Push(Box<dyn Screen>, TransitionEffect),
    // Pops the current screen and puts a new screen onto the stack
    Replace(Box<dyn Screen>, TransitionEffect),
}

pub enum TransitionEffect {
    // No transition effect between screens
    None,
}
