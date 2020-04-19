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
use render::shader::{
    Shader,
    Program,
    UniformTypedValue,
    UniformRole
};

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
    m: GLint,
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
                            UniformTypedValue::Mat4(Box::new(
                                camera.get_view_projection()
                            ))
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

        use std::ffi::CString;
        let vert_shader =
            Shader::from_vert_source(&gl ,&CString::new(include_str!("triangle.vert")).unwrap())
                .unwrap();
    
        let frag_shader =
            Shader::from_frag_source(&gl, &CString::new(include_str!("triangle.frag")).unwrap())
                .unwrap();
    
        let mut program = Program::from_shaders(&gl, &[vert_shader, frag_shader]).unwrap();

        let m = program.register_uniform(
            &String::from("M"),
            UniformTypedValue::Mat4(Box::new(glm::identity::<f32, glm::U4>())),
            UniformRole::Transform
        );

        let vp = program.register_uniform(
            &String::from("VP"),
            UniformTypedValue::Mat4(Box::new(glm::identity::<f32, glm::U4>())),
            UniformRole::Camera
        );
    
        program.set_used();
        
        program.set_uniform(vp, UniformTypedValue::Mat4(Box::new(camera.get_view_projection())));
    
        let mesh = match get_array_data() {
            Ok(ref descr) => {
                Mesh::from_description(&gl, descr, Some(Box::new(program)))
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
                m,
                vp
            }
        )
    });
    app_runner.run_loop();
}