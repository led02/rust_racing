extern crate quicksilver;

use quicksilver::{
    lifecycle::{Window}
};

pub struct JanUpdateStruct {
    pub test:i32,
}

pub fn init() ->JanUpdateStruct{
    JanUpdateStruct{test:0}
}

pub fn update(window: &mut Window){
	
}
