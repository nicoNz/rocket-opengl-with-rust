
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
use super::gen_checker::gen_checker_data;

pub struct App {
    mesh: Mesh,
    checker_mesh: Mesh,
    camera: Camera,
    vp: GLint,
    gl: gl::Gl,
    u_position: GLint,
    u_checker_position: GLint,
    red_team: Vec<Piece>

}

struct Piece {
    x: u8,
    y: u8
}
impl Piece {
    fn new(x: u8, y: u8) -> Self {
        Self {
            x,
            y
        }
    }
}



impl App {
    pub fn new(gl: &gl::Gl  )-> Self {

        let camera = Camera::from_position_and_look_at(&glm::vec3(-6.0,3.0, 5.0), &glm::vec3(0., 0., 0.));

        // --- Piece
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
        let u_position = *key_map.get("u_position").unwrap_or(&-1);
        if vp < 0 {
            panic!("u_position not found is shader cause panic");
        }

        shader.use_shader();
        shader.set_uniform_value(vp, UniformValue::Mat4(camera.get_view_projection())); 
        shader.set_uniform_value(u_position, UniformValue::Vec3(glm::vec3(2.0, 0.0, 2.0))); 
        let mut mesh = Mesh::new(gl, &Rc::new(RefCell::new(shader)));

        let (indicies, positions, norms) = gen_cylinder_data(0.1, 0.4, 16);

        let position = Box::new(VboF32::from_vector(&gl, &positions, 3));
        mesh.set_buffer_at_location(position, 0);

        let normal = Box::new(VboF32::from_vector(&gl, &norms, 3));
        mesh.set_buffer_at_location(normal, 1);
        
        let ibo = VboU8::from_vector(gl, &indicies, 1);
        mesh.set_indicies(ibo);

        let mut red_team: Vec<Piece> = Vec::new();

        for i in 0..(3*5) {
            let row = i/5;
            red_team.push(Piece::new((i%5)*2 + row%2, row))
        }



        // --- Checker
        let mut shader = match Shader::from_json(gl, &String::from("checker.json")) {
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
        let u_checker_position = *key_map.get("u_position").unwrap_or(&-1);
        if vp < 0 {
            panic!("u_position not found is shader cause panic");
        }

        shader.use_shader();
        shader.set_uniform_value(vp, UniformValue::Mat4(camera.get_view_projection())); 
        shader.set_uniform_value(u_checker_position, UniformValue::Vec3(glm::vec3(0.0, 0.0, 0.0))); 

        let mut checker_mesh = Mesh::new(gl, &Rc::new(RefCell::new(shader)));

        let (indicies, positions, norms, tex_coords) = gen_checker_data();
    
        let position = Box::new(VboF32::from_vector(&gl, &positions, 3));
        checker_mesh.set_buffer_at_location(position, 0);

        let normal = Box::new(VboF32::from_vector(&gl, &norms, 3));
        checker_mesh.set_buffer_at_location(normal, 1);

        let tex_coord = Box::new(VboF32::from_vector(&gl, &tex_coords, 2));
        checker_mesh.set_buffer_at_location(tex_coord, 2);
        
        let ibo = VboU8::from_vector(gl, &indicies, 1);
        checker_mesh.set_indicies(ibo);

        
        App {
            camera,
            mesh,
            checker_mesh,
            vp,
            gl: gl.clone(),
            red_team,
            u_position,
            u_checker_position
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
        let shader = &self.mesh.shader;
        for p in &self.red_team {
            shader.borrow_mut().set_uniform_value(self.u_position, UniformValue::Vec3(glm::vec3(p.x as f32 - 4.5, 0.0, p.y  as f32 - 4.5 )));
            self.mesh.draw();
        }
        self.checker_mesh.shader.borrow_mut().set_uniform_value(self.u_checker_position, UniformValue::Vec3(glm::vec3(0.0, 0.0, 0.0 )));
        self.checker_mesh.draw();
    }
    fn on_window_event(&mut self, event: &sdl2::event::Event) {
        
        match event {
            sdl2::event::Event::MouseMotion {x, .. } => {
            },
            _ => {}
        }
    }
}