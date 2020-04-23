


use crate::file::util::get_cstr_from_path;
use crate::render::uniform::Uniform;
use crate::render::uniform::UniformValue;
use gl::types::GLint;
use std::fmt::Display;
use gl;
use std;
use std::ffi::{CStr, CString};
use nalgebra_glm;
use crate::file::shader_description_parser::ShaderDescription;
use crate::file::shader_description_parser::UniformDescription;
use crate::render::program::Program;
use crate::render::raw_shader::RawShader;



pub struct Shader {
    pub program: Program,
    vertex_shader: RawShader,
    fragment_shader: RawShader,
    uniforms: std::collections::HashMap<GLint, Uniform>,
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
                Err(String::from(std::fmt::format(format_args!( "fail to create shader, {}", e))))
            }
        }

    }
    pub fn get_uniform_to_key_map(&self) -> Res {
        let map: Res = Res::new();
        for (key, value) in self.uniforms.iter() {
            map.insert(value.name.clone(), *key);
        }
        map
    }
    pub fn use_shader(&mut self) {
        self.program.set_used();
    }

    pub fn set_uniform_value(&self, i: i32, v: UniformValue) {
        if let Some(uniform) = self.uniforms.get(&i) {
            uniform.load_into_program()
        }
    }
    pub fn from_shader_description(gl: &gl::Gl, shader_description: &ShaderDescription) -> Result<Self, String> {
        match (
            shader_description.vertex_shader_file,
            shader_description.fragment_shader_file 
        ) {
            (
                Some(fragment_shader_file), 
                Some(vertex_shader_file)
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
                                let program = Program::from_shaders(gl, &[vertex_shader, fragment_shader]);
                                match program {
                                    Ok(program) => {
                                        Ok(
                                            Self {
                                                fragment_shader,
                                                vertex_shader,
                                                program,
                                                uniforms = shader_description.uniforms,
                                                shader_description : shader_description.clone()
            
                                            }
                                        )
                                    },
                                    Err(e) => Err(e)
                                }
                            }
                        }
                        
                    },
                    _ => Err(String::from("at least on shader source asn't found from file path"))
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











