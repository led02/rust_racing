extern crate quicksilver;

use quicksilver::{
    geom::{Rectangle, Transform},
    graphics::{Background::Col, Color},
    lifecycle::{Window}
};

pub struct SimonUpdateStruct {
    pub x_pos: i32,
    pub y_pos: i32
}

pub fn update(window: &mut Window) {
    window.draw_ex(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE), Transform::rotate(45), 0);
}

pub fn init() ->SimonUpdateStruct{
    SimonUpdateStruct{x_pos: 0, y_pos:0}
}