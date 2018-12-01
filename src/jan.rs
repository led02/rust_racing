

extern crate quicksilver;

use quicksilver::{
    lifecycle::{Window}
};
use std::net::UdpSocket;

use ::{Game};


pub struct JanUpdateStruct {
    pub test: i32,
    socket:UdpSocket,
}

pub fn init() ->JanUpdateStruct{
    let mut bind_socket = UdpSocket::bind("127.0.0.1:34254");

    JanUpdateStruct{
        test:0,
        socket : bind_socket.unwrap(),
    }

}


pub fn update(_window: &mut Window, _game: &mut Game){
    let mut buf = [0; 10];
    let (amt, src) = _game._jan_update_struct.socket.recv_from(&mut buf);
}

pub fn draw(_window: &mut Window, _game: &mut Game) {

}