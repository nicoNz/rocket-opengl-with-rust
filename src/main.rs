#![feature(proc_macro_hygiene, decl_macro)]
#![feature(vec_into_raw_parts)]
#[macro_use] extern crate rocket;

extern crate gl;
extern crate sdl2;
extern crate nalgebra_glm as glm;
extern crate json;

pub mod render;
pub mod camera;
pub mod network;
pub mod window_app;
pub mod parameter;
pub mod file;
pub mod apps;

pub use apps::checker::App; 

use window_app::{
    WindowAppRunner,
};

fn main() {

    let mut app_runner =  WindowAppRunner::new( move |gl: &gl::Gl| {
        Box::new(
            App::App::new(&gl) 
        )
    });
    app_runner.run_loop();
}