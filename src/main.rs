#![feature(proc_macro_hygiene, decl_macro)]
#![feature(vec_into_raw_parts)]
#[macro_use] extern crate rocket;

extern crate gl;
extern crate sdl2;
extern crate nalgebra_glm as glm;
extern crate json;

pub mod render;
pub mod camera;
pub mod json_parser;
pub mod network;
pub mod window_app;

use render::mesh::Mesh;
use render::shader::{
    Shader,
    Program
};

use network::http_receiver::{
    launch_http,
    PARAM,
    TARGET
};
use window_app::init;

fn main() {

    let receiver = &launch_http();

   let (gl, sdl, window, _gl_context) = init();

    let mut camera = camera::Camera::from_position_and_look_at(&glm::vec3(-6.0,0.0, 5.0), &glm::vec3(0., 0., 0.));


    use std::ffi::CString;
    let vert_shader =
        Shader::from_vert_source(&gl ,&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        Shader::from_frag_source(&gl, &CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let mut shader_program = Program::from_shaders(&gl, &[vert_shader, frag_shader]).unwrap();

    shader_program.set_used();
    shader_program.u_vp_value = camera.get_view_projection();

    let mut mesh = match json_parser::get_array_data() {
        Ok(ref descr) => {
            Mesh::from_description(&gl, descr, Some(shader_program))
        },
        Err(e) => {
            panic!("fail to get buffers");
        }
    };


    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in receiver.try_iter() {
            match event.target {
                TARGET::CAMERA => {
                    match event.param {
                        PARAM::X => camera.set_position_x(event.value),
                        PARAM::Y => camera.set_position_y(event.value)
                    };
                    match mesh.program {
                        Some(ref mut program) => {
                            program.u_vp_value = camera.get_view_projection();
                        }
                        None => println!("no bound material")
                    }
                }
                TARGET::MODEL => {
                    match mesh.program {
                        Some(ref mut program) => {
                            program.set_offset(event.value);
                        }
                        None => println!("no bound material")
                    }
                }
            }
        }
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseMotion {x, .. } => {
                    let v = x as f32;
                    match mesh.program {
                        Some(ref mut program) => {
                            program.set_offset(v);
                        }
                        None => println!("no bound material")
                    }
                },
                _ => {}
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }
        
        mesh.draw();

        window.gl_swap_window();
    }
}