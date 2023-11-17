use devices_generated::teste::{Any};
use zmq::{SocketType::SUB, Context};

mod devices_generated;

const SERVER_ADDRESS: &str = "tcp://127.0.0.1:5555";

fn main() {
    
    let context = Context::new();
    let subscriber = context.socket(SUB).unwrap();
    assert!(subscriber.connect(SERVER_ADDRESS).is_ok());
    assert!(subscriber.set_subscribe(&[]).is_ok());

    loop {
        let mut buf = zmq::Message::new();
        subscriber.recv(&mut buf, 0).unwrap();
    //  DESEREALIZER
     let _msg = devices_generated::teste::root_as_base(&buf);
        let msg = _msg.unwrap();
        match msg.device_type() {
            Any::TypeA => println!("A {:?}",msg.device_as_type_a().unwrap()),
            Any::TypeB => println!("B {:?}",msg.device_as_type_b().unwrap()),
            Any::TypeC => println!("C {:?}",msg.device_as_type_c().unwrap()),
            Any::TypeD => println!("D {:?}",msg.device_as_type_d().unwrap()),
            x => { println!("{:?}", x)}
        }
    }
}
