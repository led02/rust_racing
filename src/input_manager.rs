

extern crate quicksilver;

use quicksilver::{
    lifecycle::{Window},
    input::{Key}
};

use ::{Game};

//functions that need a keybinding: (add more as needed) (?)
pub enum Keybinding {
    Accelerate, Decelerate, Turn_left, Turn_right,
}


pub struct InputUpdateStruct {
    pub test: u8,
    pub mapping:[Key; 4],
    pub pressed:[bool; 4]
}


pub fn init() ->InputUpdateStruct {
    InputUpdateStruct {
        test: 0,
        mapping:[   Key::Up,            //[Accelerate]
                    Key::Down,          //[Decelerate]
                    Key::Left,          //[Turn_left]
                    Key::Right],        //[Turn_right]
        pressed:[false, false, false, false],
    }
}



pub fn update(_window: &mut Window, _game: &mut Game){

    let mapping = &mut _game._input_update_struct.mapping;
    let pressed = &mut _game._input_update_struct.pressed;




    /*
    if _window.keyboard()[mapping[Keybinding::Turn_left]].is_down() {
        pressed[Keybinding::Turn_left] = true;
    }
    if _window.keyboard()[mapping[Keybinding::Turn_right]].is_down() {
        pressed[Keybinding::Turn_right] = true;
    }
    if _window.keyboard()[mapping[Keybinding::Accelerate]].is_down() {
        pressed[Keybinding::Accelerate] = true;
    }
    if _window.keyboard()[mapping[Keybinding::Turn_left]].is_down() {
        pressed[Keybinding::Accelerate] = true;
    }
    NOPE! :D*/
}

pub fn draw(_window: &mut Window, _game: &mut Game) {

}