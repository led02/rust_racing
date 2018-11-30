

extern crate quicksilver;

use quicksilver::{
    lifecycle::{Window}
};

use ::{Game};

pub struct JanUpdateStruct {
    pub test:i32,
}

pub fn init() ->JanUpdateStruct{
    JanUpdateStruct{test:0}
}


pub fn update(_window: &mut Window, _game: &mut Game){

}

pub fn draw(_window: &mut Window, _game: &mut Game) {

}