
use crate::render::mesh::Mesh;
use crate::render::shader::Shader;
use crate::render::uniform::UniformValue;
use crate::window_app::{
    WindowAppRunner,
    WindowApp
};

use crate::camera::Camera;
use crate::render::vbo::{
    VboF32,
    VboU8
};
use std::rc::Rc;
use std::cell::RefCell;
use gl::types::GLint;

use super::gen_cylinder::gen_cylinder_data;

pub struct App {
    mesh: Mesh,
    camera: Camera,
    vp: GLint,
    gl: gl::Gl
}


impl App {
    pub fn new(gl: &gl::Gl  )-> Self {

        let camera = Camera::from_position_and_look_at(&glm::vec3(-6.0,3.0, 5.0), &glm::vec3(0., 0., 0.));

        let mut shader = match Shader::from_json(gl, &String::from("basicLight.json")) {
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
        
    
        let mut mesh = Mesh::new(gl, &Rc::new(RefCell::new(shader)));

        let (indicies, positions, norms) = gen_cylinder_data(0.2, 1.0, 16);

        let position = Box::new(VboF32::from_vector(&gl, &positions, 3));
        mesh.set_buffer_at_location(position, 0);

        let normal = Box::new(VboF32::from_vector(&gl, &norms, 3));
        mesh.set_buffer_at_location(normal, 1);
        
        let ibo = VboU8::from_vector(gl, &indicies, 1);
        mesh.set_indicies(ibo);

        App {
            camera,
            mesh,
            vp,
            gl: gl.clone()
        }
    }
}

impl WindowApp for App {

    fn update(&mut self) {
        //let camera = &mut self.camera;
    }

    fn draw(&self) {
        let gl = &self.gl;
        unsafe {
            gl.Enable(gl::CULL_FACE);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl.Enable(gl::DEPTH_TEST);
        }
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