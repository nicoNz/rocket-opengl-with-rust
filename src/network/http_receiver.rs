

use std::sync::mpsc::{Sender, channel, Receiver};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

type SendSyncSender = Arc<Mutex<Sender<Msg>>>;

fn target_to_enum(target_name: &rocket::http::RawStr)->Result<TARGET, ()> {
    match target_name.as_str() {
        "camera" => Ok(TARGET::CAMERA),
        "model" => Ok(TARGET::MODEL),
        _ => Err(())
    }
}

fn param_to_enum(param_name: &rocket::http::RawStr)->Result<PARAM, ()> {
    match param_name.as_str() {
        "x" => Ok(PARAM::X),
        "y" => Ok(PARAM::Y),
        _ => Err(())
    }
}


#[get("/<some_target>/<some_param>/<some_value>")]
fn index(some_target: &rocket::http::RawStr, some_param: &rocket::http::RawStr, some_value: f32, sender: rocket::State<SendSyncSender>) -> &'static str {

    match (target_to_enum(some_target) , param_to_enum(some_param)) {
        (Ok(some_target), Ok(some_param)) => {
            if !sender.lock().unwrap().send(Msg {
                target : some_target,
                param : some_param,
                value : some_value
            }).is_ok() {
                println!("sending the message failed" )
            }
        }
        _ => println!("some cast fail" )
    }
    "Hello, world!"
}


pub enum TARGET {
    MODEL,
    CAMERA
}

pub enum PARAM {
    X,
    Y
}


pub struct Msg {
    pub target: TARGET,
    pub param: PARAM,
    pub value: f32
}

pub fn launch_http() -> Receiver<Msg> {
    let (sender, receiver) = channel::<Msg>();
    let thread_safe_sender = Arc::new(Mutex::new(sender));
    thread::spawn(|| {
        rocket::ignite().manage(thread_safe_sender).mount("/", routes![index]).launch();
    });
    return receiver;
}
