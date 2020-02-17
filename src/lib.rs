#[macro_use] extern crate log;
extern crate net_gazer_core as core;

use core::*;
use pnet::packet::ethernet::EthernetPacket;


const ID:u8=10;
const DESCRIPTION:&str="Test plugin";


pub struct Test;


impl <'a> Plugin<'a> for Test{
    fn get_description()->String{ DESCRIPTION.into()}
    fn get_id() -> u8 {ID}
   
    fn process(_tx:&'a CoreTxSender, _pkt:&'a EthernetPacket){
        error!("plugin processing with [{}] {}", ID, DESCRIPTION);

    }
}
