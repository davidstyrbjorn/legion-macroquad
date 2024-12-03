pub use crate::prelude::*; // Bring in our prelude stuff

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub color: Color,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdleBounce {
    pub dx: f32,
    pub dy: f32,
}

pub struct Text {
    pub text: String,
    pub size: f32,
    pub color: Color,
}

pub type Callback = fn();
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Button {
    pub cb: Option<Callback>,
    pub background: Color,
    pub on_hover: Color,
}
