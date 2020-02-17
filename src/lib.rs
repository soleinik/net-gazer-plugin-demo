#[macro_use] extern crate log;
extern crate net_gazer_core as core;

use core::*;
use pnet::packet::ethernet::EthernetPacket;


const ID:u8=10;
const NAME:&str="Test plugin";


pub struct Test;


impl <'p, 'd> Plugin<'p, EthernetPacket<'d>> for Test{

    fn get_name(&self)->&str{NAME}

    fn get_id(&self) -> u8 {ID}
 
    fn on_load(&self){
        env_logger::init();

        error!("Hello from [{}] \"{}\"! ", ID, NAME);

    }
    fn on_unload(&self){
        error!("Good bye from [{}] \"{}\" ", ID, NAME);
    }

    fn process(&self, _tx:&'_ CoreSender, _pkt:&'_ EthernetPacket){
        error!("plugin processing with [{}] \"{}\"", ID, NAME);
    }

}


#[no_mangle]
pub extern "C" fn net_gazer_plugin_new<'p,'d> () -> * mut dyn Plugin<'p, EthernetPacket<'d>>{
     let boxed:Box<Test> = Box::new(Test{});
     Box::into_raw(boxed)
}



