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

//use file::json_parser ;

use gl::types::GLint;
use file::vbo_description_parser::get_array_data;

struct App {
    mesh: Mesh,
    camera: Camera,
    receiver: std::sync::mpsc::Receiver<network::http_receiver::Msg>,
    vp: GLint
    //program: Option<Box<Program>>
}

impl WindowApp for App {

    fn update(&mut self) {
        let receiver = &self.receiver;
        let camera = &mut self.camera;
        
        for event in receiver.try_iter() {
            match event.target {
                TARGET::CAMERA => {
                    match event.param {
                        PARAM::X => camera.set_position_x(event.value),
                        PARAM::Y => camera.set_position_y(event.value)
                    };
                    match &mut self.mesh.program {
                        Some(program) => program.set_uniform(
                            self.vp,
                            UniformValue::Mat4(camera.get_view_projection())
                        ),
                        _ => println!("err")
                    }
                }
                TARGET::MODEL => {
                    // match &mut self.mesh.program {
                    //     Some(program)=>program.set_offset(event.value),
                    //     _ => println!("err")
                    // }
                }
            }
        }
    }

    fn draw(&self) {

        self.mesh.draw();
    }
    fn on_window_event(&mut self, event: &sdl2::event::Event) {
        
        match event {
            sdl2::event::Event::MouseMotion {x, .. } => {
                // let v = *x as f32;
                // match &mut self.mesh.program {
                //     Some(program)=>program.set_offset(v),
                //     _ => println!("err")
                // }
            },
            _ => {}
        }
    }
}
fn main() {

    let receiver = launch_http();

    let mut app_runner =  WindowAppRunner::new( move |gl: &gl::Gl| {
        let camera = camera::Camera::from_position_and_look_at(&glm::vec3(-6.0,0.0, 5.0), &glm::vec3(0., 0., 0.));

        let shader = match Shader::from_json(gl, &String::from("my_shader.json")) {
            Ok(shader) => shader,
            Err(e) => {
                panic!("fail to create shader from json; Err : {}", e)
            } 
        };

        let key_map = shader.get_uniform_to_key_map();
        let vp = *key_map.get("VP").unwrap_or(&-1);
        if vp < 0 {
            panic!("VP not found is shader cause panic");
        }

        
        shader.use_shader();
        shader.set_uniform_value(vp, UniformValue::Mat4(camera.get_view_projection()));
    
        let mesh = match get_array_data(String::from("vertdata.json")) {
            Ok(ref descr) => {
                // TODO => Mesh should use the shader description for double checking
                Mesh::from_description(&gl, descr, Some(Box::new(shader.program)))
            },
            Err(e) => {
                panic!("fail to get buffers");
            }
        };

        Box::new(
            App {
                camera : Camera::from_position_and_look_at(&glm::vec3(-6.0,0.0, 5.0), &glm::vec3(0., 0., 0.)),
                receiver,
                mesh,
                vp
            }
        )
    });
    app_runner.run_loop();
}