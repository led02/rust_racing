extern crate quicksilver;

use quicksilver::{
    lifecycle::{Window},
    input::{Key}
};

use ::{Game};

pub struct RacingCarUpdateStruct {
    pub speed:double,
    pub direction:(double, double),
    pub position:(double,double),
    pub steering: double,

}

pub fn init() ->RacingCarUpdateStruct{
    RacingCarUpdateStruct{  speed:0,
                            direction:(0,0),
                            position:(100,100),
                            steering:0,
                            }
}

pub fn update(_window: &mut Window, _game: &mut Game){
    let car = &mut _game._racingcar_update_struct;
    //TODO: move the below (partially) to input_manager.rs
    if window.keyboard()[Key::Left].is_down() {
        car.steering = car.steering +0.1;
        if car.steering > 1{
            car.steering = 1;
        }
    }
    if window.keyboard()[Key::Right].is_down() {
        car.steering = car.steering -0.1;
        if car.steering < -1{
            car.steering = -1;
        }
    }
    if window.keyboard()[Key::Down].is_down() {
        car.speed = car.speed -0.1;
        if car.speed < -10{
            car.speed = -10;
        }
    }
    if window.keyboard()[Key::Up].is_down() {
        car.speed = car.speed +0.1;
        if car.speed > 10{
            car.speed = 10;
        }
    }

    //move/turn the car:
    //TODO continue here =)

}

pub fn draw(_window: &mut Window, _game: &mut Game) {

}

