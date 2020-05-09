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

use render::mesh::Mesh;
use crate::render::shader::Shader;
use crate::render::uniform::UniformValue;

use network::http_receiver::{
    launch_http,
    PARAM,
    TARGET
};
use window_app::{
    WindowAppRunner,
    WindowApp
};

use camera::Camera;
use std::rc::Rc;
use std::cell::RefCell;

//use file::json_parser ;


use gl::types::GLint;
use file::vbo_description_parser::get_array_data;

fn main() {

    
    let mut app_runner =  WindowAppRunner::new( move |gl: &gl::Gl| {


        Box::new(
            App::App::new(&gl) 
        )
    });
    app_runner.run_loop();
}