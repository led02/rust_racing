extern crate quicksilver;

use quicksilver::{
    geom::{Rectangle, Transform},
    graphics::{Background::Col, Color, Image, Background::Img},
    lifecycle::{Window, Asset}
};

use ::{Game};

pub struct SimonUpdateStruct {
    pub x_pos: i32,
    pub y_pos: i32
}

pub fn update(_window: &mut Window, _game: &mut Game) {

}

pub fn init() ->SimonUpdateStruct{
    SimonUpdateStruct{x_pos: 0, y_pos:0}
}

pub fn draw(_window: &mut Window, _game: &mut Game) {
    _window.draw_ex(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE), Transform::rotate(45), 0);
}