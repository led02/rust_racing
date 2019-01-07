extern crate quicksilver;



use quicksilver::{
    Result,
    geom::{Circle, Rectangle, Transform},
    input::{Key},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run},
};

use ::{Game};

use mathhelper;

//representing a single RacingCar
pub struct RacingCarUpdateStruct {
    pub speed:f64,
    pub direction:f64,
    pub position:(u32,u32),
    pub steering: f64,

}

pub fn init() ->RacingCarUpdateStruct{
    RacingCarUpdateStruct{  speed:0.0,
                            direction:0.0,
                            position:(100,100),
                            steering:0.0,
                            }
}
impl State for RacingCarUpdateStruct {

    fn new() -> Result<RacingCarUpdateStruct> {
        Ok(init())
    }
     fn update(&mut self, _window: &mut Window) -> Result<()> {
        //let car = &mut _game._racingcar_update_struct;
        //TODO: move the below (partially) to input_manager.rs(?)
        if _window.keyboard()[Key::Left].is_down() {
            self.steering = self.steering + 0.1;
            if self.steering > 1.0 {
                self.steering = 1.0;
            }
        }
        if _window.keyboard()[Key::Right].is_down(){
//      for testing purposes:
//            let (x,y) = self.position;
//            self.position = (x+1, y);
////////////////////////////////////
            self.steering = self.steering - 0.1;
            if self.steering < -1.0 {
                self.steering = -1.0;
            }
        }
        if _window.keyboard()[Key::Down].is_down() {
            self.speed = self.speed - 0.1;
            if self.speed < -10.0 {
                self.speed = -10.0;
            }
        }
        if _window.keyboard()[Key::Up].is_down() {
            self.speed = self.speed + 0.1;
            if self.speed > 10.0 {
                self.speed = 10.0;
            }
        }


         //move/turn the car:
        //TODO continue here =)
         //simple stupid version:

         self.direction += self.steering/30.0;   //"spaceship controls"
         if self.direction > 3.1415 {
             self.direction = 3.1415;
         }
         else if self.direction < -3.1415{
             self.direction = -3.1415;
         }

         let dir = mathhelper::angleToDirectionVector(self.direction);
         let (dirx,diry) = dir;
         let (posx, posy) = self.position;
         let newx = dirx + posx as f64;
         let newy = diry + posy as f64;

         self.position = (newx as u32, newy as u32);

         Ok(())
    }

     fn draw(&mut self, _window: &mut Window) -> Result<()>{
        let (x,y) = self.position;
         _window.draw_ex(&Circle::new((x, y), 10), Col(Color::BLUE), Transform::rotate(45), 0);
         Ok(())
    }
}
