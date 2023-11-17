use std::{thread, time::Duration};

use devices_generated::teste::{TypeC, TypeCArgs, Any, BaseArgs, Base};
use flatbuffers::FlatBufferBuilder;
use zmq::{SocketType::PUB, Context};

const SERVER_ADDRESS: &str = "tcp://127.0.0.1:5555";

mod devices_generated;

fn main() {
    let mut builder = FlatBufferBuilder::new();

    //Criação dos dados da mensagem base
    let data = builder.create_string("dummy data 2");
    let name = builder.create_string("generic name");
    let id = 1;
    //Criação do device
    let device = TypeC::create(&mut builder, &TypeCArgs{data: Some(data)});

    //Criação da mensagem base e conversão em um vetor de bytes
    let base = Base::create(&mut builder, &BaseArgs{ id:id 
        , name: Some(name), device: Some(device.as_union_value()), device_type: Any(3)  });

    builder.finish(base, None);
    let buf: &[u8] = builder.finished_data();

    //Envio da mesma em loop usando o zeromq
    let context = Context::new();
    let publisher = context.socket(PUB).unwrap();
    assert!(publisher.bind(SERVER_ADDRESS).is_ok());

    loop {
        publisher.send(&buf, 0).unwrap();
        thread::sleep(Duration::from_secs(5));
    }
}
