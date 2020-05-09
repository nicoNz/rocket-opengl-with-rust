
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


pub struct App {
    mesh: Mesh,
    camera: Camera,
    vp: GLint,
    gl: gl::Gl
}

fn create_ring_pos(height: f32, rad: f32, circ_def: u8) -> Vec<f32> {
    let step = std::f32::consts::PI * 2. / circ_def as f32;
    let mut res: Vec<f32> = Vec::new();

    for i in 0..circ_def {
        let angle = step * i as f32;
        let x = rad * f32::cos(angle);
        let y = rad * f32::sin(angle);
        res.push(x);
        res.push(0.);
        res.push(y);
        res.push(x);
        res.push(height);
        res.push(y);
        println!("x : {} ; y  : {}", &x, &y);
    }
    res
}

fn create_cap_pos(height: f32, rad: f32, circ_def: u8, res: &mut Vec<f32>, top: bool) {
    let v: f32 = match top {
        true => 0.0,
        false => height
    };
    res.push(0.);
    res.push(v);
    res.push(0.);
    let step = std::f32::consts::PI * 2. / circ_def as f32;
    for i in 0..circ_def {
        let angle = step * i as f32;
        let x = rad * f32::cos(angle);
        let y = rad * f32::sin(angle);
        res.push(x);
        res.push(v);
        res.push(y);
    }
}

fn create_norms(circ_def: u8) -> Vec<f32> {
    let step = std::f32::consts::PI * 2. / circ_def as f32;
    let mut res: Vec<f32> = Vec::new();

    for i in 0..circ_def {
        let angle = step * i as f32;
        let x = f32::cos(angle);
        let y = f32::sin(angle);
        res.push(x);
        res.push(0.0);
        res.push(y);
        res.push(x);
        res.push(0.0);
        res.push(y);
    }
    res
}

fn create_cap_norms(circ_def: u8, res: &mut Vec<f32>, top: bool){

    let v: f32 = match top {
        true => -1.0,
        false => 1.0
    };
    for i in 0..circ_def+1 {
        res.push(0.0);
        res.push(v);
        res.push(0.0);
    }
}




fn create_indicies(circ_def: u8) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for i in 0..circ_def-1 {
        let p = i * 2;
        res.push(p);
        res.push(p+1);
        res.push(p+2);

        res.push(p+3);
        res.push(p+2);
        res.push(p+1);
    }


    res.push(circ_def*2-2);
    res.push(circ_def*2-1);
    res.push(0);

    res.push(1);
    res.push(0);
    res.push(circ_def*2-1);
    
    res
}

fn create_cap_indicies(circ_def: u8, res: &mut Vec<u8>, top: bool ) {
    let (from, to, center) = match top {
        true => (circ_def*2+1, circ_def*3, circ_def*2),
        false => (circ_def*3+2, circ_def*4+1, circ_def*3+1)
    };
    for p in from..to {
        res.push(p);
        res.push(center);
        res.push(p+1);
    }
    res.push(to);
    res.push(center);
    res.push(from);
}

fn gen_cylinder_data(height: f32, rad: f32, circ_def: u8) -> (Vec<u8>, Vec<f32>, Vec<f32>) {
    let mut pos = create_ring_pos(height, rad, circ_def);
    create_cap_pos(height, rad, circ_def, &mut pos, false);
    create_cap_pos(height, rad, circ_def, &mut pos, true);

    let mut norm = create_norms(circ_def);
    create_cap_norms(circ_def, &mut norm, false);
    create_cap_norms(circ_def, &mut norm, true);

    let mut indicies = create_indicies(circ_def);
    create_cap_indicies(circ_def, &mut indicies, true);
    create_cap_indicies(circ_def, &mut indicies, false);
    (indicies, pos, norm)
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
        

        let vbo_i = VboU8::from_vector(gl, &indicies, 1);
        mesh.set_indicies(vbo_i);


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

        let camera = &mut self.camera;
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