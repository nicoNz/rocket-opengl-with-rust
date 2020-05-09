
use crate::render::mesh::Mesh;
use crate::render::shader::Shader;
use crate::render::uniform::UniformValue;
use crate::network;
use crate::network::http_receiver::{
    launch_http,
    PARAM,
    TARGET
};
use crate::window_app::{
    WindowApp
};

use crate::camera::Camera;
use std::rc::Rc;
use std::cell::RefCell;

//use file::json_parser ;


use gl::types::GLint;
use crate::file::vbo_description_parser::get_array_data;

pub struct App {
    mesh: Mesh,
    camera: Camera,
    receiver: std::sync::mpsc::Receiver<network::http_receiver::Msg>,
    vp: GLint
}

impl App {
    pub fn new(gl: &gl::Gl  )-> Self {
        let receiver = launch_http();

        let camera = Camera::from_position_and_look_at(&glm::vec3(-6.0,0.0, 5.0), &glm::vec3(0., 0., 0.));

        let mut shader = match Shader::from_json(gl, &String::from("myshader.json")) {
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
        let luma = *key_map.get("luma").unwrap_or(&-1);
        if luma < 0 {
            panic!("luma not found is shader cause panic");
        }

        shader.use_shader();
        shader.set_uniform_value(vp, UniformValue::Mat4(camera.get_view_projection()));
        shader.set_uniform_value(luma, UniformValue::F32(0.7));
    

        let mesh = match get_array_data(String::from("vertdata.json")) {
            Ok(ref descr) => {
                // TODO => Mesh should use the shader description for double checking
                Mesh::from_description(&gl, descr, &Rc::new(RefCell::new(shader)))
            },
            Err(e) => {
                panic!("fail to get buffers");
            }
        };

        App {
            camera,
            receiver,
            mesh,
            vp
            
        }
    }
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
                    self.mesh.shader.borrow_mut().set_uniform_value(
                        self.vp,
                        UniformValue::Mat4(camera.get_view_projection())
                    );
                }
                TARGET::MODEL => {
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
            },
            _ => {}
        }
    }
}