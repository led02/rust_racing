

extern crate quicksilver;

use quicksilver::{
    lifecycle::{Window}
};
//use std::io::Result;
use std::net::UdpSocket;

use ::{Game};


pub struct JanUpdateStruct {
    pub test: u8,
    socket:UdpSocket,
}

//Spaghetticode: unwrap() several times instead of proper result handling

pub fn init() ->JanUpdateStruct{
    let mut bind_socket :UdpSocket = UdpSocket::bind("127.0.0.1:34254").unwrap();
    //TODO: include errorhandling for bind (e.g. loop while bind unsuccessful?)


    //testing purposes:

    bind_socket.send_to(&[1; 10], "127.0.0.1:34254");
    //return:
    JanUpdateStruct{
        test:0,
        socket : bind_socket,
    }

}


pub fn update(_window: &mut Window, _game: &mut Game){


    let mut buf = [1; 10];
    let (amt, src) = _game._jan_update_struct.socket.recv_from(&mut buf).unwrap();
    let buf = &mut buf[..amt];
    buf.reverse();
    _game._jan_update_struct.socket.send_to(buf, &src);
    _game._jan_update_struct.test = _game._jan_update_struct.test + buf[0];

}

pub fn draw(_window: &mut Window, _game: &mut Game) {

}