extern crate quicksilver;

use quicksilver::{
    lifecycle::{Window}
};

pub struct RiekeUpdateStruct {
    pub test:i32
}

pub fn init() ->RiekeUpdateStruct{
    RiekeUpdateStruct{test:0}
}

pub fn update(window: &mut Window){
	
}



