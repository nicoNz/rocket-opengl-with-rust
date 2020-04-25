


//use std::fmt::Display;
//use std::ffi::{CStr, CString};
//use nalgebra_glm;
use crate::file::util::get_cstr_from_path;
use crate::render::uniform::Uniform;
use crate::render::uniform::UniformValue;
use gl::types::GLint;
use gl;
use std;
use crate::file::shader_description_parser::ShaderDescription;
use crate::file::shader_description_parser::UniformDescription;
use crate::render::program::Program;
use crate::render::raw_shader::RawShader;
use std::collections::HashMap;
//use std::iter::
pub struct Uniforms {
    collection: HashMap<GLint, Uniform>,
    key_gen: i32
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            collection : HashMap::new(),
            key_gen : 0
        }
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<i32, Uniform> {
        self.collection.iter()
    }
    pub fn push(&mut self, uniform: Uniform) -> i32 {
        self.key_gen += 1;
        self.collection.insert(self.key_gen,  uniform);
        self.key_gen
    }

    pub fn get_mut(&mut self, k: i32) -> Option<&mut Uniform> {
        self.collection.get_mut(&k)
    }

    pub fn get(&mut self, k: i32) -> Option<&Uniform> {
        self.collection.get(&k)
    }
    pub fn len(&self) -> usize{
        self.collection.len()
    }
}

pub struct Shader {
    pub program: Program,
    vertex_shader: RawShader,
    fragment_shader: RawShader,
    uniforms: Uniforms,
    shader_description: ShaderDescription
}


/**
 *     pub fn set_uniform(&mut self, loc: GLint, value: UniformValue) {
        
        if let Some(v) = self.uniforms.get_mut(&loc) {
            v.value = value;
        } else {
            println!("key {} did not exist", loc);
        };
    }
 */
type Res = std::collections::HashMap<String, i32>;
impl Shader {
    
    pub fn from_json(gl: &gl::Gl, path: &String) -> Result<Self, String> {
        match ShaderDescription::from_file(path) {
            Ok(ref shader_description) => {
                Self::from_shader_description(gl, shader_description)
            },
            Err(e) => {
                Err(String::from(std::fmt::format(format_args!( "fail to create shader; Error => {}", e))))
            }
        }
    }

    pub fn get_uniform_to_key_map(&self) -> Res {
        let mut  map: Res = Res::new();
        println!("get_uniform_to_key_map, nElts => {}", self.uniforms.len());
        for (key, value) in self.uniforms.iter() {
            map.insert(value.name.clone(), *key);
        }
        map
    }
    
    pub fn use_shader(&self) {
        self.program.use_program();
    }

    pub fn set_uniform_value(&mut self, i: i32, v: UniformValue) {
        if let Some(ref mut uniform) = self.uniforms.get_mut(i) {
            uniform.set_value(v);
            //uniform.load_into_program()
        } else {
            println!("fail to load unif at loc {}", i);
        }
    }

    pub fn register_uniform(&mut self, uniform_description: &UniformDescription) -> GLint {
        let name = uniform_description.get_name();
        let loc = self.program.get_uniform_location(name);
        match loc {
            Ok(loc)=> {
                println!("name {} found at loc {}", name, loc);
                match Uniform::from_description_and_program(uniform_description, &self.program) {
                    Ok(uniform)=>{
                        if self.uniforms.push( uniform ) < 0 {
                            println!("Error while trying to get location of {}, key {} already exist", name, loc)
                        }
                    },
                    Err(_) => return -1
                }
                loc
            },
            Err(())=>panic!("panic because the behavious when a location is not found is not implemented")
        }
    }


    pub fn from_shader_description(gl: &gl::Gl, shader_description: &ShaderDescription) -> Result<Self, String> {
        match (
            shader_description.vertex_shader_file.as_ref(),
            shader_description.fragment_shader_file.as_ref() 
        ) {
            (
                Some(vertex_shader_file),
                Some(fragment_shader_file) 
            ) => {
                match (
                    get_cstr_from_path(&vertex_shader_file),
                    get_cstr_from_path(&fragment_shader_file)
                ) {
                    (
                        Ok(vertex_source),
                        Ok(fragment_source)
                    ) => {
                        match (
                            RawShader::from_vert_source(gl, vertex_source.as_c_str()),
                            RawShader::from_frag_source(gl, fragment_source.as_c_str())
                        ) {
                            (
                                Ok(vertex_shader),
                                Ok(fragment_shader)
                            ) => {
                                let program = Program::from_shaders(gl, &[&vertex_shader, &fragment_shader]);
                                match program {
                                    Ok(program) => {
                                        Ok(
                                            Self {
                                                fragment_shader,
                                                vertex_shader,
                                                uniforms : shader_description.uniforms.instanciate_all(&program),
                                                program,
                                                shader_description : shader_description.clone()
                                                
            
                                            }
                                        )
                                    },
                                    Err(e) => Err(e)
                                }
                            },
                            _ => Err(format!("\nat least one shader path couldn't be found \n VERTS : \n{:?} \n FRAGS : \n{:?}", vertex_source, fragment_source))   
                        }  
                    },
                    _ => {
                        
                        Err(format!("at least one shader path couldn't be found : checked the presence of {}, {}", vertex_shader_file, fragment_shader_file))
                    }
                }
            },
            _ => {
                Err(String::from("raw shaders were missing from description"))
            }
        }
    }
}




pub struct Material {

}











